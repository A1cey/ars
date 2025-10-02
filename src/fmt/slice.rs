//! Slice formatting utilities.

use core::{
    fmt::{Display, Formatter},
    ops::Deref,
};

/// A helper struct for formatting slices.
///
/// # Example
/// ```
/// # use ars::fmt::slice::FmtSlice;
/// let array = [1, 2, 3];
/// let formatted = FmtSlice(&array);
/// assert_eq!(formatted.to_string(), String::from("[1, 2, 3]"));
/// assert_eq!(format!("{}", formatted), "[1, 2, 3]");
///
/// let vec = vec![4, 5, 6];
/// let formatted = FmtSlice(&vec);
/// assert_eq!(formatted.to_string(), String::from("[4, 5, 6]"));
/// assert_eq!(format!("{}", formatted), "[4, 5, 6]");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FmtSlice<'a, T>(pub &'a [T]);

impl<T> Deref for FmtSlice<'_, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Display> Display for FmtSlice<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "[")?;

        let mut iter = self.iter();
        if let Some(val) = iter.next() {
            write!(f, "{val}")?;

            for val in iter {
                write!(f, ", {val}")?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::format;
    use std::string::ToString;

    #[test]
    fn test_fmt_array_with_to_string() {
        let array = [1, 2, 3];
        let formatted = FmtSlice(&array);
        assert_eq!(formatted.to_string(), String::from("[1, 2, 3]"));
    }

    #[test]
    fn test_fmt_array_with_format_macro() {
        let array = [1, 2, 3];
        let formatted = FmtSlice(&array);
        assert_eq!(format!("{}", formatted), "[1, 2, 3]");
    }

    #[test]
    fn test_fmt_vec_with_to_string() {
        let vec = vec![1, 2, 3];
        let formatted = FmtSlice(&vec);
        assert_eq!(formatted.to_string(), String::from("[1, 2, 3]"));
    }

    #[test]
    fn test_fmt_vec_with_format_macro() {
        let vec = vec![1, 2, 3];
        let formatted = FmtSlice(&vec);
        assert_eq!(format!("{}", formatted), "[1, 2, 3]");
    }
}
