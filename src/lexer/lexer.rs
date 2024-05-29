use std::{iter::Peekable, str::Chars};

use super::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    current: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: Peekable<Chars<'a>>) -> Self {
        let mut lexer = Lexer {
            input,
            current: Default::default(),
        };
        lexer.read_char();

        lexer
    }
}

impl Lexer<'_> {
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();

        match self.current {
            'a'..='z' | 'A'..='Z' | '0'..='9' => Token::from(self.read_value()),
            '>' | '<' => {
                let c = self.current;
                if self.peek_char() == '=' || (self.current == '<' && self.peek_char() == '>') {
                    self.read_char();
                    let token = Token::from(format!("{c}{}", self.current));
                    self.read_char();
                    return token;
                } else {
                    let token = Token::from(self.current);
                    self.read_char();
                    return token;
                }
            }
            _ => {
                let token = Token::from(self.current);
                self.read_char();
                token
            }
        }
    }

    fn skip_whitespaces(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.current {
            self.read_char();
        }
    }

    fn read_value(&mut self) -> String {
        let mut value = String::new();
        while let 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' = self.current {
            value.push(self.current);
            self.read_char();
        }
        value
    }

    fn read_char(&mut self) {
        self.current = self.peek_char();
        self.input.next();
    }

    fn peek_char(&mut self) -> char {
        match self.input.peek() {
            Some(c) => *c,
            None => '\0',
        }
    }
}
