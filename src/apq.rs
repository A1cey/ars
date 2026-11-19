mod binary_heap;
mod pairing_heap;

pub use binary_heap::BinaryHeap;
pub use pairing_heap::PairingHeap;

pub trait AddressablePriorityQueue: Sized {
    type Handle;
    type Item: PartialEq + PartialOrd;

    #[must_use]
    fn new() -> Self;

    #[must_use]
    fn build(items: impl IntoIterator<Item = Self::Item>) -> Self {
        let mut apq = Self::new();

        for item in items {
            apq.insert(item);
        }

        apq
    }

    fn insert(&mut self, item: Self::Item) -> Self::Handle;

    fn delete_min(&mut self) -> Option<Self::Item>;

    fn min(&self) -> Option<&Self::Item>;

    fn remove(&mut self, handle: Self::Handle) -> Option<Self::Item>;

    fn decrease_key(&mut self, handle: Self::Handle, new_key: Self::Item);

    fn merge(&mut self, other: impl AddressablePriorityQueue<Item = Self::Item>);

    fn into_sorted_iter(self) -> AddressablePriorityQueueIter<Self> {
        AddressablePriorityQueueIter { apq: self }
    }
}

pub struct AddressablePriorityQueueIter<Q> {
    apq: Q,
}

impl<Q> Iterator for AddressablePriorityQueueIter<Q>
where
    Q: AddressablePriorityQueue,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.apq.delete_min()
    }
}
