//! Custom errors

#[derive(Debug, PartialEq)]
pub enum SudError {
    InputParse,
    InputLengthError(usize),
    OutputParse,
    NoSolution,
    MultipleSolution(usize),
    ExcessiveSolutions(usize),
    HumanSolveError,
    HasNotBeenSolved,
    InvalidPuzzle,
    ValueNotPossible(String),
    InvalidLocation((usize, usize)),
    IllegalOperation(&'static str),
    ConflictingValues(usize, usize),
    IndexOutOfRange(usize),
    NotSolved(usize, usize),
}

impl std::error::Error for SudError {}

impl std::fmt::Display for SudError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SudError::*;

        match self {

            InputParse => write!(f, "Parse error when attempting puzzle input"),
            InputLengthError(s) => write!(
                f,
                "Input length different than expected.  Actual: {}, expected: {}",
                s,
                // todo fix these numbers
                9 * 9
            ),
            OutputParse => write!(f, "Parse error when attempting output"),
            NoSolution => write!(f, "There is no solution for the given input"),
            MultipleSolution(s) => write!(
                f,
                "There is not a unique solution. There are {} solutions.",
                s
            ),
            ExcessiveSolutions(s) => write!(f, "There are more than {} solutions and solution calculation was halted", s),
            HumanSolveError => write!(f, "There was an issue with the human solve calculation"),
            InvalidPuzzle => write!(f, "The provided puzzle input is invalid. There are conflicts with the provided numbers"),
            ValueNotPossible(i) => write!(f, "The value entered is not a valid possibility at index {}", i),
            // todo fix these too
            InvalidLocation(coord) => write!(f, "The location of : ({}, {}) exceeds puzzle parameters", coord.0, coord.1),
            IllegalOperation(s) => write!(f, "Illegal operation: {}", s),
            HasNotBeenSolved => write!(f, "The puzzle has not been solved yet."),
            ConflictingValues(first, second) => write!(f, "There is at least one conflicting value at index: {} with index: {}", first, second),
            IndexOutOfRange(index) => write!(f, "Index: {} out of range", index),
            NotSolved(missing, conflicts) => write!(f, "Not yet solved. {} missing values and {} conflicting values", missing, conflicts)
        }
    }
}
