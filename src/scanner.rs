use crate::tokens;

pub struct Scanner {
    source: String,
    tokens: Vec<tokens::Token>,

    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn create(source: String) -> Self {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
}
