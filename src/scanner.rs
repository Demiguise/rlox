use crate::tokens::{Token, TokenType};

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn create(source: String) -> Self {
        Scanner {
            source: source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn add_token(&mut self, t: TokenType, out: &mut Vec<Token>) {
        let substr = self
            .source
            .get(self.start..self.current)
            .expect("TODO: Better message?");
        out.push(Token::create(t, substr.to_string(), self.line));
    }

    fn scan_token(&mut self, out: &mut Vec<Token>) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, out),
            ')' => self.add_token(TokenType::RightParen, out),
            '{' => self.add_token(TokenType::LeftBrace, out),
            '}' => self.add_token(TokenType::RightBrace, out),
            ',' => self.add_token(TokenType::Comma, out),
            '.' => self.add_token(TokenType::Dot, out),
            '-' => self.add_token(TokenType::Minus, out),
            '+' => self.add_token(TokenType::Plus, out),
            ';' => self.add_token(TokenType::SemiColon, out),
            '*' => self.add_token(TokenType::Star, out),
            _ => println!("Unexpected char [{}]", c),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut tokens)
        }

        tokens.push(Token::create(TokenType::Eof, "".to_string(), self.line));
        return tokens;
    }

    fn advance(&mut self) -> char {
        let out = self
            .source
            .chars()
            .nth(self.current)
            .expect("TODO: Better message?");
        self.current += 1;
        return out;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
