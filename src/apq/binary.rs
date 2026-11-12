use crate::apq::AddressablePriorityQueue;

pub struct BinaryHeap;
pub struct BinaryHeapHandle;

impl AddressablePriorityQueue for BinaryHeap {
    type Handle = BinaryHeapHandle;

    type Item;

    fn build(items: &[Item]) -> Self {
        let apq = Self;

        for item in items {
            apq.insert(item);
        }

        apq
    }

    fn insert(self, item: Item) -> Handle {
        todo!()
    }

    fn delete_min(self) -> Item {
        todo!()
    }

    fn min(self) -> Item {
        todo!()
    }

    fn remove(self, handle: Handle) -> Item {
        todo!()
    }

    fn decrease_key(self, handle: Handle, new_key: Item) {
        todo!()
    }

    fn merge(self, other: impl AddressablePriorityQueue) {
        todo!()
    }
}
