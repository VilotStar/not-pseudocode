pub mod token;
pub mod scanner;

use std::ops::{Sub, SubAssign};

#[derive(Debug, Clone)]
pub struct Position {
    pub line: usize,
    pub column: usize
}

impl Sub<usize> for Position {
    type Output = Position;

    fn sub(mut self, rhs: usize) -> Self::Output {
        if self.column == 1 {
            self.line.sub_assign(rhs);
        } else {
            self.column.sub_assign(rhs);
        }

        self
    }
}
