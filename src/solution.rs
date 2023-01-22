use crate::errors::SudError;
use crate::grid::grid::Grid;
use crate::move_change::Move;

use crate::bitset::BitSetInt;

/// Solutions enum stores the solution to the puzzle.  It has three variants:
/// `None` which means there are no solutions
/// `One` which means that there is one solution, and the enum contains the solution `Grid`
/// `Multi` which means that there are multiple solutions.  This variant contains a `Vec<Grid>` of
/// the possible solutions.
#[derive(Debug, Clone)]
pub enum Solution<T>
where
    T: BitSetInt,
{
    /// No solution
    None,
    /// No solution solving method has been applied yet
    NotSet,
    One(Grid<T>),
    Multi(Vec<Grid<T>>),
    // /// Solved by human solving techniques. Note that this finds the first solution and will not find multiple solutions.
    HumanSolved(Grid<T>, Vec<Move<T>>),
}

impl<T: BitSetInt> Solution<T> {
    /// Returns the number of solutions.
    pub fn num_solutions(&self) -> Result<usize, SudError> {
        match self {
            Solution::None => Ok(0),
            Solution::One(_) => Ok(1),
            Solution::Multi(v) => Ok(v.len()),
            Solution::HumanSolved(_, _) => Ok(1),
            Solution::NotSet => Err(SudError::HasNotBeenSolved),
        }
    }

    /// Returns the unique solution.  If no solutions or multiple solutions, returns None.
    #[inline]
    pub fn get(&self) -> Result<Grid<T>, SudError> {
        match self {
            Solution::One(g) => Ok(g.clone()),
            Solution::HumanSolved(g, _) => Ok(g.clone()),
            Solution::NotSet => Err(SudError::HasNotBeenSolved),
            Solution::None => Err(SudError::NoSolution),
            Solution::Multi(g) => Err(SudError::MultipleSolution(g.len())),
        }
    }
}

impl<T: BitSetInt> PartialEq for Solution<T> {
    fn eq(&self, other: &Self) -> bool {
        use Solution::*;
        match self {
            None => matches!(other, None),
            NotSet => matches!(other, NotSet),
            One(_) => matches!(other, One(_)),
            Multi(_) => matches!(other, Multi(_)),
            HumanSolved(_, _) => matches!(other, HumanSolved(_, _)),
        }
    }
}
