mod binary;

pub use binary::BinaryHeap;

pub trait AddressablePriorityQueue<T> {
    fn insert(item: T);

    fn delete_min();

    fn min();

    fn decrease_key();
}
