use super::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct SearchResult {
    pub m: Move,
    pub score: i32,
    pub searched: i32,
}

const SEARCH_DEPTH: u8 = 2;
const QSEARCH_EPTH: u8 = 4;
const SCORE_LIMIT: i32 = 100_000_000;
const WIN_THRESH: i32 = 90_000_000;

pub fn find_best_move(b: &Board) -> Option<SearchResult> {
    let mut last_result = search_result_of(0, 0);
    for depth in 1..=SEARCH_DEPTH {
        last_result = rec_search(b, 0, depth, -SCORE_LIMIT, SCORE_LIMIT);
        if last_result.m == invalid_move() {
            return None;
        } else if last_result.score > WIN_THRESH {
            return Some(last_result);
        }
    }
    if last_result.m == invalid_move() {
        None
    } else {
        Some(last_result)
    }
}

// if rec_search finds `score > beta`, the result will be discarded by alpha-beta.
fn rec_search(b: &Board, depth: u8, max_depth: u8, alpha: i32, beta: i32) -> SearchResult {
    if b.won.is_some() {
        // always lose on this case.
        return search_result_of(-SCORE_LIMIT + (depth as i32), 1);
    }
    if depth >= max_depth {
        // do q search
        return q_rec_search(b, depth, depth + QSEARCH_EPTH, evaluator::evaluate(b), beta);
    }
    let moves = move_gen::all_valid_moves(b);

    let moves = reorder_moves(b, &moves);
    let mut best: SearchResult = search_result_of(alpha, 0);

    let mut next_board = b.clone();
    for m in moves {
        next_board.copy_from(b);
        next_board.put_move(&m);

        let result = rec_search(&next_board, depth + 1, max_depth, -beta, -best.score);
        let score = -result.score;
        // println!(
        //     "{}[{}]got {} by {:?}",
        //     " ".repeat(depth.into()),
        //     depth,
        //     score,
        //     m
        // );
        if score > best.score {
            best.m = m;
            best.score = score;
        }
        if score > beta || score > WIN_THRESH {
            // println!(
            //     "{}[{}]beta cut by {}",
            //     " ".repeat(depth.into()),
            //     depth,
            //     score
            // );
            return best;
        }
        best.searched += result.searched;
    }

    update_move_order_table(&best.m);
    best
}

fn q_rec_search(b: &Board, depth: u8, max_depth: u8, alpha: i32, beta: i32) -> SearchResult {
    if b.won.is_some() {
        // always lose on this case.
        return search_result_of(-SCORE_LIMIT + (depth as i32), 1);
    }
    if depth >= max_depth {
        // evaluate because max_depth
        return search_result_of(evaluator::evaluate(b), 1);
    }
    let moves = move_gen::qsearch_moves(b);
    if moves.is_empty() {
        // evaluate because its quiescence
        return search_result_of(evaluator::evaluate(b), 1);
    }

    let moves = reorder_moves(b, &moves);
    let mut best: SearchResult = search_result_of(alpha, 0);

    let mut next_board = b.clone();
    for m in moves {
        next_board.copy_from(b);
        next_board.put_move(&m);

        let result = q_rec_search(&next_board, depth + 1, max_depth, -beta, -best.score);
        let score = -result.score;

        // println!(
        //     "{}[{}]got {} by {:?}",
        //     " ".repeat(depth.into()),
        //     depth,
        //     score,
        //     m
        // );
        if score > best.score {
            best.m = m;
            best.score = score;
        }
        if score > beta || score > WIN_THRESH {
            // println!(
            //     "{}[{}]beta cut by {}",
            //     " ".repeat(depth.into()),
            //     depth,
            //     score
            // );
            return best;
        }
        best.searched += result.searched;
    }
    best
}

fn update_move_order_table(m: &Move) {
    let mut table = MOVE_ORDER_TABLE.lock().unwrap();
    let value = table.entry(MoveOrderingKey(m.piece, m.dst)).or_insert(0);
    *value += 1;
    if *value > MOVE_ORDER_TABLE_MAX {
        for (_, val) in table.iter_mut() {
            *val /= 2;
        }
    }
}

