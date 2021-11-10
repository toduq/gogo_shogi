mod game;
mod logic;

use game::*;
use logic::*;
use std::fs::File;

fn main() {
    let guard = pprof::ProfilerGuard::new(100).unwrap();
    println!("Board size : {}", std::mem::size_of::<Board>());
    let mut b = Board::init();
    println!("{}", b);

    for _ in 0..100 {
        let best_move = Searcher::find_best_move(&b);
        b.put_move(&best_move);
        println!("Selected move : {:?}", best_move);
        println!("Evaluation : {:?}", Evaluator::evaluate(&b));
        println!("{}", b);
    }
    println!("Game finished");
    if let Ok(report) = guard.report().build() {
        let file = File::create("flamegraph.svg").unwrap();
        report.flamegraph(file).unwrap();
    };
}
