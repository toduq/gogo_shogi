#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Turn {
    Black,
    White,
}

impl Turn {
    pub fn next(&self) -> Turn {
        match *self {
            Turn::Black => Turn::White,
            Turn::White => Turn::Black,
        }
    }

    pub fn val(&self) -> i8 {
        match *self {
            Turn::Black => 1,
            Turn::White => -1,
        }
    }
}
