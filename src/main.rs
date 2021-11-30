mod cui;
mod game;
mod logic;

use game::*;
use getopts::Options;
use logic::*;
use std::collections::HashMap;
use std::env;
use std::time::SystemTime;

fn main() {
    let mut opts = Options::new();
    opts.optflag("b", "black", "play black");
    opts.optflag("w", "white", "play white");
    let args: Vec<String> = env::args().collect();
    let opt = match opts.parse(&args[1..]) {
        Ok(m) => {
            let mut opt: HashMap<Turn, bool> = HashMap::new();
            opt.insert(Turn::Black, m.opt_present("b"));
            opt.insert(Turn::White, m.opt_present("w"));
            opt
        }
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let mut b = Board::init();
    println!("{}", b);

    let mut evaluated = 0;
    let start = SystemTime::now();
    for i in 0..1000 {
        if b.won.is_some() {
            println!("Game has finished in {} moves", i);
            break;
        }

        let selected_move = if *opt.get(&b.turn).unwrap() {
            cui::user_input(&b)
        } else {
            let best_move = searcher::find_best_move(&b).unwrap();
            println!(
                "Evaluated {} boards, Evaluation {}",
                best_move.searched, best_move.score
            );
            evaluated += best_move.searched;
            best_move.m
        };

        println!("Selected move : {:?}", selected_move);
        b.put_move(&selected_move);
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
