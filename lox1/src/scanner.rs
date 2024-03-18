mod token;

use self::token::Token;
use crate::error_reporter::ErrorReporter;

#[derive(Default)]
pub struct Scanner {
    pub data: String,
    pub file_name: String,
    pub had_error: bool,

    start: u32,
    current: u32,
    line: u32,
    col: u32,
}

impl Scanner {
    pub fn scan_tokens(&mut self) -> Vec<Token>  {
        let mut tokens: Vec<Token> = Vec::new();
        while !self.is_at_end() {
            tokens.push(self.scan_token());
        }

        tokens.push(Token {
            line: self.line + 1,
            col: 0,
            ..Default::default()
        });
        tokens
    }

    fn is_at_end(&self) -> bool {
        // hopefully nothing bad happens
        self.current >= self.data.len().try_into().unwrap()
    }

    fn scan_token(&self) -> Token {

    }

    fn advance(&mut self) -> char {
        self.get_char_at(++self.current)
    }

    fn get_char_at(&self, index: usize) -> char {
        // this is why this program doesn't support utf8 files
        self.data.as_bytes()[index]
    } 
}

impl ErrorReporter for Scanner {
    fn error(&mut self, line: u32, col: u32, file: String, kind: String, message: String) {
        println!("[{line}:{col}@{file}] {kind} Error: {message}");
        self.had_error = true;
    }
}
