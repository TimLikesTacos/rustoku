use crate::bitset::BitSetInt;
use crate::human_calcs::basic::SingleCandidate;
use crate::human_calcs::technique::Technique;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudoku::sudoku::Sudoku;
use std::marker::PhantomData;

impl<V: BitSetInt> TechStruct for SingleCandidate<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        Self::_hint(puz)
    }
}
/// Finds cells that have only one possible value, fills it in, and removes pencil marks for
/// related cells.  A single candidate is where only one value is possible in that cell.

impl<V: BitSetInt> SingleCandidate<V> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }

    fn _hint(puz: &Sudoku<V>) -> Option<Move<V>> {
        if let Some(sq) = puz.grid.iter().find(|sq| sq.count() == 1) {
            let mut themove = Move::new();
            themove.set_method(Some(Technique::SingleCandidate));
            themove.set_value(
                puz.upgrade_index(sq.index()),
                sq.poss().iter().next().unwrap().into(),
                false,
            );
            return Some(themove);
        }
        None
    }
}

#[cfg(test)]
mod singlecan {
    use super::*;
    use crate::bitset::Bit;
    use crate::errors::SudError;
    use crate::grid::SudokuSize;

    #[test]
    fn single_cand() -> Result<(), SudError> {
        let inv: Vec<u32> = vec![
            vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 2, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![0, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
        .iter()
        .flatten()
        .cloned()
        .collect();
        let mut puz = Sudoku::new_with_size(inv, SudokuSize::Three).unwrap();

        let expected: Vec<Vec<u32>> = vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ];

        let mut count = 0;
        while let Some(hint) = SingleCandidate::_hint(&puz) {
            hint.apply(&mut puz);
            count += 1;
        }

        assert_eq!(count, 50);
        // This puzzle is solved by 100% single candidates.
        for (act, exp) in puz.grid.iter().zip(expected.iter().flatten()) {
            assert_eq!(act.val(), Some(Bit::from(*exp)));
        }
        Ok(())
    }
}
