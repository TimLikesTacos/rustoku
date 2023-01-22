use crate::bitset::BitSetInt;
use crate::errors::SudError;
use crate::grid::grid::Grid;
use crate::human_calcs::humansolve::tech_index::TechniqIndex;
use std::fmt::{Debug, Formatter};

use crate::hint::Hint;
use crate::human_calcs::technique::Technique;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::square::Square;
use crate::Sudoku;

pub struct HumanSolver<V> {
    solvers: Vec<Box<dyn TechStruct<SquareValue = V>>>,
    methods: Vec<Technique>,
    current: usize,
}

impl<V: BitSetInt> Clone for HumanSolver<V> {
    fn clone(&self) -> Self {
        HumanSolver {
            solvers: self
                .methods
                .iter()
                .map(|tech_enum| tech_enum.solver())
                .collect(),
            methods: self.methods.clone(),
            current: self.current,
        }
    }
}

impl<T> Debug for HumanSolver<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HumanSolver:\nmethods: {:?}\ncurrent: {:?}",
            self.methods, self.current
        )
    }
}

impl<V: BitSetInt> HumanSolver<V> {
    pub fn new() -> HumanSolver<V> {
        let mut methods: Vec<Technique> = Technique::iterator().cloned().collect();
        // Sort the methods by increasing difficulty.
        methods.sort_by(|a, b| {
            a.default_difficulty()
                .partial_cmp(&b.default_difficulty())
                .unwrap()
        });
        HumanSolver {
            solvers: methods.iter().map(|tech_enum| tech_enum.solver()).collect(),
            methods,
            current: 0,
        }
    }

    #[inline]
    #[allow(dead_code)]
    fn method(&self, ind: usize) -> Option<&Technique> {
        self.methods.get(ind)
    }

    /// Returns the next hint.  Starts with the lowest priority and increases difficulty.
    ///  Once a hint is found, the solver starts at the easiest technique
    #[inline]
    pub fn next(&self, puz: &Sudoku<V>) -> Option<Hint<V>> {
        self.next_bones(puz, self.methods.len())
    }

    /// Returns the next hint up to but not including the technique passed in
    #[inline]
    pub fn next_up_to<I: TechniqIndex>(&self, puz: &Sudoku<V>, technique: I) -> Option<Hint<V>> {
        if let Some(max) = technique.tech_index(&self.methods) {
            self.next_bones(puz, max)
        } else {
            None
        }
    }

    fn next_bones(&self, puz: &Sudoku<V>, max: usize) -> Option<Hint<V>> {
        let mut current = 0usize;
        while current < max {
            if let Some(m) = self.solvers[current].tech_hint(puz) {
                // If a hint is found, return to the easiest technique
                return Some(Hint(m));
            } else {
                current += 1;
            }
        }
        None
    }

    pub fn solve(&self, puz: &Sudoku<V>) -> Result<(Grid<Square<V>>, Vec<Move<V>>), SudError> {
        let mut puz = puz.clone();

        let mut ret = vec![];
        while puz.remaining() > 0 {
            if let Some(amove) = self.next(&puz) {
                // Get a move using the current method.  If Some is returned, record the move and move to easier method
                ret.push(amove.apply(&mut puz));
            } else {
                // For some reason, guess did not return a value, which means that the puzzle is unsolvable.
                return Err(SudError::HumanSolveError);
            }
        }

        Ok((puz.grid, ret))
    }

    /// Solves the puzzle up to the point where the next easiest hint WILL BE the technique passed in.
    /// Note that this may use more complex difficulties, and may solve the puzzle
    pub fn solve_to(&self, puz: &mut Sudoku<V>, technique: Technique) {
        while !puz.is_solved() {
            if let Some(m) = self.next(puz) {
                if *m.technique() == Some(technique) {
                    // The called technique exists, return from the function
                    return;
                }
                m.apply(puz);
            } else {
                return;
            }
        }
    }

    /// Solves the puzzle up to the point where the next easiest hint will be the technique passed in if possible, or the
    /// next available difficulty technique.
    /// Note that this may solve the puzzle, but will not use any technique more difficult than the
    /// passed in technique
    pub fn solve_up_to(&self, puz: &mut Sudoku<V>, technique: Technique) {
        while !puz.is_solved() {
            if let Some(m) = self.next_up_to(puz, technique) {
                m.apply(puz);
            } else {
                return;
            }
        }
    }
}

impl<V: BitSetInt> Default for HumanSolver<V> {
    fn default() -> Self {
        HumanSolver::new()
    }
}

#[cfg(test)]
mod methodselector {
    use super::*;
    use crate::solution::Solution;
    use crate::sudokusize::{Res, SudokuSize};

    fn get_solution<V: BitSetInt>(puz: &Sudoku<V>) -> Grid<V> {
        let sol = puz.solution();
        match sol {
            Solution::One(g) => g.clone(),
            _ => panic!("Does not have a unique solution"),
        }
    }

