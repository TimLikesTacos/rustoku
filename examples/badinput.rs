use rustoku::basic::Sudoku;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Invalid input string: ");
    let puz = "...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9..";
    if let Err(e) = Sudoku::new(puz) {
        println!("{}", e);
    }

    println!("\n\nInvalid puzzle: ");
    let puz = ".78..7....16.83.7.....2.8511372........73........463.729..7.......86.14....3..7..";
    // Notice that this part returns an error due to incorrect puzzle due to conflicting inputs
    if let Err(e) = Sudoku::new(puz) {
        println!("{}", e);
    }

    println!("\n\nMultiple solutions: ");
    let puz = ".28..7....16.83.7.....2.8511372........73........463.729..7.......86.14....3..7..";
    // Notice that this part does not return an error.  `.new()` only checks for input parsing and length. `.validate()` checks for a valid puzzle.
    let sud = Sudoku::new(puz)?;
    if let Err(e) = sud.unique_solution() {
        println!("{}", e);
    }
    println!("Ok");
    Ok(())
}
