use crate::{
    token::{self, Token, TokenType},
    Position, error::PseudoError,
};

pub type Result<T> = std::result::Result<T, PseudoError>;

pub struct Scanner {
    source: String,
    position: Position,
    start: usize,
    current: usize,
}

impl Iterator for Scanner {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        Some(self.scan_token())
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let position = Position { line: 1, column: 1 };

        Self {
            source,
            start: 0,
            current: 0,
            position,
        }
    }

    fn scan_token(&mut self) -> Result<Token> {
        let c = self.advance()?;
        match c {
            '(' => Ok(self.construct_token(TokenType::LPAREN, self.position.clone() - 1)),
            ')' => Ok(self.construct_token(TokenType::RPAREN, self.position.clone() - 1)),
            '{' => Ok(self.construct_token(TokenType::LBRACE, self.position.clone() - 1)),
            '}' => Ok(self.construct_token(TokenType::RBRACE, self.position.clone() - 1)),
            ',' => Ok(self.construct_token(TokenType::COMMA, self.position.clone() - 1)),
            '-' => Ok(self.construct_token(TokenType::MINUS, self.position.clone() - 1)),
            '+' => Ok(self.construct_token(TokenType::PLUS, self.position.clone() - 1)),
            ';' => {
                Ok(self.construct_token(TokenType::SEMICOLON, self.position.clone() - 1))
            }
            '*' => Ok(self.construct_token(TokenType::STAR, self.position.clone() - 1)),
            '!' => {
                let start_position = self.position.clone();
                let token = if self.peek() == Some('=') {
                    self.advance()?;
                    TokenType::NEQUAL
                } else {
                    TokenType::NOT
                };
                Ok(self.construct_token(token, start_position))
            }
            '=' => {
                let start_position = self.position.clone();
                let token = if self.peek() == Some('=') {
                    self.advance()?;
                    TokenType::EEQUAL
                } else {
                    TokenType::EQUAL
                };
                Ok(self.construct_token(token, start_position))
            }
            '<' => {
                let start_position = self.position.clone();
                let token = if self.peek() == Some('=') {
                    self.advance()?;
                    TokenType::LEQUAL
                } else {
                    TokenType::LESS
                };
                Ok(self.construct_token(token, start_position))
            }
            '>' => {
                let start_position = self.position.clone();
                let token = if self.peek() == Some('=') {
                    self.advance()?;
                    TokenType::GEQUAL
                } else {
                    TokenType::GREATER
                };
                Ok(self.construct_token(token, start_position))
            }
            '/' => {
                let start_position = self.position.clone();
                let token = if self.peek() == Some('/') {
                    let advance = self.advance();
                    while (self.peek() != Some('\n')) && advance.is_ok() {
                        self.advance()?;
                    }
                    TokenType::COMMENT
                } else {
                    TokenType::SLASH
                };
                Ok(self.construct_token(token, start_position))
            }
            '"' => self.string(),
            '.' => {
                let token = if self.peek().unwrap().is_numeric() {
                    self.number(true)
                } else {
                    self.construct_token(TokenType::DOT, self.position.clone() - 1)
                };
                Ok(token)
            }
            ' ' | '\r' | '\t' => {
                let start_position = self.position.clone();
                while !self.at_end()
                    && (self.peek() == Some(' ')
                        || self.peek() == Some('\r')
                        || self.peek() == Some('\t'))
                {
                    self.advance()?;
                }
                Ok(self.construct_token(TokenType::WHITESPACE, start_position))
            }
            '\n' => {
                self.position.line += 1;
                self.position.column = 1;
                Ok(self.construct_token(TokenType::NEWLINE, self.position.clone() - 1))
            }
            '0'..='9' => Ok(self.number(false)),
            'a'..='z' | 'A'..='Z' => Ok(self.identifier()),
            c => {
                Err(PseudoError::ScannerError(self.position.clone(), format!("Unexpected char {}", c).into()))
            }
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Result<char> {
        if self.at_end() {
            return Err(PseudoError::ScannerError(self.position.clone(), "Advanced at end of program".into()))
        }
        self.current += 1;
        self.position.column += 1;
        Ok(self.source.chars().nth(self.current - 1).unwrap())
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current).unwrap())
    }

    fn string(&mut self) -> Result<Token> {
        let start_position = self.position.clone();
        loop {
            let c = self.advance()?;
            if c == '"' {
                let value = &self.source[self.start + 1..self.current - 1];
                return Ok(self.construct_token(TokenType::STRING(Box::from(value)), start_position))
            }
            if self.at_end() {
                break;
            }
        }

        Err(PseudoError::ScannerError(self.position.clone(), "Unterminated string".into()))
    }

    fn number(&mut self, initial: bool) -> Token {
        let start_position = self.position.clone();
        let mut is_float = initial;

        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            self.advance();
        }

        if self.peek() == Some('.') {
            is_float = true;
            self.advance();
     
            loop { 
                match self.peek() {
                    Some(c) => {
                        if c.is_numeric() {
                            self.advance();
                            continue;
                        }
                        break;
                    },
                    None => break,
                }
            }
        }

        let literal = &self.source[self.start..self.current];

        if is_float {
            self.construct_token(
            TokenType::FLOAT(literal.parse::<f64>().unwrap()),
                start_position,
            )
        } else {
            self.construct_token(
            TokenType::INTEGER(literal.parse::<i128>().unwrap()),
                start_position,
            )
        }
    }

    fn identifier(&mut self) -> Token {
        let start_position = self.position.clone();
        while self.peek().unwrap().is_alphabetic() {
            self.advance();
        }

        let literal = &self.source[self.start..self.current];

        match token::parse_keyword(literal) {
            Some(token_type) => self.construct_token(token_type.clone(), start_position),
            None => self
                .construct_token(TokenType::IDENTIFIER(Box::from(literal)), start_position),
        }
    }

    fn construct_token(&self, token: TokenType, start_position: Position) -> Token {
        Token::new(token, start_position, &self.position)
    }
}
