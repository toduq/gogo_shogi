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
    const SCORE_LIMIT: i32 = 100_000_000;

    pub fn find_best_move(b: &Board) -> Option<SearchResult> {
        let result = Searcher::rec_search(b, 4);
        if result.m == Searcher::invalid_move() {
            None
        } else {
            Some(result)
        }
    }

    // if rec_search finds `score > beta`, the result will be discarded by alpha-beta.
    fn rec_search(b: &Board, depth: u8) -> SearchResult {
        if depth == 0 {
            let score = Evaluator::evaluate(&b) * -1;
            return SearchResult {
                m: Searcher::invalid_move(),
                score: score,
                searched: 1,
            };
        }

        let mut moves = MoveGen::valid_moves(b);
        if moves.is_empty() {
            return Searcher::evaluate_game_end(b, depth);
        }

        moves.shuffle(&mut rand::thread_rng());
        let mut best: SearchResult = SearchResult {
            m: Searcher::invalid_move(),
            score: Searcher::SCORE_LIMIT * -1,
            searched: 0,
        };

        let mut next_board = b.clone();
        for m in moves {
            next_board.copy_from(b);
            next_board.put_move(&m);

            let result = Searcher::rec_search(&next_board, depth - 1);
            let score = result.score * -1;
            if score > best.score {
                best.m = m;
                best.score = score;
            }
            best.searched += result.searched;
        }
        best
    }

    fn evaluate_game_end(b: &Board, depth: u8) -> SearchResult {
        let is_finished = b.is_finished();
        if !is_finished.0 {
            panic!("No move available");
        } else if is_finished.1 == b.turn {
            panic!("Turn started without opponent king. {}", b,);
        }
        SearchResult {
            m: Searcher::invalid_move(),
            // shorter checkmate path should be treated as better.
            score: Searcher::SCORE_LIMIT * -1 - (depth as i32) + 100,
            searched: 1,
        }
    }

    fn invalid_move() -> Move {
        Move::new(&Piece::ABSENT, 0, 0, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_king_immediately() {
        let mut b = Board::init();
        b.put_move(&Move::new(&Piece::B_KING, 20, 14, false));
        println!("{}", b);

        let result = Searcher::find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::W_PAWN, 9, 14, false));
    }

    #[test]
    fn takes_king_immediately_even_our_when_king_will_taken_next_turn() {
        let mut b = Board::init();
        b.put_move(&Move::new(&Piece::B_KING, 20, 14, false));
        b.put_move(&Move::new(&Piece::B_KING, 14, 9, false));
        println!("{}", b);

        let result = Searcher::find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::B_KING, 9, 4, false));
    }

    #[test]
    fn avoid_checkmate() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::B_KING, 100, 24, false));
        b.put_move(&Move::new(&Piece::W_KING, 100, 1, false));
        b.put_move(&Move::new(&Piece::B_GOLD, 100, 11, false));
        b.put_move(&Move::new(&Piece::B_GOLD, 100, 7, false));
        b.put_move(&Move::new(&Piece::B_GOLD, 7, 8, false));
        println!("{}", b);

        let result = Searcher::find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::W_KING, 1, 0, false));
    }

    #[test]
    fn avoid_checkmate_by_taking() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::B_KING, 100, 24, false));
        b.put_move(&Move::new(&Piece::W_KING, 100, 1, false));
        b.put_move(&Move::new(&Piece::W_KING, 1, 0, false));
        b.put_move(&Move::new(&Piece::B_ROOK, 100, 4, false));
        b.put_move(&Move::new(&Piece::B_ROOK, 100, 9, false));
        b.put_move(&Move::new(&Piece::B_SILVER, 100, 5, false));
        b.put_move(&Move::new(&Piece::B_SILVER, 100, 6, false));
        println!("{}", b);

        let result = Searcher::find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::W_KING, 0, 5, false));
    }
}
