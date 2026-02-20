//! This module provides a compact, copyable `Range` tuple struct designed to be
//! convenient for storing, hashing, ordering and using directly as a slice index.
//!
//!
//! # Example
//! ```
//! // Basic indexing (start inclusive, end exclusive):
//! let a = [0, 1, 2, 3, 4];
//! let r = ars::range::Range::new(1, 4);
//! assert_eq!(&a[r], &[1, 2, 3]);
//! ```
use core::ops::Index;

/// A compact, copyable index range holding a `start` (inclusive) and `end` (exclusive).
///
/// This is a lightweight alternative to [`core::ops::Range<usize>`] where you may
/// want the range to implement traits like `Copy`, `Hash`, `Ord`, and `Debug` in
/// a trivial tuple form. It also provides a few convenience methods on top of the
/// raw tuple so common operations are ergonomic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Range(pub usize, pub usize);

impl Range {
    /// Construct a new `Range` from `start` and `end` (start inclusive, end exclusive).
    ///
    /// The values are not validated — callers should ensure `start <= end` if that
    /// invariant matters for their use case. Many methods (like `len`) handle
    /// inverted ranges sensibly (returning 0).
    #[must_use]
    #[inline]
    pub const fn new(start: usize, end: usize) -> Self {
        Self(start, end)
    }

    /// Returns the start (inclusive) of the range.
    #[must_use]
    #[inline]
    pub const fn start(&self) -> usize {
        self.0
    }

    /// Returns the end (exclusive) of the range.
    #[must_use]
    #[inline]
    pub const fn end(&self) -> usize {
        self.1
    }

    /// Returns the length of the range, saturating at 0 if `end < start`.
    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.1.saturating_sub(self.0)
    }

    /// Returns `true` if the range contains no elements (i.e. `start >= end`).
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 >= self.1
    }

    /// Returns `true` if the provided index is inside the range.
    #[must_use]
    #[inline]
    pub const fn contains(&self, index: usize) -> bool {
        index >= self.0 && index < self.1
    }

    /// Returns a new `Range` clamped to the provided `len`.
    ///
    /// This is useful when you want to safely apply a range to a slice without
    /// panicking from out-of-bounds indexing.
    ///
    /// # Example
    /// ```
    /// let a = &[1, 2, 3, 4];
    /// let r = ars::range::Range(2, 10);
    /// let clamped = r.clamp_to(a.len());
    /// assert_eq!(&a[clamped], &[3, 4]);
    /// ```
    #[must_use]
    pub fn clamp_to(&self, len: usize) -> Self {
        let s = core::cmp::min(self.0, len);
        let e = core::cmp::min(self.1, len);
        Self(s, e)
    }

    /// Returns the intersection of `self` and `other`, or `None` if they don't overlap.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let s = core::cmp::max(self.0, other.0);
        let e = core::cmp::min(self.1, other.1);
        if s < e { Some(Self(s, e)) } else { None }
    }

    /// Shift the range by `delta` (adds to both start and end).
    ///
    /// Note: this does not check overflow — callers should ensure shifting is safe.
    #[must_use]
    #[inline]
    pub const fn offset(&self, delta: usize) -> Self {
        Self(self.0 + delta, self.1 + delta)
    }

    /// Attempt to shrink the range from the start and/or end by provided amounts.
    ///
    /// This will saturate at empty (i.e. not underflow).
    #[must_use]
    pub const fn shrink(&self, start_shrink: usize, end_shrink: usize) -> Self {
        // Use saturating arithmetic to avoid panic.
        let s = self.0.saturating_add(start_shrink);
        let e = self.1.saturating_sub(end_shrink);
        // If we've inverted the range, normalize to empty at the original start.
        if s >= e { Self(s, s) } else { Self(s, e) }
    }
}

impl<T> Index<Range> for [T] {
    type Output = [T];

