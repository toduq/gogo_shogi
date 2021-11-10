mod piece;

pub use self::piece::Piece;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    // |turn(1)|none(3)|piece(5*25=125)|
    // squares: u128,
    pub squares: [Piece; 25],
    pub turn: Turn,
}

impl Board {
    pub fn init() -> Board {
        let mut s = [Piece::ABSENT; 25];
        s[0] = Piece::W_ROOK;
        s[1] = Piece::W_BISHOP;
        s[2] = Piece::W_SILVER;
        s[3] = Piece::W_GOLD;
        s[4] = Piece::W_KING;
        s[9] = Piece::W_PAWN;
        for i in 0..=9 {
            if s[i] != Piece::ABSENT {
                s[24 - i] = s[i].flip();
            }
        }
        Board {
            squares: s,
            turn: Turn::Black,
        }
    }

    pub const fn at(&self, pos: usize) -> Piece {
        self.squares[pos]
    }

    pub fn put_move(&mut self, m: &Move) {
        self.squares[m.src as usize] = Piece::ABSENT;
        self.squares[m.dst as usize] = m.piece;
        self.turn = self.turn.next()
    }

    pub fn is_finished(&self) -> (bool, Turn) {
        let mut b_king = false;
        let mut w_king = false;
        for p in &self.squares {
            match *p {
                Piece::B_KING => {
                    b_king = true;
                }
                Piece::W_KING => {
                    w_king = true;
                }
                _ => {}
            }
        }
        match (b_king, w_king) {
            (false, true) => (true, Turn::White),
            (true, false) => (true, Turn::Black),
            _ => (false, Turn::Black),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = "   0  1  2  3  4 \n".to_string();
        buf += "   --------------\n";
        for y in 0..=4 {
            buf.push_str(&format!(
                "{} |{} {} {} {} {}\n",
                y,
                self.at(y * 5 + 0).to_str(),
                self.at(y * 5 + 1).to_str(),
                self.at(y * 5 + 2).to_str(),
                self.at(y * 5 + 3).to_str(),
                self.at(y * 5 + 4).to_str(),
            ));
        }
        write!(f, "{}", buf)
    }
}

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

// |piece(5)|src(5)|dst(5)|promote(1)|
// 30 = drop from black hand
// 31 = drop from white hand
#[derive(Copy, Clone, Debug, PartialEq)]
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
