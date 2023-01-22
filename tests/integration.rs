extern crate rayon;
extern crate rustoku;
use crate::rustoku::basic::{Solution, Sudoku};
mod setup;
use rayon::prelude::*;

#[test]
fn test_one_file() {
    let strings = setup::one_known_file();

    strings.par_chunks(1000).flatten().for_each(|t| {
        let slice = &t[0..];
        let puz = Sudoku::new(slice).unwrap();
        let brute_sol = puz.solution();
        let (human, _moves) = puz.human_solve().unwrap();

        if let Solution::One(brute) = brute_sol {
            assert_eq!(brute, &human)
        }
    });
    println!("{} puzzles tested correctly", strings.len())
}
