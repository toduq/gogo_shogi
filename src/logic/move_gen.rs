use super::*;
use once_cell::sync::Lazy;
use std::collections::{HashMap, HashSet};

pub fn all_valid_moves(board: &Board) -> Vec<Move> {
    valid_moves(board, board.turn, true)
}

// returns only taking-moves or evasion moves
pub fn qsearch_moves(board: &Board) -> Vec<Move> {
    let mut next_board = board.clone();
    if is_checked(board) {
        // evasion moves
        valid_moves(board, board.turn, true)
            .into_iter()
            .filter(|m| {
                next_board.copy_from(board);
                next_board.put_move(m);
                next_board.flip_turn();
                !is_checked(&next_board)
            })
            .collect()
    } else {
        // taking moves and checking moves
        valid_moves(board, board.turn, true)
            .into_iter()
            .filter(|m| {
                if !board.at(m.dst as usize).is_absent() {
                    return true;
                }
                next_board.copy_from(board);
                next_board.put_move(m);
                is_checked(&next_board)
            })
            .collect()
    }
}

fn valid_moves(board: &Board, turn: Turn, include_dropping: bool) -> Vec<Move> {
    if board.won.is_some() {
        return vec![];
    }
    let mut moves = Vec::new();

    // move piece
    for (pos, piece) in board.squares.iter().enumerate() {
        if *piece == Piece::Absent || piece.turn() != turn {
            continue;
        }
        for ms in &PIECE_MOVES_WITH_POSITION[&(*piece as u8, pos)] {
            for m in ms {
                let dst_piece = board.at(m.dst as usize);
                if dst_piece.is_absent() {
                    // empty. can go through
                    moves.push(*m);
                } else if dst_piece.turn() != turn {
                    // occupied by opponent. have to stop.
                    moves.push(*m);
                    break;
                } else {
                    // occupied by same color. can't enter.
                    break;
                }
            }
        }
    }

    if include_dropping {
        let mut generated: HashSet<Piece> = HashSet::new();
        for (pos, piece) in board.hands.iter().enumerate() {
            if *piece == Piece::Absent || piece.turn() != turn || generated.contains(piece) {
                continue;
            }
            generated.insert(*piece);
            for dst in 0..25 {
                if board.at(dst) != Piece::Absent {
                    continue;
                }
                moves.push(Move::new(piece, (100 + pos) as u8, dst as u8, false))
            }
        }
    }
    moves
}

pub fn is_checked(b: &Board) -> bool {
    let my_king = Piece::BKing.of_turn(b.turn);
    match b.squares.iter().enumerate().find(|p| *p.1 == my_king) {
        None => false, // lose
        Some((my_king_pos, _)) => valid_moves(b, b.turn.next(), false)
            .iter()
            .any(|m| m.dst == my_king_pos as u8),
    }
}

// The followings are cache

#[allow(clippy::type_complexity)]
static PIECE_MOVES: Lazy<HashMap<u8, Vec<Vec<(i8, i8)>>>> = Lazy::new(|| {
    let mut map = HashMap::new();

    let king = vec![
        vec![(-1, -1)],
        vec![(-1, 0)],
        vec![(-1, 1)],
        vec![(0, -1)],
        vec![(0, 1)],
        vec![(1, -1)],
        vec![(1, 0)],
        vec![(1, 1)],
    ];
    let gold = vec![
        vec![(-1, -1)],
        vec![(-1, 0)],
        vec![(-1, 1)],
        vec![(0, -1)],
        vec![(0, 1)],
        vec![(1, 0)],
    ];
    let silver = vec![
        vec![(-1, -1)],
        vec![(-1, 0)],
        vec![(-1, 1)],
        vec![(1, -1)],
        vec![(1, 1)],
    ];
    let bishop = vec![
        vec![(-1, -1), (-2, -2), (-3, -3), (-4, -4)],
        vec![(-1, 1), (-2, 2), (-3, 3), (-4, 4)],
        vec![(1, -1), (2, -2), (3, -3), (4, -4)],
        vec![(1, 1), (2, 2), (3, 3), (4, 4)],
    ];
    let bishop_p = vec![
        vec![(-1, -1), (-2, -2), (-3, -3), (-4, -4)],
        vec![(-1, 1), (-2, 2), (-3, 3), (-4, 4)],
        vec![(1, -1), (2, -2), (3, -3), (4, -4)],
        vec![(1, 1), (2, 2), (3, 3), (4, 4)],
        vec![(-1, 0)],
        vec![(0, -1)],
        vec![(0, 1)],
        vec![(1, 0)],
        vec![(1, 1)],
    ];
    let rook = vec![
        vec![(-1, 0), (-2, 0), (-3, 0), (-4, 0)],
        vec![(0, -1), (0, -2), (0, -3), (0, -4)],
        vec![(0, 1), (0, 2), (0, 3), (0, 4)],
        vec![(1, 0), (2, 0), (3, 0), (4, 0)],
    ];
    let rook_p = vec![
        vec![(-1, 0), (-2, 0), (-3, 0), (-4, 0)],
        vec![(0, -1), (0, -2), (0, -3), (0, -4)],
        vec![(0, 1), (0, 2), (0, 3), (0, 4)],
        vec![(1, 0), (2, 0), (3, 0), (4, 0)],
        vec![(-1, -1)],
        vec![(-1, 1)],
        vec![(1, -1)],
        vec![(1, 1)],
    ];
    let pawn = vec![vec![(-1, 0)]];

    for i in 2..=21 {
        let piece = Piece::from_u8(i);
        let piece_of_black = piece.of_turn(Turn::Black);

        let mut v = match piece_of_black {
            Piece::BKing => king.clone(),
            Piece::BGold => gold.clone(),
            Piece::BSilver => silver.clone(),
            Piece::BBishop => bishop.clone(),
            Piece::BRook => rook.clone(),
            Piece::BPawn => pawn.clone(),
            Piece::BSilverP => gold.clone(),
            Piece::BBishopP => bishop_p.clone(),
            Piece::BRookP => rook_p.clone(),
            Piece::BPawnP => gold.clone(),
            _ => vec![],
        };

        if piece.turn() == Turn::White {
            v = v
                .iter()
                .map(|v2| v2.iter().map(|l| (-l.0, l.1)).collect())
                .collect();
        }

        map.insert(piece.as_u8(), v);
    }
    map
});

