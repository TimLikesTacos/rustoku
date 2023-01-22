use crate::bitset::BitSetInt;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudoku::sudoku::Sudoku;
use crate::Technique;
use std::marker::PhantomData;

pub(crate) struct Guess<V>(PhantomData<V>);
impl<V: BitSetInt> TechStruct for Guess<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        Self::_hint(puz)
    }
}

impl<V: BitSetInt> Guess<V> {
    pub(crate) fn new() -> Self {
        Self(PhantomData::default())
    }

    /// Finds the square with the fewest possible values and 'guesses' the CORRECT value.
    pub(crate) fn _hint(puz: &Sudoku<V>) -> Option<Move<V>> {
        let (ind, _) = puz
            .grid
            .iter()
            .enumerate()
            .fold((0, u32::MAX), |(pos, lowest), (i, sq)| {
                if sq.poss().count_u32() < lowest && sq.poss().count_u32() > 0 {
                    (i, sq.poss().count_u32())
                } else {
                    (pos, lowest)
                }
            });

        if let Ok(solved) = puz.unique_solution() {
            let solution = solved[ind];
            let mut themove = Move::new();
            themove.set_value(puz.upgrade_index(ind), solution, false);
            themove.method = Some(Technique::Guess);
            Some(themove)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod guess {
    use crate::grid::SudokuSize;
    use crate::human_calcs::guess::Guess;
    use crate::sudoku::sudoku::Sudoku;
    use std::error::Error;

    #[test]
    fn guess_solve() -> Result<(), Box<dyn Error>> {
        // By Dr. Arto Inkala, supposed to be one of the most difficult sudokus

        let inp =
            "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
        let mut puz = <Sudoku<u16>>::new_with_size(inp, SudokuSize::Three)?;

        let mut remaining = puz.remaining();
        assert_ne!(remaining, 0);
        while remaining > 0 {
            Guess::_hint(&puz).unwrap().apply(&mut puz);
            let new_remaining = puz.remaining();
            assert_eq!(new_remaining, remaining - 1);
            remaining = new_remaining;
        }
        assert_eq!(remaining, 0);
        Ok(())
    }
}
