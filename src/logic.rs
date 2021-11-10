use super::game::*;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct MoveGen {}

static PIECE_MOVES: Lazy<HashMap<u8, Vec<(i8, i8)>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for i in 2..=21 {
        let piece = Piece(i);

        let piece_of_black = if piece.turn() == Turn::Black {
            piece
        } else {
            piece.flip()
        };
        let mut v = match piece_of_black {
            Piece::B_KING => vec![
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ],
            Piece::B_GOLD => vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, 0)],
            Piece::B_SILVER => vec![(-1, -1), (-1, 0), (-1, 1), (1, -1), (1, 1)],
            Piece::B_BISHOP => vec![(-1, -1), (-1, 1), (1, -1), (1, 1)],
            Piece::B_ROOK => vec![(-1, 0), (0, -1), (0, 1), (1, 0)],
            Piece::B_PAWN => vec![(-1, 0)],
            _ => vec![],
        };

        if piece.turn() == Turn::White {
            v = v.iter().map(|l| (l.0 * -1, l.1)).collect();
        }

        map.insert(piece.0, v);
    }
    map
});

impl MoveGen {
    pub fn valid_moves(board: &Board) -> Vec<Move> {
        let next_turn = board.turn;
        let mut moves: Vec<Move> = Vec::new();
        let mut king_found = 0;

        for (pos, piece) in board.squares.iter().enumerate() {
            if *piece == Piece::B_KING || *piece == Piece::W_KING {
                king_found += 1;
            }
            if *piece == Piece::ABSENT || piece.turn() != next_turn {
                continue;
            }
            for m in MoveGen::move_candidates(&piece, pos) {
                let dst_piece = board.at(m.dst as usize);
                if dst_piece.is_absent() || dst_piece.turn() != next_turn {
                    moves.push(m)
                }
            }
        }

        if king_found != 2 {
            return vec![]; // game is finished.
        }
        moves
    }

    fn move_candidates(piece: &Piece, pos: usize) -> Vec<Move> {
        let y = (pos / 5) as i8;
        let x = (pos % 5) as i8;
        let mut v: Vec<Move> = Vec::new();
        for (dy, dx) in &PIECE_MOVES[&piece.0] {
            if y + dy >= 0 && y + dy <= 4 && x + dx >= 0 && x + dx <= 4 {
                v.push(Move::new(
                    piece,
                    (y * 5 + x) as u8,
                    ((y + dy) * 5 + x + dx) as u8,
                    false,
                ))
            }
        }
        v
    }
}

pub struct Evaluator {}

impl Evaluator {
    const PIECE_VALUE: [i32; 22] = [
        0, 0, 10000, -10000, 567, -567, 528, -528, 951, -951, 1087, -1087, 93, -93, 582, -582,
        1101, -1101, 1550, -1550, 598, -598,
    ];
    pub fn evaluate(b: &Board) -> i32 {
        let mut sum = 0;
        for p in b.squares {
            sum += Evaluator::PIECE_VALUE[p.0 as usize];
        }
        return sum * (b.turn.val() as i32);
    }
}

pub struct Searcher {}

struct SearchResult {
    m: Move,
    score: i32,
    searched: i32,
}

impl Searcher {
    const MAX_SCORE: i32 = 1000000;

    pub fn find_best_move(b: &Board) -> Move {
        let result = Searcher::rec_search(b, 5);
        println!("Evaluated {} boards", result.searched);
        result.m
    }

    fn rec_search(b: &Board, depth: u8) -> SearchResult {
        let mut moves = MoveGen::valid_moves(b);

        if moves.is_empty() {
            let is_finished = b.is_finished();
            if is_finished.0 {
                return SearchResult {
                    m: Move::new(&Piece::ABSENT, 0, 0, false),
                    score: if is_finished.1 == b.turn {
                        Searcher::MAX_SCORE // win
                    } else {
                        Searcher::MAX_SCORE * -1 // lose
                    },
                    searched: 1,
                };
            } else {
                // no hand
                return SearchResult {
                    m: Move::new(&Piece::ABSENT, 0, 0, false),
                    score: Searcher::MAX_SCORE * -1,
                    searched: 1,
                };
            }
        }

        moves.shuffle(&mut rand::thread_rng());
        let mut best: SearchResult = SearchResult {
            m: moves[0].clone(),
            score: Searcher::MAX_SCORE * -1,
            searched: 0,
        };

        for m in moves {
            let mut next_board = b.clone();
            next_board.put_move(&m);

            match depth {
                0 => {
                    let score = Evaluator::evaluate(&next_board) * -1;
                    if score > best.score {
                        best.m = m;
                        best.score = score;
                        best.searched += 1;
                    }
                }
                _ => {
                    let result = Searcher::rec_search(&next_board, depth - 1);
                    best.searched += result.searched;

                    let score = result.score * -1;
                    if score > best.score {
                        best.m = m;
                        best.score = score;
                    }
                }
            }
        }
        best
    }
}