#[allow(clippy::type_complexity)]
static PIECE_MOVES_WITH_POSITION: Lazy<HashMap<(u8, usize), Vec<Vec<Move>>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for pos in 0..=24 {
        let y = (pos / 5) as i8;
        let x = (pos % 5) as i8;
        for (piece, moves) in PIECE_MOVES.iter() {
            let v = moves
                .iter()
                .map(|vs| {
                    vs.iter()
                        .filter(|(dy, dx)| y + dy >= 0 && y + dy <= 4 && x + dx >= 0 && x + dx <= 4)
                        .flat_map(|(dy, dx)| {
                            let p = Piece::from_u8(*piece);
                            let src = (y * 5 + x) as u8;
                            let dst = ((y + dy) * 5 + x + dx) as u8;
                            if ((y + dy == 0 && piece % 2 == 0) || (y + dy == 4 && piece % 2 == 1))
                                && (6 <= *piece && *piece <= 13)
                            {
                                // with promotion
                                if p.of_turn(Turn::Black) == Piece::BSilver {
                                    vec![
                                        Move::new(&p, src, dst, true),
                                        Move::new(&p, src, dst, false),
                                    ]
                                } else {
                                    // bishop, rook and pawn must promote
                                    vec![Move::new(&p, src, dst, true)]
                                }
                            } else {
                                vec![Move::new(&p, src, dst, false)]
                            }
                        })
                        .collect()
                })
                .collect();
            map.insert((*piece, pos), v);
        }
    }
    map
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qsearch_moves_taking() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::WKing, 109, 2, false));
        b.put_move(&Move::new(&Piece::WGold, 109, 6, false));
        b.put_move(&Move::new(&Piece::WSilver, 109, 8, false));
        b.put_move(&Move::new(&Piece::BGold, 109, 12, false));
        b.put_move(&Move::new(&Piece::BKing, 109, 17, false));
        b.flip_turn();
        println!("{}", b);

        assert!(!is_checked(&b));

        let result: HashSet<Move> = qsearch_moves(&b).into_iter().collect();
        let expected: HashSet<Move> = vec![
            Move::new(&Piece::BGold, 12, 6, false),
            Move::new(&Piece::BGold, 12, 7, false),
            Move::new(&Piece::BGold, 12, 8, false),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn qsearch_moves_evasion() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::WKing, 109, 2, false));
        b.put_move(&Move::new(&Piece::WGold, 109, 6, false));
        b.put_move(&Move::new(&Piece::WSilver, 109, 8, false));
        b.put_move(&Move::new(&Piece::BKing, 109, 12, false));
        println!("{}", b);

        assert!(is_checked(&b));

        let result: HashSet<Move> = qsearch_moves(&b).into_iter().collect();
        let expected: HashSet<Move> = vec![
            Move::new(&Piece::BKing, 12, 16, false),
            Move::new(&Piece::BKing, 12, 17, false),
            Move::new(&Piece::BKing, 12, 18, false),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
}
