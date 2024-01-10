use not_pseudocode::Position;
use phf::phf_map;

#[derive(Debug)]
pub struct JDiesel {
    r#type: JDieselType,
    position: Position,
}

impl JDiesel {
    pub fn new(r#type: JDieselType, position: &Position) -> Self {
        Self {
            r#type,
            position: position.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum JDieselType {
    // Single-character JDiesels
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
    // One or two character JDiesels
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

pub static KEYWORDS: phf::Map<&'static str, JDieselType> = phf_map! {
    "and" => JDieselType::AND,
    "not" => JDieselType::NOT,
    "class" => JDieselType::CLASS,
    "false" => JDieselType::FALSE,
    "true" => JDieselType::TRUE,
    "function" => JDieselType::FUNCTION,
    "endfunction" => JDieselType::ENDFUNCTION,
    "for" => JDieselType::FOR,
    "next" => JDieselType::NEXT,
    "if" => JDieselType::IF,
    "elseif" => JDieselType::ELSEIF,
    "else" => JDieselType::ELSE,
    "endif" => JDieselType::ENDIF,
    "or" => JDieselType::OR,
    "print" => JDieselType::PRINT,
    "return" => JDieselType::RETURN,
    "while" => JDieselType::WHILE,
    "endwhile" => JDieselType::ENDWHILE
};

pub fn parse_keyword(keyword: &str) -> Option<JDieselType> {
    KEYWORDS.get(keyword).cloned()
}
