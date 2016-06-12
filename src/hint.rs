use std::fmt;

/// Suggestion on which words to change
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hint {
    /// Lint group that caught this word
    pub group: Option<&'static str>,
    /// The actual word
    pub value: String,
    /// Line this word appeared on (only set when supplied by calling function,
    /// should always be present when using the main `lint` function.)
    pub line: Option<usize>,
    /// Index the word starts at
    pub start: usize,
    /// Index the word ends at
    pub end: usize,
}

impl fmt::Display for Hint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "{group}: '{value}' ({line}:{start}-{end})",
            group=self.group.unwrap_or(""), value=self.value,
            line=self.line.map(|x| format!("{}", x)).unwrap_or("???".to_string()), start=self.start, end=self.end
        )
    }
}

