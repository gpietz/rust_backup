use std::ops::{Bound, RangeBounds};

pub trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut byte_end = start + len;
        if start >= self.len() {
            return "";
        }
        if byte_end >= self.len() {
            byte_end = self.len();
        }
        &self[start..byte_end]
    }

    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

#[test]
fn test_substring() {
    let s = "Hello, World!";
    assert_eq!(s.substring(0, 5), "Hello");
    assert_eq!(s.substring(7, 5), "World");
    assert_eq!(s.substring(7, 100), "World!");
    assert_eq!(s.substring(0, 0), "");
    assert_eq!(s.substring(100, 0), "");
}

#[test]
fn test_slice() {
    let s = "Hello, World!";
    let slice = s.slice(..5);
    assert_eq!(slice, "Hello");
    let slice = s.slice(7..);
    assert_eq!(slice, "World!");
    let slice = s.slice(7..12);
    assert_eq!(slice, "World");
    let slice = s.slice(..);
    assert_eq!(slice, "Hello, World!");
}
