//! Find common stylistic problems in english texts like technical or scientific
//! documents. Based on [write-good][npm-write-good] and [this article][matt].
//!
//! [npm-write-good]: https://github.com/btford/write-good
//! [matt]: http://matt.might.net/articles/shell-scripts-for-passive-voice-weasel-words-duplicates/
#![deny(missing_docs, dead_code)]

extern crate aho_corasick;
extern crate regex;
#[macro_use] extern crate lazy_static;

mod range_map;

mod data;
mod hint;
mod matcher;
mod pattern_groups;

use aho_corasick::AcAutomaton;

pub use hint::Hint;

/// Lint a string of english text
///
/// Given some english text, this function will try to give suggestions on how
/// to improve the language.
///
/// # Examples
///
/// ```
/// extern crate english_lint;
///
/// let text: &str = "This chapter consists of relatively independent tutorials";
/// let suggestions: Vec<english_lint::Hint> = english_lint::lint(text);
///
/// assert_eq!(suggestions, vec![
///     english_lint::Hint { group: Some("adverbs"),
///                          value: "relatively".to_owned(),
///                          line: Some(1), start: 25, end: 35 },
/// ]);
/// ```
pub fn lint(input: &str) -> Vec<Hint> {
    let mut pattern_groups = pattern_groups::PatternGroups::with_capacity(32_000);
    pattern_groups.push(data::ADVERBS, "adverbs");
    pattern_groups.push(data::WEAKENS, "weakens");
    pattern_groups.push(data::CLICHES, "cliches");
    pattern_groups.push(data::WORDY, "wordy");
    pattern_groups.push(data::WEASELS, "weasels");

    let automaton = AcAutomaton::new(pattern_groups.patterns.clone());

    input
        .lines().enumerate()
        .flat_map(|(index, line)| {
            let line_number = index + 1;

            let irregulars = data::PASSIVE.find_iter(line)
                .map(move |(start, end)| Hint {
                    group: Some("passive"),
                    value: line[start..end].to_owned(),
                    line: Some(line_number.clone()),
                    start: start,
                    end: end,
                 });

            matcher::matcher(&automaton, &pattern_groups, line)
                .into_iter()
                .map(move |hint| Hint { line: Some(line_number.clone()), ..hint.clone()})
                .chain(irregulars)
        })
        .collect()
}
