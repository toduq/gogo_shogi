use super::checkmate::MateResult;
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

const SEARCH_DEPTH: u8 = 3;
const QSEARCH_EPTH: u8 = 3;
const SCORE_LIMIT: i32 = 100_000_000;
const WIN_THRESH: i32 = 90_000_000;

pub fn find_best_move(b: &Board) -> Option<SearchResult> {
    let mut last_result = search_result_of(0, 0);
    for depth in 1..=SEARCH_DEPTH {
        last_result = rec_search(b, 0, depth, -SCORE_LIMIT, SCORE_LIMIT, false);
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
fn rec_search(
    b: &Board,
    depth: u8,
    max_depth: u8,
    alpha: i32,
    beta: i32,
    need_qsearch: bool,
) -> SearchResult {
    if b.won.is_some() {
        // always lose on this case.
        return search_result_of(-SCORE_LIMIT + (depth as i32), 1);
    }
    if depth >= max_depth {
        // do q search
        let qsearch_depth = if need_qsearch {
            depth + QSEARCH_EPTH
        } else {
            depth
        };
        return q_rec_search(b, depth, qsearch_depth, evaluator::evaluate(b), beta);
    }
    let moves = move_gen::all_valid_moves(b);

    let moves = reorder_moves(b, &moves);
    let mut best: SearchResult = search_result_of(alpha, 0);

    let mut next_board = b.clone();
    for m in moves {
        next_board.copy_from(b);
        next_board.put_move(&m);

        let is_taking_move = !b.squares[m.dst as usize].is_absent();
        let result = rec_search(
            &next_board,
            depth + 1,
            max_depth,
            -beta,
            -best.score,
            is_taking_move,
        );
        let score = -result.score;
        best.searched += result.searched;
        if score > best.score {
            best.m = m;
            best.score = score;
        }
        if score > beta || score > WIN_THRESH {
            return best;
        }
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
        return evaluate_leaf(b, depth);
    }
    let moves = move_gen::taking_moves(b);
    if moves.is_empty() {
        // evaluate because its quiescence
        return evaluate_leaf(b, depth);
    }

    let moves = reorder_moves(b, &moves);
    let mut best: SearchResult = search_result_of(alpha, 0);

    let mut next_board = b.clone();
    for m in moves {
        next_board.copy_from(b);
        next_board.put_move(&m);

        let result = q_rec_search(&next_board, depth + 1, max_depth, -beta, -best.score);
        let score = -result.score;
        best.searched += result.searched;
        if score > best.score {
            best.m = m;
            best.score = score;
        }
        if score > beta || score > WIN_THRESH {
            return best;
        }
    }
    best
}

fn evaluate_leaf(b: &Board, depth: u8) -> SearchResult {
    match checkmate::is_checkmate(b) {
        MateResult::Unknown => search_result_of(evaluator::evaluate(b), 1),
        MateResult::Win => search_result_of(SCORE_LIMIT - (depth as i32), 1),
        MateResult::Lose => search_result_of(-SCORE_LIMIT + (depth as i32), 1),
    }
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
        .position(|p| *p == Piece::BKing.of_turn(b.turn.next()))
        .unwrap();
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
        let mut b = gen("41wk,43bg,22bg,15bk");
        b.flip_turn();
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::WKing, 1, 0, false));
    }

    #[test]
    fn avoid_checkmate_by_taking() {
        let mut b = gen("51wk,11br,12br,52bs,42bs,15bk");
        b.flip_turn();
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::WKing, 0, 5, false));
    }

    #[test]
    fn checkmate_with_1_moves() {
        let b = gen("41wk,43bg,15bk,__bs");
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::BSilver, 100, 6, false),);
    }

    #[test]
    fn checkmate_with_3_moves() {
        // https://www.aonoshogi.com/3tetsume/000/002.php
        let b = gen("11wr,21wk,41bP,52bR,55bk,__bs,__wg,__ws");
        let result = find_best_move(&b);
        assert_eq!(result.unwrap().m, Move::new(&Piece::BPawnP, 1, 2, false));
    }

    fn gen(s: &str) -> Board {
        let b = board_gen::from_str(s);
        println!("{}", b);
        b
    }
}
