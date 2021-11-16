use super::*;
use std::collections::HashSet;

const PIECE_VALUE: [i32; 22] = [
    0, 0, 100000, -100000, 567, -567, 528, -528, 951, -951, 1087, -1087, 93, -93, 582, -582, 1101,
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
    let move_dst: HashSet<u8> = move_gen::moves_only(b).iter().map(|m| m.src).collect();
    sum += move_dst.len() as i32;
    sum * (b.turn.val() as i32)
}
