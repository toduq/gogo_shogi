use super::super::game::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct MoveGen {}

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
        let piece = Piece(i);
        let piece_of_black = piece.of_turn(Turn::Black);

        let mut v = match piece_of_black {
            Piece::B_KING => king.clone(),
            Piece::B_GOLD => gold.clone(),
            Piece::B_SILVER => silver.clone(),
            Piece::B_BISHOP => bishop.clone(),
            Piece::B_ROOK => rook.clone(),
            Piece::B_PAWN => pawn.clone(),
            Piece::B_SILVER_P => gold.clone(),
            Piece::B_BISHOP_P => bishop_p.clone(),
            Piece::B_ROOK_P => rook_p.clone(),
            Piece::B_PAWN_P => gold.clone(),
            _ => vec![],
        };

        if piece.turn() == Turn::White {
            v = v
                .iter()
                .map(|v2| v2.iter().map(|l| (l.0 * -1, l.1)).collect())
                .collect();
        }

        map.insert(piece.0, v);
    }
    map
});

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
                            let p = Piece(*piece);
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

impl MoveGen {
    pub fn valid_moves(board: &Board) -> Vec<Move> {
        let king_count = board
            .squares
            .iter()
            .map(|p| p.of_turn(Turn::Black))
            .filter(|p| *p == Piece::B_KING)
            .count();
        if king_count != 2 {
            return vec![]; // game is finished
        }

        let my_turn = board.turn;
        let mut moves = Vec::new();

        // move piece
        for (pos, piece) in board.squares.iter().enumerate() {
            if *piece == Piece::ABSENT || piece.turn() != my_turn {
                continue;
            }
            for ms in &PIECE_MOVES_WITH_POSITION[&(piece.0, pos)] {
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

        // piece from hands
        for (pos, piece) in board.hands.iter().enumerate() {
            if *piece == Piece::ABSENT || piece.turn() != my_turn {
                continue;
            }
            for dst in 0..25 {
                if board.at(dst) != Piece::ABSENT {
                    continue;
                }
                moves.push(Move::new(piece, (100 + pos) as u8, dst as u8, false))
            }
        }

        moves
    }
}
