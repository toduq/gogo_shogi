use super::{Piece, Turn, Move};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    pub squares: [Piece; 25],
    pub turn: Turn,
    pub hands: [Piece; 10],
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
            hands: [Piece::ABSENT; 10],
        }
    }

    #[cfg(test)]
    pub fn empty() -> Board {
        Board {
            squares: [Piece::ABSENT; 25],
            turn: Turn::Black,
            hands: [Piece::ABSENT; 10],
        }
    }

    pub fn copy_from(&mut self, from: &Board) {
        self.squares = from.squares;
        self.turn = from.turn;
        self.hands = from.hands;
    }

    pub const fn at(&self, pos: usize) -> Piece {
        self.squares[pos]
    }

    pub fn put_move(&mut self, m: &Move) {
        let took = self.squares[m.dst as usize];

        if m.src >= 100 {
            // from hands
            self.hands[(m.src - 100) as usize] = Piece::ABSENT;
        } else {
            // move in board
            self.squares[m.src as usize] = Piece::ABSENT;
        }
        if m.promote {
            self.squares[m.dst as usize] = Piece(m.piece.0 + 8);
        } else {
            self.squares[m.dst as usize] = m.piece;
        }

        self.turn = self.turn.next();

        if took != Piece::ABSENT {
            for i in 0..10 {
                if self.hands[i] == Piece::ABSENT {
                    self.hands[i] = if took.0 >= 14 {
                        Piece(took.flip().0 - 8)
                    } else {
                        took.flip()
                    };
                    break;
                }
            }
        }
    }

    pub fn flip_turn(&mut self) {
        self.turn = self.turn.next();
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
        buf.push_str("Hands: ");
        for h in self.hands {
            if h != Piece::ABSENT {
                buf.push_str(&format!("{} ", h.to_str()));
            }
        }
        write!(f, "{}", buf)
    }
}