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
                    if self.r#match('=') {
                        self.append_token(BangEqual);
                    } else {
                        self.append_token(Bang);
                    }
                }
                '=' => {
                    if self.r#match('=') {
                        self.append_token(EqualEqual);
                    } else {
                        self.append_token(Equal);
                    }
                }
                '<' => {
                    if self.r#match('=') {
                        self.append_token(LessEqual);
                    } else {
                        self.append_token(Less);
                    }
                }
                '>' => {
                    if self.r#match('=') {
                        self.append_token(GreaterEqual);
                    } else {
                        self.append_token(Greater);
                    }
                }
                '\n' => self.line += 1,
                c => {
                    let message = format!("Unexpected character: {}", c);
                    lox::error(self.line, &message);
                }
            }
        }

        self.result.push(Token {
            token_type: TokenType::Eof,
            lexeme: String::from(""),
            literal: String::from(""),
            line: self.line,
        });

        self.result.clone()
    }

    fn r#match(&mut self, c: char) -> bool {
        if self.at_end() || self.chars[self.current] != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn advance(&mut self) -> char {
        let result = self.chars[self.current];
        self.current += 1;
        result
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn append_token(&mut self, token_type: TokenType) {
        let token = Token {
            token_type,
            lexeme: String::from(&self.source[self.start..self.current]),
            literal: String::from(""),
            line: self.line,
        };
        self.result.push(token);
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

#[derive(Debug, Clone)]
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
