use super::Turn;
use once_cell::sync::Lazy;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Piece(pub u8);

static BOARD_REPR: Lazy<Vec<String>> = Lazy::new(|| {
    let mut v: Vec<String> = Vec::new();
    v.push("  ".to_string());
    v.push("  ".to_string());
    for s in ["王", "金", "銀", "角", "飛", "歩", "全", "馬", "龍", "と"] {
        v.push(format!("\x1b[32m{}\x1b[m", s));
        v.push(format!("\x1b[33m{}\x1b[m", s));
    }
    v
});

#[allow(dead_code)]
impl Piece {
    pub const ABSENT: Piece = Piece(0);
    // not promoted
    pub const B_KING: Piece = Piece(2);
    pub const W_KING: Piece = Piece(3);
    pub const B_GOLD: Piece = Piece(4);
    pub const W_GOLD: Piece = Piece(5);
    pub const B_SILVER: Piece = Piece(6);
    pub const W_SILVER: Piece = Piece(7);
    pub const B_BISHOP: Piece = Piece(8);
    pub const W_BISHOP: Piece = Piece(9);
    pub const B_ROOK: Piece = Piece(10);
    pub const W_ROOK: Piece = Piece(11);
    pub const B_PAWN: Piece = Piece(12);
    pub const W_PAWN: Piece = Piece(13);
    // promoted (+8)
    pub const B_SILVER_P: Piece = Piece(14);
    pub const W_SILVER_P: Piece = Piece(15);
    pub const B_BISHOP_P: Piece = Piece(16);
    pub const W_BISHOP_P: Piece = Piece(17);
    pub const B_ROOK_P: Piece = Piece(18);
    pub const W_ROOK_P: Piece = Piece(19);
    pub const B_PAWN_P: Piece = Piece(20);
    pub const W_PAWN_P: Piece = Piece(21);

    const DEBUG_REPR: [&'static str; 22] = [
        "ABSENT",
        "__INVALID__",
        "B_KING",
        "W_KING",
        "B_GOLD",
        "W_GOLD",
        "B_SILVER",
        "W_SILVER",
        "B_BISHOP",
        "W_BISHOP",
        "B_ROOK",
        "W_ROOK",
        "B_PAWN",
        "W_PAWN",
        "B_SILVER_P",
        "W_SILVER_P",
        "B_BISHOP_P",
        "W_BISHOP_P",
        "B_ROOK_P",
        "W_ROOK_P",
        "B_PAWN_P",
        "W_PAWN_P",
    ];

    pub const fn is_absent(&self) -> bool {
        self.0 == Piece::ABSENT.0
    }

    pub fn turn(&self) -> Turn {
        if self.0 % 2 == 0 {
            Turn::Black
        } else {
            Turn::White
        }
    }

    pub fn of_turn(&self, turn: Turn) -> Piece {
        if self.turn() == turn {
            *self
        } else {
            self.flip()
        }
    }

    pub fn flip(&self) -> Piece {
        if self.0 % 2 == 0 {
            Piece(self.0 + 1)
        } else {
            Piece(self.0 - 1)
        }
    }

    pub fn to_str(&self) -> &'static str {
        &BOARD_REPR[self.0 as usize]
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Piece({})", Piece::DEBUG_REPR[self.0 as usize])
    }
}
