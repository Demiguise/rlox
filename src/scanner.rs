use crate::tokens::{Token, TokenType};
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Scanner {
    source: String,

    start: usize,
    current: usize,
    line: usize,
}

lazy_static! {
    static ref keyword_map: HashMap<&'static str, TokenType> = {
        HashMap::from([
            ("and", TokenType::And),
            ("class", TokenType::Class),
            ("else", TokenType::Else),
            ("false", TokenType::False),
            ("for", TokenType::For),
            ("fun", TokenType::Fun),
            ("if", TokenType::If),
            ("nil", TokenType::Nil),
            ("or", TokenType::Or),
            ("print", TokenType::Print),
            ("return", TokenType::Return),
            ("super", TokenType::Super),
            ("this", TokenType::This),
            ("true", TokenType::True),
            ("var", TokenType::Var),
            ("while", TokenType::While),
        ])
    };
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
            .expect("Failed to get substring out of source");
        out.push(Token::create(t, substr.to_string(), self.line));
    }

    fn handle_number(&mut self, out: &mut Vec<Token>) {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Then we might encounter a '.'
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // Advance past the '.'
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        /*
            TODO: The book extracts the actual "Value" out of the source
            and parses it into a relevant string or number. We'll try that later.
        */
        self.add_token(TokenType::Number, out)
    }

    // Identifiers start with [a-zA-Z] but can have numbers in them
    fn handle_identifier(&mut self, out: &mut Vec<Token>) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let text = self
            .source
            .get(self.start..self.current)
            .expect("Failed to get the identifier string from source!");
        match keyword_map.get(text) {
            Some(t) => self.add_token(t.clone(), out),
            None => self.add_token(TokenType::Identifier, out),
        }
    }

    fn handle_string(&mut self, out: &mut Vec<Token>) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            // Unterminated string, should error here
            return;
        }

        self.advance(); // For the closing quote

        /*
            TODO: The book extracts the actual "Value" out of the source
            and parses it into a relevant string or number. We'll try that later.
        */
        //self.source.get(self.start + 1 .. self.current -1);
        self.add_token(TokenType::String, out);
    }

    fn scan_token(&mut self, out: &mut Vec<Token>) {
        // Macro to help setup the one-to-two character operators
        macro_rules! match_add {
            ($c:literal, $if_true: expr, $if_false: expr) => {
                match self.match_char($c) {
                    true => self.add_token($if_true, out),
                    false => self.add_token($if_false, out),
                };
            };
        }

        let c = self.advance();
        match c {
            // Single Character Tokens
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

            // One/Two Character Tokens
            '!' => match_add!('=', TokenType::BangEqual, TokenType::Bang),
            '=' => match_add!('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => match_add!('=', TokenType::LessEqual, TokenType::Less),
            '>' => match_add!('=', TokenType::GreaterEqual, TokenType::Greater),

            // White space (Does nothing)
            ' ' => {}
            '\r' => {}
            '\t' => {}

            // New line
            '\n' => self.line += 1,

            // String Literals
            '"' => self.handle_string(out),

            // Comments
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, out)
                }
            }
            _ => {
                if c.is_numeric() {
                    self.handle_number(out);
                } else if c.is_alphabetic() {
                    self.handle_identifier(out);
                } else {
                    println!("Unexpected char [{}]", c);
                }
            }
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current)
            .expect("Failed to get Nth char for peek");
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("Failed to get Nth char for peek");
    }

    fn advance(&mut self) -> char {
        let out = self
            .source
            .chars()
            .nth(self.current)
            .expect("Failed to get Nth char for peek");
        self.current += 1;
        return out;
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self
            .source
            .chars()
            .nth(self.current)
            .expect("Failed to get Nth char for match_char")
            != c
        {
            return false;
        }

        // Consume token and advance
        self.current += 1;
        return true;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}
