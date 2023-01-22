use rustoku::medium::*;
use rustoku::{OutputString, Technique};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let str = "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";

    // Find the brute force solution.  Not needed for human solving, but will verify that solutions match
    let puz = Sudoku::new(str)?;

    let (human, moves) = puz.human_solve()?;

    // This is the brute force solution.
    let solution = puz.solution().get()?;

    assert_eq!(human, solution);
    println!("--Human--\n{:?}\n", human.output_string('.', None));

    let is_a_fish = |a_move: &Move| {
        if let Some(tech) = a_move.technique() {
            match tech {
                Technique::FishN(_)
                | Technique::Jellyfish
                | Technique::Swordfish
                | Technique::XWing
                | Technique::FinnedN(_)
                | Technique::FinnedJellyfish
                | Technique::FinnedSwordfish
                | Technique::FinnedXWing => true,
                _ => false,
            }
        } else {
            false
        }
    };

    let count = moves.into_iter().filter(is_a_fish).count();
    println!(
        "There were {} fish techniques used to solve this puzzle",
        count
    );

    Ok(())
}
