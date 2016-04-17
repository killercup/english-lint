extern crate aho_corasick;

mod range_map;

mod data;
mod hint;
mod matcher;
mod pattern_groups;

use aho_corasick::AcAutomaton;

pub use hint::Hint;

pub fn lint(input: &str) -> Vec<Hint> {
    let mut pattern_groups = pattern_groups::PatternGroups::with_capacity(32_000);
    pattern_groups.push(data::ADVERBS, "adverbs");
    pattern_groups.push(data::WEAKENS, "weakens");
    pattern_groups.push(data::CLICHES, "cliches");
    
    let automaton = AcAutomaton::new(pattern_groups.patterns.clone());
    
    input
        .lines().enumerate()
        .flat_map(|(index, line)| {
            let line_number = index + 1;
            matcher::matcher(&automaton, &pattern_groups, line)
                .into_iter()
                .map(move |hint| Hint { line: Some(line_number), ..hint.clone()})
        })
        .collect()
}