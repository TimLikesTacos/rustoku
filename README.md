# Rustoku

![Finned Jellyfish](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/finned_jelly.png?raw=true)  

We all like Sudoku and writing brute force solvers, but this library solves sudoku puzzles using human style techniques,
along with traditional brute force.  
The following are methods either implemented or planned.  

<ul style="color:green"> <b>Implemented</b>
<li > Brute Force
<li> Single Possibilities
<li> Single Candidates
<li> Naked Tuples</li>
<li> Hiddle Tuples</li>
<li> Claiming Candidates</li>
<li> Pointing Candidates</li>
<li> Basic Fish (X-wing, Swordfish...)</li>
<li> Finned Fish</li>
</ul>
<ul style="color:red"> <b>Not Yet Implemented</b>

<li> Sashimi Fish</li>
<li> Franken Fish</li>
<li> Mutant Fish</li>
</ul>
  
Ranking difficulty for software developed and solved sudokus usually falls into a calculation of the number of 
pre-filled squares compared the total number. This gives a rough estimate of difficulty, but there are many examples of 
certain puzzles that are easy to solve even with a low number of pre-filled squares.  Using human techniques allows
calculating a more accurate rating.

Brute force solvers also do not give you a hint to improve your solving skills.  This library
can take an unsolved puzzle that you may be struggling with, and give you a hint on a difficult
step, without necessarily giving the answer to the puzzle.


## This is a work in progress
Updates and refactoring are in progress including solving techniques, caching human techniques for improved
performance, and the library interface including things such as how moves are stored and difficulty obtained.
Solving techniques have not been testing on puzzles larger than 9x9.

### About this library
This library was primarily built to give myself experience developing in Rust, including different data structures
 to solve the unique algorithms for human-style sudoku solving.  This results in that this library does not
require any dependencies outside the standard library. With a few future changes, this library could be used where
the `no_std` attribute is needed.  

For 9x9 puzzles, use `use rustoku::basic::*` to import all the structs and traits needed for normal 9x9 puzzles.  
For 16x16 or 25x25, use `use::rustoku::medium::*`  
For 36x36 or 49x49, use `use::rustoku::large::*`  
And for some reason you want to go bigger:  
for 64x64, 81x81, or 100x100 puzzles: `use::rustoku::xlarge::*`  

## Examples
These examples are included in the repo.  To run an example:
```cargo
cargo run --example <example_name>
```
where `<example_name>` is replaced with `brute`, `human`, `hint`, or others.

A string input with periods (`.`) is used to input the unsolved puzzle.  Length, along with being a valid puzzle,
are checked.  An `Err` is returned if an invalid string is used.

### Brute Force

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
   use rustoku::basic::*;
   use rustoku::OutputString;
   let puz = "...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9...";
   let puzzle = Sudoku::new(puz)?;
   println!("--Original--\n{}", puzzle.output_string('.', None));
   if let Solution::One(grid) = puzzle.solution() {
      println!("--Brute--\n{}", grid.output_string('.', None));      
   }
   Ok(())
}
```

This example will brute force solve the input string.  The solution can be obtained using `puz.solution()` or displayed
through implementation of the `Display` trait.

This will result in the following output:
```
--Original--
...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9...
--Brute--
742156839963428517158397642316985724495712368827634951689243175534871296271569483

```

### Human Solving

```rust
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

```

This example solves the puzzle using both human and brute force techniques, then ensures the solutions 
are identical.  
The following output would be displayed:
```
--Human--
"145327698839654127672918543496185372218473956753296481367542819984761235521839764"

There were 3 basic fish techniques used to solve this puzzle

```


Human solving uses a difficulty system so that the easiest techniques are performed first, and moves up in difficulty until a hint is found.
Once a hint is found and applied, the solver starts over again at the easiest technique. Future work will involve allowing custom difficulty calculations.

### Playing Sudoku

This library allows making a sudoku game, not just solving puzzles.  See the example app for a demo.


## Examples
Clone the repo and check out examples for more.



## Screenshots from a demo app
A simple demo was made using this library and WebAssembly, found <a href="https://github.com/timlikestacos/rustoku-web">here, the rustoku-web repo.</a> 

![Claiming Candidates](https://github.com/TimLikesTacos/Rustoku-web/blob/main/screenshots/claiming.png?raw=true)  
![Finned Jellyfish](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/finned_jelly.png?raw=true)  
![Finned Swordfish](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/finnedswordfish.png?raw=true)
![Finned Xwing](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/finnedxwing.png?raw=true)
![hiddendouble](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/hiddendouble.png?raw=true)
![Hidden Quad](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/hiddenquad.png?raw=true)
![Pointing](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/pointing.png?raw=true)
![Swordfish](https://github.com/timlikestacos/rustoku-web/blob/main/screenshots/swordfish.png?raw=true)
