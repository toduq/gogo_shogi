use super::Piece;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Move {
    pub piece: Piece,
    pub src: u8,
    pub dst: u8,
    pub promote: bool,
}

impl Move {
    pub fn new(piece: &Piece, src: u8, dst: u8, promote: bool) -> Move {
        Move {
            piece: *piece,
            src,
            dst,
            promote,
        }
    }
}
