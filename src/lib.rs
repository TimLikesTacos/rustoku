/*! A fast sudoku framework for finding solutions by brute force and human solving techniques.
   Testing has not been performed for puzzles larger than normal 9x9 puzzles.
**/

pub(crate) mod brute;
pub mod errors;
mod grid;
pub mod hint;

#[cfg(not(test))]
pub(crate) mod bitset;
#[cfg(test)]
pub mod bitset;

pub mod human_calcs;
pub(crate) mod move_change;
pub(crate) mod solution;
pub(crate) mod square;
pub(crate) mod sudoku;
pub(crate) mod sudokusize;
pub(crate) use sudoku::sudoku::Sudoku;

pub use grid::grid::Grid;
pub use grid::types::Index;
pub use human_calcs::technique::Technique;
pub use sudoku::initial_input::InitialInput;
pub use sudoku::output::OutputString;
pub use sudoku::sudoku::Move;
/// Use the basic mod for normal 9x9 puzzles
pub mod basic {
    pub type Sudoku = crate::sudoku::sudoku::Sudoku<u16>;
    pub type Move = crate::sudoku::sudoku::Move<u16>;
    pub type HumanSolver = crate::human_calcs::humansolve::solver::HumanSolver<u16>;
    pub type Hint = crate::hint::Hint<u16>;
    pub type Solution = crate::solution::Solution<u16>;
}

/// Use this module for 16x16 or 25x25 puzzles
pub mod medium {
    pub type Sudoku = crate::sudoku::sudoku::Sudoku<u32>;
    pub type Move = crate::sudoku::sudoku::Move<u32>;
    pub type HumanSolver = crate::human_calcs::humansolve::solver::HumanSolver<u32>;
    pub type Hint = crate::hint::Hint<u32>;
    pub type Solution = crate::solution::Solution<u32>;
}

/// use this module for 36x36 or 49x49 puzzles
pub mod large {
    pub type Sudoku = crate::sudoku::sudoku::Sudoku<u64>;
    pub type Move = crate::sudoku::sudoku::Move<u64>;
    pub type HumanSolver = crate::human_calcs::humansolve::solver::HumanSolver<u64>;
    pub type Hint = crate::hint::Hint<u64>;
    pub type Solution = crate::solution::Solution<u64>;
}

/// Use this module for 64x64, 81x81, or 100x100 puzzles
pub mod xlarge {
    pub type Sudoku = crate::sudoku::sudoku::Sudoku<u128>;
    pub type Move = crate::sudoku::sudoku::Move<u128>;
    pub type HumanSolver = crate::human_calcs::humansolve::solver::HumanSolver<u128>;
    pub type Hint = crate::hint::Hint<u128>;
    pub type Solution = crate::solution::Solution<u128>;
}

pub mod custom {
    pub use crate::sudoku::sudoku::Sudoku;
}

pub type SudError = sudoku::sudoku::SudError;
