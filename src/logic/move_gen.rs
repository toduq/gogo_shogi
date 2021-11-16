use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn all_valid_moves(board: &Board) -> Vec<Move> {
    valid_moves(board, true)
}

pub fn moves_only(board: &Board) -> Vec<Move> {
    valid_moves(board, false)
}

// pub fn moves_of_check(board: &Board) -> Vec<Move> {
//     let mut next_board = board.clone();
//     valid_moves(board, true)
//         .iter()
//         .filter(|m| {
//             next_board.copy_from(board);
//             next_board.put_move(m);
//             next_board.flip_turn();
//             is_check_or_win(&next_board)
//         })
//         .map(|m| *m)
//         .collect()
// }

fn valid_moves(board: &Board, include_hands: bool) -> Vec<Move> {
    if is_king_absent(board) {
        return vec![]; // game is finished
    }

    let my_turn = board.turn;
    let mut moves = Vec::new();

    // move piece
    for (pos, piece) in board.squares.iter().enumerate() {
        if *piece == Piece::Absent || piece.turn() != my_turn {
            continue;
        }
        for ms in &PIECE_MOVES_WITH_POSITION[&(*piece as u8, pos)] {
            for m in ms {
                let dst_piece = board.at(m.dst as usize);
                if dst_piece.is_absent() {
                    // empty. can go through
                    moves.push(*m);
                } else if dst_piece.turn() != my_turn {
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

    if include_hands {
        // piece from hands
        for (pos, piece) in board.hands.iter().enumerate() {
            if *piece == Piece::Absent || piece.turn() != my_turn {
                continue;
            }
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

fn is_king_absent(b: &Board) -> bool {
    let king_count = b
        .squares
        .iter()
        .map(|p| p.of_turn(Turn::Black))
        .filter(|p| *p == Piece::BKing)
        .count();
    king_count != 2
}

// pub fn is_check_or_win(b: &Board) -> bool {
//     let opp_king = Piece::BKing.of_turn(b.turn.next());
//     let opp_king_pos = b.squares.iter().find(|p| **p == opp_king);
//     if opp_king_pos.is_none() {
//         return true; // wins
//     }
//     let opp_king_pos = opp_king_pos.unwrap();
//     valid_moves(b, false)
//         .iter()
//         .any(|m| m.dst == opp_king_pos.as_u8())
// }

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
                                vec![
                                    Move::new(&p, src, dst, true),
                                    Move::new(&p, src, dst, false),
                                ]
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
