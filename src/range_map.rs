//! Trivial mapping of ranges to things.
//!
//! This is only an implementation detail is is utterly unqualified for production.

use std::ops::Range;

/// Map: (start...end) -> Value
#[derive(Debug, Clone)]
pub struct RangeMap<T> {
    data: Vec<(Range<usize>, T)>,
}

impl<T> RangeMap<T> {
    pub fn new() -> Self {
        RangeMap { data: vec![] }
    }

    // TODO: Check for overlapping ranges
    pub fn push(&mut self, range: Range<usize>, entry: T) {
        self.data.push((range, entry));
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data
            .iter()
            .filter(|&&(ref range, _)| (idx >= range.start) && (idx < range.end))
            .map(|&(_, ref data)| data)
            .next()
    }
}

#[test]
fn range_map_contains() {
    let mut map = RangeMap::new();

    map.push(0..10, "foo");
    map.push(20..30, "bar");

    assert_eq!(map.get(5), Some(&"foo"));
    assert_eq!(map.get(15), None);
    assert_eq!(map.get(25), Some(&"bar"));
}