// re-order moves for alpha-beta cut
fn reorder_moves(b: &Board, moves: &[Move]) -> Vec<Move> {
    let mut power_of_moves: [u8; 25] = [0; 25];
    for m in moves {
        if m.src < 100 {
            power_of_moves[m.dst as usize] += 1;
        }
    }
    let table = MOVE_ORDER_TABLE.lock().unwrap();
    let mut tupls: Vec<(Move, i32)> = moves
        .iter()
        .map(|m| (*m, move_priority(b, m, &power_of_moves, &table)))
        .collect();
    tupls.sort_by(|a, b| b.1.cmp(&a.1));
    tupls.iter().map(|t| t.0).collect()
}

fn move_priority(
    b: &Board,
    m: &Move,
    power_of_moves: &[u8; 25],
    statistics: &HashMap<MoveOrderingKey, MoveOrderingValue>,
) -> i32 {
    let mut priority = 0;
    let scale = 100;

    // piece taking move
    let dst_piece = b.at(m.dst as usize);
    if dst_piece != Piece::Absent {
        priority += (5000 + evaluator::PIECE_VALUE[dst_piece as usize].abs()) * scale;
    }

    // moving precedes putting
    if m.src < 100 {
        priority += 100 * scale;
    } else if power_of_moves[m.dst as usize] == 0 {
        // free piece is bad.
        priority -= 100 * scale;
    }

    // strong piece is better (< 5000)
    priority += evaluator::PIECE_VALUE[m.piece as usize].abs();

    // near enemy's king is better (< 4000)
    let enemy_king_pos = b
        .squares
        .iter()
        .enumerate()
        .find(|p| *p.1 == Piece::BKing.of_turn(b.turn.next()))
        .unwrap()
        .0;
    priority += (4 - ((enemy_king_pos as i32 / 5) - (m.dst as i32 / 5)).abs()) * 500;
    priority += (4 - ((enemy_king_pos as i32 % 5) - (m.dst as i32 % 5)).abs()) * 500;

    // order by statistics (< 1000)
    priority += *statistics
        .get(&MoveOrderingKey(m.piece, m.dst))
        .unwrap_or(&0);

    // randomize order (< 3000)
    priority += (rand::thread_rng().next_u32() / 10) as i32 % 3000;

    priority
}

fn invalid_move() -> Move {
    Move::new(&Piece::Absent, 0, 0, false)
}

fn search_result_of(score: i32, searched: i32) -> SearchResult {
    SearchResult {
        m: invalid_move(),
        score,
        searched,
    }
}

#[derive(Hash, PartialEq, Eq)]
struct MoveOrderingKey(Piece, u8);
type MoveOrderingValue = i32;
const MOVE_ORDER_TABLE_MAX: i32 = 1000;
static MOVE_ORDER_TABLE: Lazy<Mutex<HashMap<MoveOrderingKey, MoveOrderingValue>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

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
    fn reorder_moves_test() {
        let mut b = Board::init();
        b.put_move(&Move::new(&Piece::BBishop, 23, 12, false));
        b.flip_turn();

        let moves = vec![
            Move::new(&Piece::BBishop, 14, 8, false), // nothing
            Move::new(&Piece::BBishop, 14, 0, true),  // take piece with promote
            Move::new(&Piece::BBishop, 14, 4, true),  // win with promote
        ];

        println!("{}", b);
        let reordered_moves = reorder_moves(&b, &moves);
        assert_eq!(reordered_moves, vec![moves[2], moves[1], moves[0]]);
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
    //     b.put_move(&Move::new(&Piece::WGold, 109, 9, false));
    //     b.put_move(&Move::new(&Piece::WSilver, 109, 8, false));
    //     // put pieces
    //     b.put_move(&Move::new(&Piece::WRook, 100, 0, false));
    //     b.put_move(&Move::new(&Piece::WKing, 100, 1, false));
    //     b.put_move(&Move::new(&Piece::BPawnP, 100, 10, false));
    //     b.put_move(&Move::new(&Piece::BPawnP, 10, 9, false));
    //     b.put_move(&Move::new(&Piece::BPawnP, 9, 8, false));
    //     // dummy
    //     b.put_move(&Move::new(&Piece::BKing, 109, 24, false));
    //     println!("{}", b);

    //     let result = find_best_move(&b);
    //     println!("{:?}", result);
    //     assert_eq!(result.unwrap().m, Move::new(&Piece::BSilver, 101, 7, false));
    // }
}
