use rustoku::basic::*;
use rustoku::OutputString;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let puz = "...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9...";
    let puzzle = Sudoku::new(puz)?;
    println!("--Original--\n{}", puzzle.output_string('.', None));
    if let Solution::One(grid) = puzzle.solution() {
        println!("--Brute--\n{}", grid.output_string('.', None));
    }

    Ok(())
}
