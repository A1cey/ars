mod binary;

pub use binary::BinaryHeap;

pub trait AddressablePriorityQueue {
    pub type Handle;
    pub type Item;

    fn build(items: &[Item]) -> Self;

    fn insert(self, item: Item) -> Handle;

    fn delete_min(self) -> Item;

    fn min(self) -> Item;

    fn remove(self, handle: Handle) -> Item;

    fn decrease_key(self, handle: Handle, new_key: Item);

    fn merge(self, other: impl AddressablePriorityQueue);
}
