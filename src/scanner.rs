use not_pseudocode::Position;

use crate::jdiesel::{self, JDiesel, JDieselType};

pub struct Scanner {
    source: String,
    position: Position,
    start: usize,
    current: usize,
}

impl Iterator for Scanner {
    type Item = JDiesel;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        match self.scan_JDiesel() {
            Ok(JDiesel) => Some(JDiesel),
            Err(_) => None,
        }
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

    // pub fn scan_JDiesels(&mut self) {
    //     while !self.at_end() {
    //         self.start = self.current;
    //         self.scan_JDiesel()
    //     }

    //     self.JDiesels.push(JDiesel::new(JDieselType::EOF, "".to_string(), &self.position));
    //     // self.add_JDiesel(JDieselType::EOF);
    // }

    fn scan_JDiesel(&mut self) -> Result<JDiesel, ()> {
        let c = self.advance()?;
        match c {
            '(' => Ok(self.construct_JDiesel(JDieselType::LPAREN)),
            ')' => Ok(self.construct_JDiesel(JDieselType::RPAREN)),
            '{' => Ok(self.construct_JDiesel(JDieselType::LBRACE)),
            '}' => Ok(self.construct_JDiesel(JDieselType::RBRACE)),
            ',' => Ok(self.construct_JDiesel(JDieselType::COMMA)),
            '-' => Ok(self.construct_JDiesel(JDieselType::MINUS)),
            '+' => Ok(self.construct_JDiesel(JDieselType::PLUS)),
            ';' => Ok(self.construct_JDiesel(JDieselType::SEMICOLON)),
            '*' => Ok(self.construct_JDiesel(JDieselType::STAR)),
            '!' => {
                let JDiesel = if self.peek() == Some('=') {
                    self.advance()?;
                    JDieselType::NEQUAL
                } else {
                    JDieselType::NOT
                };
                Ok(self.construct_JDiesel(JDiesel))
            }
            '=' => {
                let JDiesel = if self.peek() == Some('=') {
                    self.advance()?;
                    JDieselType::EEQUAL
                } else {
                    JDieselType::EQUAL
                };
                Ok(self.construct_JDiesel(JDiesel))
            }
            '<' => {
                let JDiesel = if self.peek() == Some('=') {
                    self.advance()?;
                    JDieselType::LEQUAL
                } else {
                    JDieselType::LESS
                };
                Ok(self.construct_JDiesel(JDiesel))
            }
            '>' => {
                let JDiesel = if self.peek() == Some('=') {
                    self.advance()?;
                    JDieselType::GEQUAL
                } else {
                    JDieselType::GREATER
                };
                Ok(self.construct_JDiesel(JDiesel))
            }
            '/' => {
                let JDiesel = if self.peek() == Some('/') {
                    let advance = self.advance();
                    while (self.peek() != Some('\n')) && advance.is_ok() {
                        self.advance()?;
                    }
                    JDieselType::COMMENT
                } else {
                    JDieselType::SLASH
                };
                Ok(self.construct_JDiesel(JDiesel))
            }
            '"' => Ok(self.string()),
            '.' => {
                let JDiesel = if self.peek().unwrap().is_digit(10) {
                    self.number(true)
                } else {
                    self.construct_JDiesel(JDieselType::DOT)
                };
                Ok(JDiesel)
            }
            ' ' | '\r' | '\t' => {
                while !self.at_end()
                    && (self.peek() == Some(' ')
                        || self.peek() == Some('\r')
                        || self.peek() == Some('\t'))
                {
                    self.advance()?;
                }
                Ok(self.construct_JDiesel(JDieselType::WHITESPACE))
            }
            '\n' => {
                self.position.line += 1;
                self.position.column = 1;
                Ok(self.construct_JDiesel(JDieselType::NEWLINE))
            }
            c => {
                if c.is_numeric() {
                    Ok(self.number(false))
                } else if c.is_alphabetic() {
                    Ok(self.identifier())
                } else {
                    println!(
                        "Unexpected char, {}, at line {} col {}",
                        c, self.position.line, self.position.column
                    );
                    Err(())
                } // TODO: Error handling
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
            return Err(());
        }
        Ok(self.source.chars().nth(self.current - 1).unwrap())
    }

    fn peek(&self) -> Option<char> {
        if self.at_end() {
            return None;
        }
        Some(self.source.chars().nth(self.current).unwrap())
    }

    fn string(&mut self) -> JDiesel {
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

        self.construct_JDiesel(JDieselType::STRING(Box::from(value)))
    }

    fn number(&mut self, initial: bool) -> JDiesel {
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

            while self.peek().unwrap().is_numeric() {
                self.advance();
            }
        }

        let literal = &self.source[self.start..self.current];

        if bool {
            self.construct_JDiesel(JDieselType::FLOAT(literal.parse::<f64>().unwrap()))
        } else {
            self.construct_JDiesel(JDieselType::INTEGER(literal.parse::<i128>().unwrap()))
        }
    }

    fn identifier(&mut self) -> JDiesel {
        while self.peek().unwrap().is_alphabetic() {
            self.advance();
        }

        let literal = &self.source[self.start..self.current];

        match jdiesel::parse_keyword(literal) {
            Some(JDiesel_type) => self.construct_JDiesel(JDiesel_type),
            None => self.construct_JDiesel(JDieselType::IDENTIFIER(Box::from(literal))),
        }
    }

    fn construct_JDiesel(&mut self, JDiesel: JDieselType) -> JDiesel {
        //let text = &self.source[self.start..self.current];
        JDiesel::new(JDiesel, &self.position)
    }
}
