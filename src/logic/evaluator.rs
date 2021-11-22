use super::*;

pub const PIECE_VALUE: [i32; 22] = [
    0, 0, 5000, -5000, 567, -567, 528, -528, 951, -951, 1087, -1087, 93, -93, 582, -582, 1101,
    -1101, 1550, -1550, 598, -598,
];

pub fn evaluate(b: &Board) -> i32 {
    let mut sum = 0;
    for p in b.squares {
        sum += evaluator::PIECE_VALUE[p as usize];
    }
    for p in b.hands {
        sum += evaluator::PIECE_VALUE[p as usize] * 9 / 10;
    }

    let mut distance = 0;
    let opp_king_pos = b
        .squares
        .iter()
        .position(|p| *p == Piece::BKing.of_turn(b.turn))
        .unwrap() as i32;
    for (i, p) in b.squares.iter().enumerate() {
        if p.turn() == b.turn {
            distance += std::cmp::max(
                (opp_king_pos / 5 - i as i32 / 5).abs(),
                (opp_king_pos % 5 - i as i32 % 5).abs(),
            )
        }
    }
    sum * (b.turn.val() as i32) - distance
}
