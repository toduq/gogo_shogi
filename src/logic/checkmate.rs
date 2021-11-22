use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum MateResult {
    Unknown,
    Win,
    Lose,
}

pub fn is_checkmate(b: &Board) -> MateResult {
    if move_gen::is_checked(b) && move_gen::evasion_moves(b).is_empty() {
        return MateResult::Lose;
    }
    let mut next_board = b.clone();
    for m in move_gen::check_moves(b) {
        next_board.copy_from(b);
        next_board.put_move(&m);
        if move_gen::is_checked(&next_board) && move_gen::evasion_moves(&next_board).is_empty() {
            return MateResult::Win;
        }
    }
    MateResult::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn king_is_in_front_of_gold() {
        let mut b = board_gen::from_str("31wk,32bg,33bp,55bk");
        assert_eq!(is_checkmate(b.flip_turn()), MateResult::Lose);
    }

    #[test]
    fn king_is_in_front_of_silver() {
        let mut b = board_gen::from_str("31wk,32bs,33bp,55bk");
        assert_eq!(is_checkmate(b.flip_turn()), MateResult::Unknown);
    }
}
