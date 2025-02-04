pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
    result: Vec<Token>,
}

impl Scanner {
    pub fn new(input: &str) -> Self {
        return Scanner {
            source: String::from(input),
            start: 0,
            current: 0,
            line: 1,
            result: vec![],
        };
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            let char = self.advance();
            match char {
                '(' => self.append_token(TokenType::LEFT_PAREN),
                ')' => self.append_token(TokenType::RIGHT_PAREN),
                '{' => self.append_token(TokenType::LEFT_BRACE),
                '}' => self.append_token(TokenType::RIGHT_BRACE),
                ',' => self.append_token(TokenType::COMMA),
                '.' => self.append_token(TokenType::DOT),
                '-' => self.append_token(TokenType::MINUS),
                '+' => self.append_token(TokenType::PLUS),
                ';' => self.append_token(TokenType::SEMICOLON),
                '*' => self.append_token(TokenType::STAR),
                '\n' => self.line = self.line + 1,
                c => eprintln!("Unexpected character {}", c),
            }
        }

        self.result.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::from(""),
            literal: String::from(""),
            line: self.line,
        });

        return self.result.clone();
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let chars: Vec<char> = self.source.chars().collect();
        let result = chars[self.current];
        self.current = self.current + 1;
        result
    }

    fn append_token(&mut self, token_type: TokenType) -> () {
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
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}
