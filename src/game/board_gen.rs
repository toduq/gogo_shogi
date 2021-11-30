use super::{Board, Piece, Turn};

/// Parses board expression
///
/// Initial board is expressed as follows.
/// 11wk,21wg,31ws,41wb,51wr,12wp,54bp,15br,25bb,35bs,45bg,55bk
///
/// Promoted piece is expressed as `35bS`.
/// Piece in hands is expressed as `__bs`.
pub fn from_str(s: &str) -> Board {
    let mut board = Board::empty();
    let chars = s.as_bytes();
    for i in 0..=(chars.len() / 5) {
        let turn = if chars[i * 5 + 2] == b'b' {
            Turn::Black
        } else {
            Turn::White
        };
        let piece = piece_repr(chars[i * 5 + 3]);
        if chars[i * 5] == b'_' {
            for i in 0..10 {
                if board.hands[i].is_absent() {
                    board.hands[i] = piece.of_turn(turn);
                    break;
                }
            }
        } else {
            let pos = (chars[i * 5 + 1] - b'0' - 1) * 5 + (5 - (chars[i * 5] - b'0'));
            board.squares[pos as usize] = piece.of_turn(turn);
        };
    }
    board
}

pub fn piece_repr(p: u8) -> Piece {
    match p {
        b'k' => Piece::BKing,
        b'g' => Piece::BGold,
        b's' => Piece::BSilver,
        b'b' => Piece::BBishop,
        b'r' => Piece::BRook,
        b'p' => Piece::BPawn,
        b'S' => Piece::BSilverP,
        b'B' => Piece::BBishopP,
        b'R' => Piece::BRookP,
        b'P' => Piece::BPawnP,
        _ => Piece::Invalid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_hands() {
        let generated = from_str("31wk,33bp,15bk,__bg,__wp");
        let mut expected = Board::empty();
        expected.squares[2] = Piece::WKing;
        expected.squares[12] = Piece::BPawn;
        expected.squares[24] = Piece::BKing;
        expected.hands[0] = Piece::BGold;
        expected.hands[1] = Piece::WPawn;
        assert_eq!(generated, expected);
        println!("{}", generated);
    }
}
