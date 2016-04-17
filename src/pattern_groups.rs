use range_map::RangeMap;

pub struct PatternGroups {
    pub patterns: Vec<&'static str>,
    pub group_names: RangeMap<&'static str>,
}

impl PatternGroups {
    #[allow(dead_code)]
    pub fn new() -> PatternGroups {
        PatternGroups {
            patterns: Vec::new(),
            group_names: RangeMap::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> PatternGroups {
        PatternGroups {
            patterns: Vec::with_capacity(cap),
            group_names: RangeMap::new(),
        }
    }

    pub fn push(&mut self, slice: &[&'static str], name: &'static str) {
        if slice.is_empty() { return; }

        let current_offset = self.patterns.len();
        self.patterns.extend(slice);
        self.group_names.push(current_offset..current_offset + slice.len(), name);
    }
}

#[test]
fn offsets_and_stuff() {
    let mut pgs = PatternGroups::new();
    pgs.push(&["foo", "bar"], "one");
    pgs.push(&["lorem", "ipsum"], "two");

    assert_eq!(pgs.group_names.get(0), Some(&"one"));
    assert_eq!(pgs.group_names.get(1), Some(&"one"));
    assert_eq!(pgs.group_names.get(2), Some(&"two"));
    assert_eq!(pgs.group_names.get(3), Some(&"two"));
}
