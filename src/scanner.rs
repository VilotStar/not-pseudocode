use not_pseudocode::Position;

use crate::token::{Token, TokenType, self};

pub struct Scanner {
    source: String,
    position: Position,
    start: usize,
    current: usize
}

impl Iterator for Scanner {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        match self.scan_token() {
            Ok(token) => Some(token),
            Err(_) => None
        }
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let position = Position {
            line: 1,
            column: 1
        };

        Self {
            source,
            start: 0,
            current: 0,
            position
        }
    }

    // pub fn scan_tokens(&mut self) {
    //     while !self.at_end() {
    //         self.start = self.current;
    //         self.scan_token()
    //     }

    //     self.tokens.push(Token::new(TokenType::EOF, "".to_string(), &self.position));
    //     // self.add_token(TokenType::EOF);
    // }

    fn scan_token(&mut self) -> Result<Token, ()> {
        let c = self.advance()?;
        match c {
            '(' => Ok(self.construct_token(TokenType::LPAREN)),
            ')' => Ok(self.construct_token(TokenType::RPAREN)),
            '{' => Ok(self.construct_token(TokenType::LBRACE)),
            '}' => Ok(self.construct_token(TokenType::RBRACE)),
            ',' => Ok(self.construct_token(TokenType::COMMA)),
            '-' => Ok(self.construct_token(TokenType::MINUS)),
            '+' => Ok(self.construct_token(TokenType::PLUS)),
            ';' => Ok(self.construct_token(TokenType::SEMICOLON)),
            '*' => Ok(self.construct_token(TokenType::STAR)),
            '!' => {
                let token = if self.peek() == Some('=') { self.advance()?; TokenType::NEQUAL } else { TokenType::NOT };
                Ok(self.construct_token(token))
            },
            '=' => {
                let token = if self.peek() == Some('=') { self.advance()?; TokenType::EEQUAL} else {TokenType::EQUAL};
                Ok(self.construct_token(token))
            },
            '<' => {
                let token = if self.peek() == Some('=') {self.advance()?; TokenType::LEQUAL} else {TokenType::LESS};
                Ok(self.construct_token(token))
                        },
            '>' => {
                let token = if self.peek() == Some('=') {self.advance()?; TokenType::GEQUAL} else {TokenType::GREATER};
                Ok(self.construct_token(token))
                        },
            '/' => {
                let token = if self.peek() == Some('/') {
                    let advance = self.advance();
                    while (self.peek() != Some('\n')) && advance.is_ok() { self.advance()?; }
                    TokenType::COMMENT
                } else {
                    TokenType::SLASH
                };
                Ok(self.construct_token(token))
            }
            '"' => Ok(self.string()),
            '.' => {
                let token = if self.peek().unwrap().is_digit(10) { self.number(true) } else { self.construct_token(TokenType::DOT) };
                Ok(token)
            },
            ' ' | '\r' | '\t' => {
                while !self.at_end() && (self.peek() == Some(' ') || self.peek() == Some('\r') || self.peek() == Some('\t')) {
                    self.advance()?;
                }
                Ok(self.construct_token(TokenType::WHITESPACE))
            },
            '\n' => {
                self.position.line += 1;
                self.position.column = 1;
                Ok(self.construct_token(TokenType::NEWLINE))
            },
            c => {
                if c.is_numeric() { Ok(self.number(false)) }
                else if c.is_alphabetic() { Ok(self.identifier()) }
                else { println!("Unexpected char, {}, at line {} col {}", c, self.position.line, self.position.column); Err(()) } // TODO: Error handling
            }
        }
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Result<char, ()> {
        self.current += 1;
        self.position.column += 1;
        if self.at_end() {
            return Err(())
        }
        Ok(self.source.chars().nth(self.current - 1).unwrap())
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() {
            return None
        }
        Some(self.source.chars().nth(self.current).unwrap())
    }

    fn string(&mut self) -> Token {
        while self.peek() != Some('"') && !self.at_end() {
            if self.peek() == Some('\n') {
                self.position.line += 1;
                self.position.column = 1;
            }
            self.advance();
        }

        if self.at_end() {
            println!("Unterminated string"); // TODO: Error handling
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        self.construct_token(TokenType::STRING(Box::from(value)))
    }

    fn number(&mut self, initial: bool) -> Token {
        let mut bool = initial;
        while let Some(c) = self.peek() {
            if !c.is_numeric() {
                break;
            }
            self.advance();
         }

        if self.peek() == Some('.') {
            bool = true;
            self.advance();

            while self.peek().unwrap().is_numeric() { self.advance(); }
        }

        let literal = &self.source[self.start..self.current];

        if bool {
            self.construct_token(TokenType::FLOAT(literal.parse::<f64>().unwrap()))
        } else {
            self.construct_token(TokenType::INTEGER(literal.parse::<i128>().unwrap()))
        }
    }

    fn identifier(&mut self) -> Token {
        while self.peek().unwrap().is_alphabetic() { self.advance(); }

        let literal = &self.source[self.start..self.current];

        match token::parse_keyword(literal) {
            Some(token_type) => self.construct_token(token_type),
            None => self.construct_token(TokenType::IDENTIFIER(Box::from(literal)))
        }
    }

    fn construct_token(&mut self, token: TokenType) -> Token {
        //let text = &self.source[self.start..self.current];
        Token::new(token, &self.position)
    }
}
