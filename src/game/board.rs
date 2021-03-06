use super::{board_gen, Move, Piece, Turn};

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub squares: [Piece; 25],
    pub hands: [Piece; 10],
    pub turn: Turn,
    pub won: Option<Turn>,
}

impl Board {
    pub fn init() -> Board {
        board_gen::from_str("11wk,21wg,31ws,41wb,51wr,12wp,54bp,15br,25bb,35bs,45bg,55bk")
    }

    pub fn empty() -> Board {
        Board {
            squares: [Piece::Absent; 25],
            hands: [Piece::Absent; 10],
            turn: Turn::Black,
            won: None,
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
            self.hands[(m.src - 100) as usize] = Piece::Absent;
        } else {
            // move in board
            self.squares[m.src as usize] = Piece::Absent;
        }
        if m.promote {
            self.squares[m.dst as usize] = Piece::from_u8(m.piece as u8 + 8);
        } else {
            self.squares[m.dst as usize] = m.piece;
        }

        self.turn = self.turn.next();

        if took != Piece::Absent {
            for i in 0..10 {
                if self.hands[i] == Piece::Absent {
                    self.hands[i] = if took.as_u8() >= 14 {
                        Piece::from_u8(took.flip() as u8 - 8)
                    } else {
                        took.flip()
                    };
                    break;
                }
            }
            if took.of_turn(Turn::Black) == Piece::BKing {
                self.won = Some(took.flip().turn());
            }
        }
    }

    #[allow(unused)]
    pub fn flip_turn(&mut self) -> &Board {
        self.turn = self.turn.next();
        self
    }

    // There are 23 absent squares and it uses 23 bits.
    // There are 12 occupied squares and it uses 12*6=72bits.
    // 1(turn) + 23(empty) + 72(occupied) = 96bits.
    #[allow(unused)]
    pub fn u128_repr(&self) -> u128 {
        let mut hash = 0u128;
        hash |= self.turn as u128;
        for p in self.squares {
            if p.is_absent() {
                // absent is represented by 0
                hash <<= 1;
            } else {
                // piece is reprersented by 1|piece
                hash <<= 6;
                hash |= 1 << 5;
                hash |= p as u128;
            }
        }
        let mut hands_sorted = self.hands;
        hands_sorted.sort();
        for p in hands_sorted {
            if p.is_absent() {
                hash <<= 1;
            } else {
                hash <<= 6;
                hash |= 1 << 5;
                hash |= p as u128;
            }
        }
        hash
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = "   5  4  3  2  1 \n".to_string();
        buf += "   --------------\n";
        for y in 0..=4 {
            buf.push_str(&format!(
                "{} |{} {} {} {} {}\n",
                y + 1,
                self.at(y * 5).to_str(),
                self.at(y * 5 + 1).to_str(),
                self.at(y * 5 + 2).to_str(),
                self.at(y * 5 + 3).to_str(),
                self.at(y * 5 + 4).to_str(),
            ));
        }
        buf.push_str("Hands: ");
        for h in self.hands {
            if h != Piece::Absent {
                buf.push_str(&format!("{} ", h.to_str()));
            }
        }
        write!(f, "{}", buf)
    }
}
