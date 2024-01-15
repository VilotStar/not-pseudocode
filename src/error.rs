use std::{borrow::Cow, fmt::{Formatter, Display}};

use crate::Position;

#[derive(Debug)]
pub enum PseudoError {
    ScannerError(Position, Cow<'static, str>)
}

impl Display for PseudoError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::ScannerError(position, ref reason) => {
                write!(f, "Scanner error at line {} col {}: {}", position.line, position.column, reason)
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, PseudoError>;