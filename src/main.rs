mod game;
mod logic;

use game::*;
use logic::*;
use std::time::SystemTime;

fn main() {
    let mut b = Board::init();
    println!("{}", b);

    let mut evaluated = 0;
    let start = SystemTime::now();
    for i in 0..1000 {
        let best_move = Searcher::find_best_move(&b);
        if let None = best_move {
            println!("Game has finished in {} moves", i);
            break;
        }
        let best_move = best_move.unwrap();

        b.put_move(&best_move.m);
        println!("Selected move : {:?}", best_move.m);
        println!(
            "Evaluated {} boards, Evaluation {}",
            best_move.searched, best_move.score
        );
        evaluated += best_move.searched;
        println!("{}", b);
        println!("==========================");

        if i == 299 {
            println!("Abort. Too long game.");
            break;
        }
    }

    let ms = start.elapsed().unwrap().as_millis();
    println!(
        "Evaluated {} boards in {} ms. ({} boards/sec)",
        evaluated,
        ms,
        (evaluated as u128) * 1000 / ms
    );
}