    fn index(&self, index: Range) -> &Self::Output {
        &self[index.0..index.1]
    }
}

impl<T> Index<&Range> for [T] {
    type Output = [T];

    fn index(&self, index: &Range) -> &Self::Output {
        &self[index.0..index.1]
    }
}

impl From<core::ops::Range<usize>> for Range {
    fn from(r: core::ops::Range<usize>) -> Self {
        Self(r.start, r.end)
    }
}

impl From<Range> for core::ops::Range<usize> {
    fn from(r: Range) -> Self {
        r.0..r.1
    }
}

impl From<(usize, usize)> for Range {
    fn from(t: (usize, usize)) -> Self {
        Self(t.0, t.1)
    }
}

impl From<Range> for (usize, usize) {
    fn from(r: Range) -> (usize, usize) {
        (r.0, r.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn index_with_range_by_value() {
        let s: &[i32] = &[10, 20, 30, 40, 50];
        let r = Range::new(1, 4);
        assert_eq!(s[r].to_vec(), vec![20, 30, 40]);
    }

    #[test]
    fn index_with_range_by_ref() {
        let s: &[i32] = &[1, 2, 3, 4];
        let r = Range::new(0, 2);
        assert_eq!(s[&r].to_vec(), vec![1, 2]);
    }

    #[test]
    #[should_panic]
    fn indexing_out_of_bounds_panics() {
        let s: &[i32] = &[1, 2, 3];
        // End > len should panic when used directly as an index.
        let _ = &s[Range::new(2, 10)];
    }

    #[test]
    fn traits_and_hashing() {
        let a = Range::new(2, 5);
        let b = a; // Copy
        let c = a.clone(); // Clone

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert!(a >= Range::new(1, 4));
        assert!(Range::new(0, 1) < Range::new(1, 2));

        // Hash equality for equal values
        let mut h1 = DefaultHasher::new();
        a.hash(&mut h1);
        let mut h2 = DefaultHasher::new();
        c.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());

        // Debug formatting contains the tuple representation
        let debug = format!("{:?}", a);
        assert!(debug.contains("2"));
        assert!(debug.contains("5"));
    }

    #[test]
    fn helpers_len_empty_contains() {
        assert_eq!(Range::new(2, 5).len(), 3);
        assert!(Range::new(2, 2).is_empty());
        assert!(Range::new(3, 4).contains(3));
        assert!(!Range::new(3, 4).contains(4));
        // inverted range yields zero length
        assert_eq!(Range::new(5, 3).len(), 0);
        assert!(Range::new(5, 3).is_empty());
    }

    #[test]
    fn clamp_and_intersect() {
        let r = Range::new(2, 10);
        assert_eq!(r.clamp_to(5), Range::new(2, 5));
        assert_eq!(r.clamp_to(1), Range::new(1, 1));

        let a = Range::new(0, 5);
        let b = Range::new(3, 8);
        assert_eq!(a.intersect(&b), Some(Range::new(3, 5)));
        assert_eq!(Range::new(0, 2).intersect(&Range::new(2, 4)), None);
    }

    #[test]
    fn offset_and_shrink() {
        let r = Range::new(2, 7);
        assert_eq!(r.offset(3), Range::new(5, 10));
        assert_eq!(r.shrink(1, 2), Range::new(3, 5));
        // shrinking more than length yields empty at the s position
        assert_eq!(r.shrink(10, 0).is_empty(), true);
    }

    #[test]
    fn conversions_roundtrip() {
        let core: core::ops::Range<usize> = 1..4;
        let r: Range = core.into();
        assert_eq!(r, Range::new(1, 4));

        let back: core::ops::Range<usize> = r.into();
        assert_eq!(back, 1..4);

        let t = (2usize, 6usize);
        let rr: Range = t.into();
        assert_eq!(rr, Range::new(2, 6));
        let tup: (usize, usize) = rr.into();
        assert_eq!(tup, (2, 6));
    }
}
