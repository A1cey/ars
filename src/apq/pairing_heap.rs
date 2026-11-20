use std::collections::{HashMap, HashSet};

use alloc::vec::Vec;

use crate::apq::AddressablePriorityQueue;

#[derive(Debug, Default)]
pub struct PairingHeap<Item> {
    forest: HashSet<Handle>,
    min_ptr: Option<Handle>,
    nodes: Vec<Node<Item>>,
    handles: HashMap<u64, usize>,
    next_id: u64,
}

#[derive(Debug)]
pub struct Node<Item> {
    parent: Option<Handle>,
    item: Item,
    children: HashSet<Handle>,
    id: u64,
    version: u32,
}

impl<Item> Node<Item> {
    pub(super) const fn handle(&self) -> Handle {
        Handle {
            id: self.id,
            version: self.version,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle {
    id: u64,
    version: u32,
}

impl Handle {
    pub(super) fn inc_version(self) -> Self {
        Self {
            id: self.id,
            version: self.version + 1,
        }
    }
}

impl<Item: PartialEq + PartialOrd> PairingHeap<Item> {
    const fn create_handle(&mut self) -> Handle {
        let handle = Handle {
            id: self.next_id,
            version: 0,
        };
        self.next_id += 1;
        handle
    }

    fn node(&self, handle: Handle) -> &Node<Item> {
        let idx = *self.handles.get(&handle.id).expect("Valid handle");
        let node = self.nodes.get(idx).expect("Valid handle");
        assert_eq!(node.version, handle.version, "Invalidated handle");
        node
    }

    fn node_mut(&mut self, handle: Handle) -> &mut Node<Item> {
        let idx = *self.handles.get(&handle.id).expect("Valid handle");
        let node = self.nodes.get_mut(idx).expect("Valid handle");
        assert_eq!(node.version, handle.version, "Invalidated handle");
        node
    }

    fn is_root(&self, handle: Handle) -> bool {
        self.node(handle).parent.is_none()
    }

    fn update_min_ptr(&mut self) {
        match self.min_ptr {
            Some(min) => {
                let curr = self.node(handle);
                let min = self.node(min);

                if curr.item < min.item {
                    self.min_ptr = Some(handle)
                }
            }
            None => self.min_ptr = Some(handle),
        };
    }

    fn new_tree(&mut self, handle: Handle) {
        self.forest.insert(handle);

        match self.min_ptr {
            Some(min) => {
                let curr = self.node(handle);
                let min = self.node(min);

                if curr.item < min.item {
                    self.min_ptr = Some(handle)
                }
            }
            None => self.min_ptr = Some(handle),
        };
    }

    fn cut(&mut self, handle: Handle) {
        if let Some(parent_handle) = self.node(handle).parent {
            let parent = self.node_mut(parent_handle);
            parent.children.remove(&handle);
        }

        self.node_mut(handle).parent = None;

        self.new_tree(handle);
    }

    fn combine(&mut self, root_a: Handle, root_b: Handle) {
        if root_a == root_b || !self.is_root(root_a) || !self.is_root(root_b) {
            return;
        }

        let item_a = &self.node(root_a).item;
        let item_b = &self.node(root_b).item;

        let min = self.min_ptr.expect("If items exist, a min exists.");
        let min_item = &self.node(min).item;

        // min check to prevent equal sized items to move min into child
        if item_b == min_item {
            self.link(root_b, root_a);
        } else if item_a == min_item || item_a <= item_b {
            self.link(root_a, root_b)
        } else {
            self.link(root_b, root_a)
        }
    }

    fn link(&mut self, new_parent: Handle, removed_root: Handle) {
        self.forest.remove(&removed_root);
        self.node_mut(new_parent).children.insert(removed_root);
    }

    fn union(&mut self) {
        if self.forest.is_empty() {
            return;
        }

        // first: combine pairs
        let roots = self.forest.iter().copied().collect::<Vec<_>>();
        let (pairs, _rem) = roots.as_chunks::<2>();

        for &[left, right] in pairs {
            self.combine(left, right);
        }

        // second: merge all roots -> min elem is only root after this, combine never moves min into
        // child
        let min_handle = self
            .min_ptr
            .expect("forest is not empty, thus a min element exists");

        self.forest.remove(&min_handle);

        let roots = self.forest.drain().collect::<Vec<_>>();

        for root in roots {
            self.link(min_handle, root);
        }

        self.forest.insert(min_handle);
    }
}

impl<Item: PartialEq + PartialOrd> AddressablePriorityQueue for PairingHeap<Item> {
    type Handle = Handle;

    type Item = Item;

    fn new() -> Self {
        Self {
            forest: HashSet::new(),
            min_ptr: None,
            handles: HashMap::new(),
            nodes: Vec::new(),
            next_id: 0,
        }
    }

    fn insert(&mut self, item: Self::Item) -> Self::Handle {
        let handle = self.create_handle();
        self.handles.insert(handle.id, self.nodes.len());
        self.nodes.push(Node {
            parent: None,
            item,
            id: handle.id,
            version: handle.version,
            children: HashSet::new(),
        });
        self.new_tree(handle);
        handle
    }

    fn delete_min(&mut self) -> Option<Self::Item> {
        let min_handle = self.min_ptr.take()?;

        self.forest.remove(&min_handle);

        let min_idx = self
            .handles
            .remove(&min_handle.id)
            .expect("Valid min handle");

        let min_node = self.nodes.remove(min_idx);

        for child in min_node.children {
            self.cut(child);
        }

        self.union();

        Some(min_node.item)
    }

    fn min(&self) -> Option<&Self::Item> {
        self.min_ptr.map(|handle| &self.node(handle).item)
    }

    fn remove(&mut self, handle: Self::Handle) -> Option<Self::Item> {
        // guard for external handle
        if !self.handles.contains_key(&handle.id) {
            return None;
        }

        if self.min_ptr == Some(handle) {
            self.delete_min()
        } else {
            self.cut(handle);
            self.forest.remove(&handle);
            let node_idx = self.handles.remove(&handle.id)?;
            let node = self.nodes.remove(node_idx);
            let children = node.children.iter().copied().collect::<Vec<_>>();

            for child in children {
                self.cut(child);
            }

            Some(node.item)
        }
    }

    fn decrease_key(&mut self, handle: Self::Handle, new_key: Self::Item) {
        if !self.handles.contains_key(&handle) {
            panic!("Invalid handle");
        }

        let node = self.node(handle);
        if node.item <= new_key {
            return;
        }

        self.cut(handle);
    }

    fn merge(&mut self, other: impl AddressablePriorityQueue<Item = Self::Item>) {
        for item in other.into_sorted_iter() {
            self.insert(item);
        }
    }
}
