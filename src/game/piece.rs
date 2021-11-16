use super::Turn;
use once_cell::sync::Lazy;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    Absent = 0,
    Invalid,
    // not promoted
    BKing,
    WKing,
    BGold,
    WGold,
    BSilver,
    WSilver,
    BBishop,
    WBishop,
    BRook,
    WRook,
    BPawn,
    WPawn,
    // promoted (+8)
    BSilverP,
    WSilverP,
    BBishopP,
    WBishopP,
    BRookP,
    WRookP,
    BPawnP,
    WPawnP,
}

impl Piece {
    #[inline]
    pub const fn is_absent(self) -> bool {
        self.as_u8() == 0
    }

    #[inline]
    pub const fn from_u8(i: u8) -> Piece {
        REVERSE_INDEX[i as usize]
    }

    #[inline]
    pub const fn as_u8(self) -> u8 {
        self as u8
    }

    #[inline]
    pub fn turn(self) -> Turn {
        if self as u8 % 2 == 0 {
            Turn::Black
        } else {
            Turn::White
        }
    }

    #[inline]
    pub fn of_turn(self, turn: Turn) -> Piece {
        if self.turn() == turn {
            self
        } else {
            self.flip()
        }
    }

    #[inline]
    pub fn flip(self) -> Piece {
        if self as u8 % 2 == 0 {
            Piece::from_u8(self as u8 + 1)
        } else {
            Piece::from_u8(self as u8 - 1)
        }
    }

    pub fn to_str(self) -> &'static str {
        &BOARD_REPR[self as usize]
    }
}

const REVERSE_INDEX: [Piece; 22] = [
    Piece::Absent,
    Piece::Invalid,
    Piece::BKing,
    Piece::WKing,
    Piece::BGold,
    Piece::WGold,
    Piece::BSilver,
    Piece::WSilver,
    Piece::BBishop,
    Piece::WBishop,
    Piece::BRook,
    Piece::WRook,
    Piece::BPawn,
    Piece::WPawn,
    Piece::BSilverP,
    Piece::WSilverP,
    Piece::BBishopP,
    Piece::WBishopP,
    Piece::BRookP,
    Piece::WRookP,
    Piece::BPawnP,
    Piece::WPawnP,
];

static BOARD_REPR: Lazy<Vec<String>> = Lazy::new(|| {
    let mut v: Vec<String> = vec!["  ".to_string(), "  ".to_string()];
    for s in ["王", "金", "銀", "角", "飛", "歩", "全", "馬", "龍", "と"] {
        v.push(format!("\x1b[32m{}\x1b[m", s));
        v.push(format!("\x1b[33m{}\x1b[m", s));
    }
    v
});
