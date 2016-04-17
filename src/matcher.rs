use aho_corasick::{AcAutomaton, Automaton, Match};

use hint::Hint;
use pattern_groups::PatternGroups;

pub fn matcher(automaton: &AcAutomaton<&'static str>, groups: &PatternGroups, text: &str) -> Vec<Hint> {
    automaton
        .find(&text.to_lowercase())
        .map(|m: Match| Hint {
            line: None,
            start: m.start,
            end: m.end,
            group: groups.group_names.get(m.pati).map(|&x| x),
            value: groups.patterns[m.pati],
        })
        .collect()
}

#[test]
fn matcher_works() {
    use data;

    let text = "To boldly go where no-one has gone before";

    let mut pattern_groups = PatternGroups::new();
    pattern_groups.push(data::ADVERBS, "adverbs");

    assert_eq!(
        matcher(&AcAutomaton::new(pattern_groups.patterns.clone()), &pattern_groups, text),
        vec![Hint { start: 3, end: 9, line: None, group: Some("adverbs"), value: "boldly"}]
    );

    let mut pattern_groups = PatternGroups::new();
    pattern_groups.push(data::WEAKENS, "weakens");

    assert_eq!(
        matcher(&AcAutomaton::new(pattern_groups.patterns.clone()), &pattern_groups, text),
        vec![]
    );
}

#[test]
fn matcher_works_with_groups() {
    use data;

    let text = "Wisely written specs put me at loose ends.";

    let mut pattern_groups = PatternGroups::with_capacity(4_000);
    pattern_groups.push(data::ADVERBS, "adverbs");
    pattern_groups.push(data::WEAKENS, "weakens");
    pattern_groups.push(data::CLICHES, "cliches");

    assert_eq!(
        matcher(&AcAutomaton::new(pattern_groups.patterns.clone()), &pattern_groups, text),
        vec![
            Hint { group: Some("adverbs"), value: "wisely", line: None, start: 0, end: 6 },
            Hint { group: Some("cliches"), value: "at loose ends", line: None, start: 28, end: 41 }
        ]
    );
}
