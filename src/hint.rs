use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hint {
    pub group: Option<&'static str>,
    pub value: String,
    pub line: Option<usize>,
    pub start: usize,
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

