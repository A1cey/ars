use std::collections::HashMap;

use alloc::vec::Vec;

use crate::{apq::AddressablePriorityQueue, rng::PCGRand};

type Children<'item, Item> = (Option<(Handle, &'item Item)>, Option<(Handle, &'item Item)>);

#[derive(Debug, Default)]
pub struct BinaryHeap<Item> {
    items: Vec<(Handle, Item)>,
    handles: HashMap<Handle, usize>,
    rng: PCGRand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Handle(u64);

impl<Item: PartialEq + PartialOrd> BinaryHeap<Item> {
    const fn create_handle(&mut self) -> Handle {
        Handle(self.rng.random())
    }

    fn item(&self, handle: Handle) -> Option<&Item> {
        let idx = *self.handles.get(&handle)?;
        self.items.get(idx).map(|(_, item)| item)
    }

    fn swap(&mut self, first_handle: Handle, second_handle: Handle) {
        let first_idx = *self
            .handles
            .get(&first_handle)
            .expect("Valid internal handle");
        let second_idx = *self
            .handles
            .get(&second_handle)
            .expect("Valid internal handle");

        self.items.swap(first_idx, second_idx);

        self.handles
            .entry(first_handle)
            .and_modify(|e| *e = second_idx);
        self.handles
            .entry(second_handle)
            .and_modify(|e| *e = first_idx);
    }

    fn bubble_up(&mut self, handle: Handle) {
        while let Some((parent_handle, parent)) = self.parent(handle)
            && let Some(curr) = self.item(handle)
            && curr < parent
        {
            self.swap(handle, parent_handle);
        }
    }

    fn bubble_down(&mut self, handle: Handle) {
        while let Some(curr) = self.item(handle) {
            match self.children(handle) {
                (Some((left_handle, left)), Some((right_handle, right))) => {
                    if curr > left {
                        self.swap(handle, left_handle);
                    } else if curr > right {
                        self.swap(handle, right_handle);
                    } else {
                        break;
                    }
                }
                (Some((left_handle, left)), None) => {
                    if curr > left {
                        self.swap(handle, left_handle);
                    } else {
                        break;
                    }
                }
                (None, Some((right_handle, right))) => {
                    if curr > right {
                        self.swap(handle, right_handle);
                    } else {
                        break;
                    }
                }
                (None, None) => break,
            }
        }
    }

    fn parent(&self, handle: Handle) -> Option<(Handle, &Item)> {
        let idx = *self.handles.get(&handle)?;

        if idx == 0 {
            return None;
        }
        let parent_idx = idx / 2;
        self.items
            .get(parent_idx)
            .map(|(parent_handle, parent_item)| (*parent_handle, parent_item))
    }

    fn children(&self, handle: Handle) -> Children<'_, Item> {
        let idx = self.handles.get(&handle).expect("Valid internal handle");

        let left = idx * 2 + 1;
        let right = idx * 2 + 2;
        (
            self.items.get(left).map(|(handle, item)| (*handle, item)),
            self.items.get(right).map(|(handle, item)| (*handle, item)),
        )
    }
}

impl<Item: PartialEq + PartialOrd> AddressablePriorityQueue for BinaryHeap<Item> {
    type Handle = Handle;
    type Item = Item;

    #[must_use]
    fn new() -> Self {
        Self {
            items: Vec::new(),
            handles: HashMap::new(),
            rng: PCGRand::default(),
        }
    }

    fn insert(&mut self, item: Item) -> Self::Handle {
        let handle = self.create_handle();
        self.items.push((handle, item));
        self.handles.insert(handle, self.items.len() - 1);
        self.bubble_up(handle);
        handle
    }

    /// Invalidates the handle to the last item
    fn delete_min(&mut self) -> Option<Item> {
        let _ = self.items.first()?; // guard

        let (last_handle, _) = *self.items.last()?; // get the handle before swap
        let (min_handle, item) = self.items.swap_remove(0);
        self.handles.remove(&min_handle);
        self.handles.entry(last_handle).and_modify(|e| *e = 0); // correct idx
        self.bubble_down(last_handle);

        Some(item)
    }

    fn min(&self) -> Option<&Item> {
        self.items.first().map(|(_, item)| item)
    }

    fn remove(&mut self, handle: Handle) -> Option<Item> {
        let (last_handle, _) = *self.items.last()?; // get the handle before swap
        let idx = *self.handles.get(&handle)?;
        let (_, item) = self.items.swap_remove(idx);
        self.handles.remove(&handle);
        self.handles.entry(last_handle).and_modify(|e| *e = idx); // correct idx

        if let Some((_, last_item)) = self.items.get(idx)
            && let Some((_, parent)) = self.parent(last_handle)
            && parent > last_item
        {
            self.bubble_up(last_handle);
        } else {
            self.bubble_down(last_handle);
        }

        Some(item)
    }

    fn decrease_key(&mut self, handle: Handle, new_key: Item) {
        if let Some(idx) = self.handles.get(&handle)
            && let Some((_, item)) = self.items.get_mut(*idx)
            && new_key < *item
            && let Some(idx) = self.handles.get(&handle)
        {
            self.items[*idx].1 = new_key;
            self.bubble_up(handle);
        }
    }

    fn merge(&mut self, other: impl AddressablePriorityQueue<Item = Item>) {
        for item in other.into_sorted_iter() {
            self.insert(item);
        }
    }
}
