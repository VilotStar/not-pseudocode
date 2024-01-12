use std::borrow::Cow;

use crate::Position;

#[derive(Debug)]
pub enum PseudoError {
    ScannerError(Position, Cow<'static, str>)
}