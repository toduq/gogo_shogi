use super::*;

pub struct SearchResult {
    pub m: Move,
    pub score: i32,
    pub searched: i32,
}

const SEARCH_DEPTH: u8 = 4;
const SCORE_LIMIT: i32 = 100_000_000;

pub fn find_best_move(b: &Board) -> Option<SearchResult> {
    let result = rec_search(b, SEARCH_DEPTH, -SCORE_LIMIT, SCORE_LIMIT);
    if result.m == invalid_move() {
        None
    } else {
        Some(result)
    }
}

// if rec_search finds `score > beta`, the result will be discarded by alpha-beta.
fn rec_search(b: &Board, depth: u8, alpha: i32, beta: i32) -> SearchResult {
    let moves = move_gen::all_valid_moves(b);
    if moves.is_empty() {
        return evaluate_game_end(b, depth);
    }

    if depth == 0 {
        return SearchResult {
            m: invalid_move(),
            score: evaluator::evaluate(b),
            searched: 1,
        };
    }

    let moves = reorder_moves(b, &moves);

    let mut best: SearchResult = SearchResult {
        m: invalid_move(),
        score: alpha,
        searched: 0,
    };

    let mut next_board = b.clone();
    for m in moves {
        next_board.copy_from(b);
        next_board.put_move(&m);

        let result = rec_search(&next_board, depth - 1, -beta, -best.score);
        let score = -result.score;
        if score > best.score {
            best.m = m;
            best.score = score;
        }
        if score > beta {
            return best;
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
        m: invalid_move(),
        // shorter checkmate path should be treated as better.
        score: -SCORE_LIMIT - (depth as i32) + 100,
        searched: 1,
    }
}

// re-order moves for alpha-beta cut
fn reorder_moves(b: &Board, moves: &[Move]) -> Vec<Move> {
    let mut tupls: Vec<(Move, i32)> = moves.iter().map(|m| (*m, move_priority(b, m))).collect();
    tupls.sort_by(|a, b| b.1.cmp(&a.1));
    tupls.iter().map(|t| t.0).collect()
}

fn move_priority(b: &Board, m: &Move) -> i32 {
    let mut priority = 0;
    let dst_piece = b.at(m.dst as usize);
    if dst_piece == Piece::BKing || dst_piece == Piece::WKing {
        priority += 10000;
    } else if dst_piece != Piece::Absent {
        priority += 1000;
    }
    if m.promote {
        priority += 100;
    }
    priority += (rand::thread_rng().next_u32() / 10) as i32 % 10;
    priority
}

fn invalid_move() -> Move {
    Move::new(&Piece::Absent, 0, 0, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_king_immediately() {
        let mut b = Board::init();
        b.put_move(&Move::new(&Piece::BKing, 20, 14, false));

        println!("{}", b);
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::WPawn, 9, 14, false));
    }

    #[test]
    fn takes_king_immediately_even_our_when_king_will_taken_next_turn() {
        let mut b = Board::init();
        b.put_move(&Move::new(&Piece::BKing, 20, 9, false));
        b.flip_turn();

        println!("{}", b);
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::BKing, 9, 4, false));
    }

    #[test]
    fn avoid_checkmate() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::WKing, 100, 1, false));
        b.put_move(&Move::new(&Piece::BGold, 100, 11, false));
        b.put_move(&Move::new(&Piece::BGold, 100, 8, false));
        // dummy
        b.put_move(&Move::new(&Piece::BKing, 100, 24, false));
        b.flip_turn();

        println!("{}", b);
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::WKing, 1, 0, false));
    }

    #[test]
    fn avoid_checkmate_by_taking() {
        let mut b = Board::empty();
        b.put_move(&Move::new(&Piece::WKing, 100, 0, false));
        b.put_move(&Move::new(&Piece::BRook, 100, 4, false));
        b.put_move(&Move::new(&Piece::BRook, 100, 9, false));
        b.put_move(&Move::new(&Piece::BSilver, 100, 5, false));
        b.put_move(&Move::new(&Piece::BSilver, 100, 6, false));
        // dummy
        b.put_move(&Move::new(&Piece::BKing, 100, 24, false));
        b.flip_turn();

        println!("{}", b);
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::WKing, 0, 5, false));
    }

    #[test]
    fn checkmate_with_1_moves() {
        // https://www.aonoshogi.com/1tetsume/000/006.php
        let mut b = Board::empty();
        // for hands
        b.put_move(&Move::new(&Piece::WSilver, 109, 11, false));
        // put pieces
        b.put_move(&Move::new(&Piece::WKing, 109, 1, false));
        b.put_move(&Move::new(&Piece::BGold, 100, 12, false));
        b.put_move(&Move::new(&Piece::BGold, 12, 11, false));
        // dummy
        b.put_move(&Move::new(&Piece::BKing, 109, 24, false));
        b.flip_turn();
        println!("{}", b);

        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::BSilver, 100, 6, false),);
    }

    // #[test]
    // fn checkmate_with_3_moves() {
    //     // https://www.aonoshogi.com/3tetsume/000/001.php
    //     let mut b = Board::empty();
    //     // for hands
    //     b.put_move(&Move::new(&Piece::W_GOLD, 109, 9, false));
    //     b.put_move(&Move::new(&Piece::W_SILVER, 109, 8, false));
    //     // put pieces
    //     b.put_move(&Move::new(&Piece::W_ROOK, 100, 0, false));
    //     b.put_move(&Move::new(&Piece::W_KING, 100, 1, false));
    //     b.put_move(&Move::new(&Piece::B_PAWN_P, 100, 10, false));
    //     b.put_move(&Move::new(&Piece::B_PAWN_P, 10, 9, false));
    //     b.put_move(&Move::new(&Piece::B_PAWN_P, 9, 8, false));
    //     // dummy
    //     b.put_move(&Move::new(&Piece::B_KING, 109, 24, false));
    //     println!("{}", b);

    //     let result = find_best_move(&b);
    //     assert_eq!(
    //         result.unwrap().m,
    //         Move::new(&Piece::B_SILVER, 101, 7, false)
    //     );
    // }
}
