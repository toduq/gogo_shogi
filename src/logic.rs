mod move_gen;

pub use self::move_gen::MoveGen;

use super::game::*;
use rand::seq::SliceRandom;

pub struct Evaluator {}

impl Evaluator {
    const PIECE_VALUE: [i32; 22] = [
        0, 0, 100000, -100000, 567, -567, 528, -528, 951, -951, 1087, -1087, 93, -93, 582, -582,
        1101, -1101, 1550, -1550, 598, -598,
    ];
    pub fn evaluate(b: &Board) -> i32 {
        let mut sum = 0;
        for p in b.squares {
            sum += Evaluator::PIECE_VALUE[p.0 as usize];
        }
        for p in b.hands {
            sum += Evaluator::PIECE_VALUE[p.0 as usize] * 9 / 10;
        }
        return sum * (b.turn.val() as i32);
    }
}

pub struct Searcher {}

pub struct SearchResult {
    pub m: Move,
    pub score: i32,
    pub searched: i32,
}

impl Searcher {
    const MAX_SCORE: i32 = 1000000;

    pub fn find_best_move(b: &Board) -> Option<SearchResult> {
        let result = Searcher::rec_search(b, 3);
        if result.m == Searcher::invalid_move() {
            None
        } else {
            Some(result)
        }
    }

    fn rec_search(b: &Board, depth: u8) -> SearchResult {
        let mut moves = MoveGen::valid_moves(b);
        if moves.is_empty() {
            return Searcher::evaluate_game_end(b);
        }

        moves.shuffle(&mut rand::thread_rng());
        let mut best: SearchResult = SearchResult {
            m: Searcher::invalid_move(),
            score: Searcher::MAX_SCORE * -2,
            searched: 0,
        };

        let mut next_board = b.clone();
        for m in moves {
            next_board.copy_from(b);
            next_board.put_move(&m);

            match depth {
                0 => {
                    let score = Evaluator::evaluate(&next_board) * -1;
                    best.searched += 1;

                    if score > best.score {
                        best.m = m;
                        best.score = score;
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

    fn evaluate_game_end(b: &Board) -> SearchResult {
        let is_finished = b.is_finished();
        if !is_finished.0 {
            panic!("No move available");
        } else if is_finished.1 == b.turn {
            panic!("Turn started without opponent king. {}", b,);
        }
        SearchResult {
            m: Searcher::invalid_move(),
            score: Searcher::MAX_SCORE * -1,
            searched: 1,
        }
    }

    fn invalid_move() -> Move {
        Move::new(&Piece::ABSENT, 0, 0, false)
    }
}