    #[test]
    fn all_single_poss() -> Res<()> {
        // This example is solveable with only singleposs
        let puz = Sudoku::new_with_size(
            vec![
                vec![5u16, 3, 4, 0, 7, 0, 0, 0, 0],
                vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
                vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
                vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
                vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
                vec![0, 1, 0, 0, 2, 0, 0, 0, 6],
                vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
                vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
                vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
            ]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>(),
            SudokuSize::Three,
        )?;

        let human = HumanSolver::new();
        let (sol, moves) = human.solve(&puz)?;
        for amove in moves {
            let possdiff = Technique::SinglePossibility.default_difficulty();
            let lessthan = |a: f32, b: f32| -> bool { a - b < 0.001f32 };

            assert!(lessthan(
                amove.technique().unwrap().default_difficulty(),
                possdiff
            ));
        }

        let solution = get_solution(&puz);
        //assert_eq!(Sudoku::<u16>::valid_entries_squares(&sol), Ok(81));
        assert_eq!(sol, solution);
        Ok(())
    }

    #[test]
    fn pointing() -> Res<()> {
        let in_str =
            "984........25...4...19.4..2..6.9723...36.2...2.9.3561.195768423427351896638..9751";

        let in_str2 =
            "34...6.7..8....93...2.3..6.....1.....9736485......2...............6.8.9....923785";

        let puz = Sudoku::<u16>::new_with_size(in_str, SudokuSize::Three)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );

        let puz = Sudoku::<u32>::new(in_str2)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            &moves
        );

        Ok(())
    }

    #[test]
    fn tuple1() -> Res<()> {
        let inp =
            ".49132....81479...327685914.96.518...75.28....38.46..5853267...712894563964513...";
        let puz = Sudoku::<u64>::new(inp)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );
        Ok(())
    }

    #[test]
    fn hiddenquad() -> Res<()> {
        let inp =
            ".3.....1...8.9....4..6.8......57694....98352....124...276..519....7.9....95...47.";
        let sud = Sudoku::<u128>::new(inp)?;
        let (sol, moves) = sud.human_solve()?;
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );
        let puz = Sudoku::<u128>::new(inp)?;
        let _brute = puz.unique_solution()?;

        assert!(puz.compare_with_solution()?);
        assert_eq!(&sol, puz.unique_solution()?);
        Ok(())
    }

    #[test]
    fn jelly() -> Res<()> {
        let inp =
            "2.41.358.....2.3411.34856..732954168..5.1.9..6198324....15.82..3..24.....263....4";
        let puz = Sudoku::<u32>::new(inp)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );

        Ok(())
    }

    #[test]
    fn finned_jelly() -> Res<()> {
        let inp =
            "...16.87..1.875..38.73..651.5.62173...17..5.473.5..1...7........8.256917.62..7...";
        let puz = Sudoku::<u16>::new(inp)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );
        // ensure a finned jellyfish is used.
        assert!(moves
            .iter()
            .any(|m| m.technique() == &Some(Technique::FinnedJellyfish)));
        // check to make sure guessingis not being used
        assert!(
            moves
                .iter()
                .all(|m| m.technique() != &Some(Technique::Guess)),
            "{:?}",
            moves
        );
        Ok(())
    }

    #[test]
    fn hardest() -> Res<()> {
        // By Dr. Arto Inkala, supposed to be one of the most difficult sudokus

        let inp =
            "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
        let puz = Sudoku::<u16>::new(inp)?;
        let (sol, moves) = puz.human_solve()?;
        assert_eq!(&sol, puz.unique_solution()?);
        // As more complex solving techniques are created, this value should approach zero.
        println!(
            "Number of guesses for the hardest sudoku: {}",
            moves
                .iter()
                .filter(|m| m.technique() == &Some(Technique::Guess))
                .count()
        );
        Ok(())
    }

    // #[test]
    // fn solve_to() -> Result<(), SudError> {
    //     let inp =
    //         "2.41.358.....2.3411.34856..732954168..5.1.9..6198324....15.82..3..24.....263....4";
    //     let mut puz = Sudoku::new(inp)?;
    //     let original = puz.clone();
    //     let solver = HumanSolver::new();
    //     assert_ne!(&puz as *const Sudoku, &original as *const Sudoku);
    //
    //     solver.solve_to(&mut puz, Technique::Jellyfish);
    //
    //     assert_ne!(puz.grid, original.grid);
    //     let puznext = solver.next(&puz).unwrap();
    //     let orignext = solver.next(&original).unwrap();
    //     assert_ne!(puznext, orignext);
    //
    //     assert_eq!(*puznext.technique(), Technique::Jellyfish);
    //
    //     Ok(())
    // }
    //
    // #[test]
    // fn solve_up_to() -> Result<(), SudError> {
    //     let inp =
    //         "2.41.358.....2.3411.34856..732954168..5.1.9..6198324....15.82..3..24.....263....4";
    //     let mut puz = Sudoku::new(inp)?;
    //     let original = puz.clone();
    //     let solver = HumanSolver::new();
    //
    //     solver.solve_up_to(&mut puz, Technique::Swordfish);
    //
    //     assert_ne!(puz.grid, original.grid);
    //     let puznext = solver.next(&puz).unwrap();
    //     let orignext = solver.next(&original).unwrap();
    //     assert_ne!(puznext, orignext, "\n\n{:?}\n\n", solver);
    //
    //     assert_eq!(*puznext.technique(), Technique::Jellyfish);
    //
    //     Ok(())
    // }
}
