/// Checks that two byte slices are an ASCII case-insensitive match.
#[inline]
#[must_use]
pub fn eq_ignore_ascii_case(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.eq_ignore_ascii_case(y))
}

#[cfg(test)]
mod tests {
    use crate::ascii::eq_ignore_ascii_case;

    #[test]
    fn equal_slices() {
        assert!(eq_ignore_ascii_case(b"Hello", b"HELLO"))
    }
    
    #[test]
    fn different_slices() {
        assert!(!eq_ignore_ascii_case(b"Hello", b"Bye"))
    }
}
