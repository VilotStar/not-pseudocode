use phf::phf_map;

use crate::Position;

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    position: (Position, Position),
}

impl Token {
    pub fn new(
        r#type: TokenType,
        start_position: Position,
        end_position: &Position,
    ) -> Self {
        Self {
            r#type,
            position: (start_position, end_position.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,
    // One or two character tokens
    NEQUAL,
    EQUAL,
    EEQUAL,
    GREATER,
    GEQUAL,
    LESS,
    LEQUAL,
    // Literals
    IDENTIFIER(Box<str>),
    STRING(Box<str>),
    INTEGER(i128),
    FLOAT(f64),
    // Keywords
    AND,
    NOT,
    CLASS,
    FALSE,
    TRUE,
    FUNCTION,
    ENDFUNCTION,
    FOR,
    NEXT,
    IF,
    ELSEIF,
    ELSE,
    ENDIF,
    OR,
    PRINT,
    RETURN,
    WHILE,
    ENDWHILE,
    // Bullshit
    COMMENT,
    WHITESPACE,
    NEWLINE,
    // End Of File
    EOF,
}

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::AND,
    "not" => TokenType::NOT,
    "class" => TokenType::CLASS,
    "false" => TokenType::FALSE,
    "true" => TokenType::TRUE,
    "function" => TokenType::FUNCTION,
    "endfunction" => TokenType::ENDFUNCTION,
    "for" => TokenType::FOR,
    "next" => TokenType::NEXT,
    "if" => TokenType::IF,
    "elseif" => TokenType::ELSEIF,
    "else" => TokenType::ELSE,
    "endif" => TokenType::ENDIF,
    "or" => TokenType::OR,
    "print" => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "while" => TokenType::WHILE,
    "endwhile" => TokenType::ENDWHILE
};

pub fn parse_keyword(keyword: &str) -> Option<&TokenType> {
    KEYWORDS.get(&keyword.to_lowercase())
}
