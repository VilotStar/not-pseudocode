pub mod error;
pub mod scanner;
pub mod token;

use std::ops::{Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

impl Sub<u32> for Position {
    type Output = Position;

    fn sub(mut self, rhs: u32) -> Self::Output {
        if self.column == 1 {
            self.line.sub_assign(rhs);
        } else {
            self.column.sub_assign(rhs);
        }

        self
    }
}
