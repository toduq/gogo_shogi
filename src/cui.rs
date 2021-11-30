use super::*;
use std::io;
use std::io::Write;

pub fn user_input(b: &Board) -> Move {
    let valid_moves = move_gen::all_valid_moves(b);

    loop {
        print!("Please input your move [3332/3231p/g32] : ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input_move = parse(b, input);
        let is_valid = valid_moves.iter().any(|m| *m == input_move);
        if is_valid {
            return input_move;
        } else {
            println!("Invalid move {:?}", input_move);
        }
    }
}

fn parse(b: &Board, s: String) -> Move {
    let bytes = s.into_bytes();
    match bytes[0] {
        b'1'..=b'5' => {
            let src = to_pos(bytes[0], bytes[1]);
            let dst = to_pos(bytes[2], bytes[3]);
            Move::new(
                &b.squares[src as usize],
                src,
                dst,
                bytes.len() >= 5 && bytes[4] == b'p',
            )
        }
        _ => {
            let dst = to_pos(bytes[1], bytes[2]);
            let piece = board_gen::piece_repr(bytes[0]).of_turn(b.turn);
            let src = b.hands.iter().position(|p| *p == piece).unwrap() + 100;
            Move::new(&piece, src as u8, dst, false)
        }
    }
}

fn to_pos(x: u8, y: u8) -> u8 {
    (y - b'1') * 5 + (4 - x + b'1')
}
