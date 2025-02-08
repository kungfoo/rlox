use core::{cmp::PartialEq, prelude::v1::derive};
use std::thread::current;

use crate::lox;
use TokenType::*;

pub struct Scanner {
    source: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    result: Vec<Token>,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        Scanner {
            source: String::from(input),
            chars: input.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            result: vec![],
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            let char = self.advance();
            match char {
                '(' => self.append_token(LeftParen),
                ')' => self.append_token(RightParen),
                '{' => self.append_token(LeftBrace),
                '}' => self.append_token(RightBrace),
                ',' => self.append_token(Comma),
                '.' => self.append_token(Dot),
                '-' => self.append_token(Minus),
                '+' => self.append_token(Plus),
                ';' => self.append_token(Semicolon),
                '*' => self.append_token(Star),
                '!' => {
                    if self.next_is('=') {
                        self.append_token(BangEqual);
                    } else {
                        self.append_token(Bang);
                    }
                }
                '=' => {
                    if self.next_is('=') {
                        self.append_token(EqualEqual);
                    } else {
                        self.append_token(Equal);
                    }
                }
                '<' => {
                    if self.next_is('=') {
                        self.append_token(LessEqual);
                    } else {
                        self.append_token(Less);
                    }
                }
                '>' => {
                    if self.next_is('=') {
                        self.append_token(GreaterEqual);
                    } else {
                        self.append_token(Greater);
                    }
                }
                '/' => {
                    if self.next_is('/') {
                        while self.peek() != '\n' && !self.at_end() {
                            // keep eating character until the end of the line
                            self.advance();
                        }
                    } else {
                        self.append_token(Slash);
                    }
                }
                '0'..='9' => self.consume_number(),
                '"' => self.consume_string(),
                '\n' => self.line += 1,
                '\t' => {}
                c => {
                    let message = format!("Unexpected character: {}", c);
                    lox::error(self.line, &message);
                }
            }
        }

        self.result.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: LiteralType::Nil,
            line: self.line,
        });

        self.result.clone()
    }

    fn next_is(&mut self, c: char) -> bool {
        if self.at_end() || self.chars[self.current] != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.chars[self.current + 1];
    }

    fn advance(&mut self) -> char {
        let result = self.chars[self.current];
        self.current += 1;
        result
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn consume_string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            lox::error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        let value = &self.source[self.start + 1..self.current - 1];
        self.append_token_literal(TString, LiteralType::StringLiteral(String::from(value)));
    }

    fn consume_number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            //consume the .
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        let number: f32 = value
            .parse::<f32>()
            .expect("Could not convert value to f32");
        self.append_token_literal(Number, LiteralType::NumberLiteral(number));
    }

    fn is_digit(&self, c: char) -> bool {
        match c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn append_token(&mut self, token_type: TokenType) {
        self.append_token_literal(token_type, LiteralType::Nil);
    }

    fn append_token_literal(&mut self, token_type: TokenType, literal: LiteralType) {
        let token = Token {
            token_type,
            lexeme: String::from(&self.source[self.start..self.current]),
            literal,
            line: self.line,
        };
        self.result.push(token);
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: LiteralType,
    line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    TString,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
enum LiteralType {
    Nil,
    StringLiteral(String),
    NumberLiteral(f32),
    BooleanLiteral(bool),
}

#[cfg(test)]
mod tests {
    use core::assert_eq;

    use super::*;

    fn scan(input: &str) -> Vec<TokenType> {
        Scanner::new(input)
            .scan_tokens()
            .iter()
            .map(|t| t.token_type.clone())
            .collect()
    }

    #[test]
    fn it_reads_single_line_comments() {
        assert_eq!(scan("// just a comment"), vec![Eof]);
    }

    #[test]
    fn it_reads_bang_equals() {
        assert_eq!(scan("asdf != foo;"), vec![BangEqual, Semicolon, Eof]);
    }

    #[test]
    fn it_reads_bang_operator() {
        assert_eq!(scan("10 ! 20;"), vec![Number, Bang, Number, Semicolon, Eof]);
    }

    #[test]
    fn it_reads_a_bunch_of_single_character_lexemes() {
        assert_eq!(
            scan("() {} . <> * "),
            vec![LeftParen, RightParen, LeftBrace, RightBrace, Dot, Less, Greater, Star, Eof]
        )
    }

    #[test]
    fn it_reads_a_string_literal() {
        assert_eq!(scan("\"this is a string\";"), vec![TString, Semicolon, Eof]);
    }

    #[test]
    fn it_reads_a_boolean_statement_with_strings() {
        assert_eq!(
            scan("\"asdf\" == \"boo\";"),
            vec![TString, EqualEqual, TString, Semicolon, Eof]
        )
    }

    #[test]
    fn it_reads_an_integer_number() {
        assert_eq!(scan("var foo = 9;"), vec![Equal, Number, Semicolon, Eof])
    }

    #[test]
    fn it_reads_a_float_number() {
        assert_eq!(
            scan("var foo = 9.123455443;"),
            vec![Equal, Number, Semicolon, Eof]
        )
    }
}
