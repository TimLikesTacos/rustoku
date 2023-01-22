use crate::bitset::*;
use crate::grid::gridcoord::HouseCoord;
use crate::grid::house::House;
use crate::grid::*;
use crate::human_calcs::basic::candidates::Candidates;
use crate::human_calcs::basic::ClaimingCandidate;
use crate::human_calcs::technique::Technique;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::Sudoku;
use std::marker::PhantomData;

impl<V: BitSetInt> TechStruct for ClaimingCandidate<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}
impl<T: BitSetInt> ClaimingCandidate<T> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }

    fn _hint(&self, puz: &Sudoku<T>) -> Option<Vec<Move<T>>> {
        let mut vec = Vec::new();
        for step in 0..puz.house_size() {
            let dirs = [House::Row(step), House::Col(step)];

            for dir in dirs.iter() {
                if let Some(h) = Self::claiming_candidate(puz, dir) {
                    vec.push(h);
                }
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }
    fn claiming_candidate(puz: &Sudoku<T>, dir: &House<usize>) -> Option<Move<T>> {
        for step in 0..puz.box_dimension() {
            let amove = match dir {
                House::Row(n) => {
                    let row = Row::new(*n, step * puz.box_dimension(), puz.sudoku_size());
                    let base_box = row.to_box();
                    let cands = Self::find_candidates(puz, row, base_box);
                    Self::claiming(puz, &cands, base_box)
                }
                House::Col(n) => {
                    let col = Col::new(*n, step * puz.box_dimension(), puz.sudoku_size());
                    let base_box = col.to_box();
                    let cands = Self::find_candidates(puz, col, base_box);
                    Self::claiming(puz, &cands, base_box)
                }
                _ => unreachable!("Box is not used for claiming candidate"),
            };

            if amove.is_some() {
                return amove;
            }
        }
        // No candidates where found.
        None
    }

    // `Step` is in the nominal range of 0..=2, as in the 1st to 3rd row or column in the box.
    fn find_candidates<'a, 'b: 'a>(
        puz: &'b Sudoku<T>,
        house: impl HouseCoord,
        gridbox: GridBox,
    ) -> Candidates<T, Vec<usize>> {
        // Values within the row/col in the box
        let mut box_both = BitSet::empty();
        // Values within box, but not in the row / col
        let mut box_outside = BitSet::empty();
        // Values in row/col, but not in box
        let mut house_outside = BitSet::empty();
        // Indices that have been visited by the house iter
        let mut indices = Vec::new();

        for house_sq in puz.grid.house_iter(house) {
            // If square is in the box, add it to box_both.
            // If not, then add it to outside
            let which_box_is_house = Index::new(house_sq.index, puz.sudoku_size()).to_box();
            indices.push(house_sq.index);

            if which_box_is_house.box_num == gridbox.box_num {
                box_both = box_both.union(*house_sq.poss());
            } else {
                house_outside = house_outside.union(*house_sq.poss());
            }
        }

        for box_sq in puz
            .grid
            .house_iter(gridbox)
            .filter(|box_sq| !indices.contains(&box_sq.index))
        {
            box_outside = box_outside.union(*box_sq.poss());
        }

        Candidates {
            box_both,
            box_outside,
            outside: house_outside,
            extra: indices,
        }
    } // End candidates

    fn claiming<'a, 'b: 'a>(
        puz: &'b Sudoku<T>,
        candidates: &'a Candidates<T, Vec<usize>>,
        gridbox: GridBox,
    ) -> Option<Move<T>> {
        /*  Claiming candidates are ones that the only possibilities for a value in
           a row or column are in the box.  This means that any square that has this
           value in the box can be eliminated.  We calculate this be finding what
           flags are in both box and row that are not in the rest of the row / col.
        */
        // Get values that make a claim
        let poss_claiming = candidates.box_both.difference(candidates.outside);
        // Find which values we can eliminate in the rest of the box.
        let claiming = poss_claiming.intersect(candidates.box_outside);
        // No values are potential claiming candidates
        if claiming == BitSet::empty() {
            return None;
        }

        // Create the move for this value.
        let mut themove = Move::new();
        themove.method = Some(Technique::Claiming);

        for sq in puz.grid.house_iter(gridbox) {
            let values = sq.poss().intersect(claiming);
            if values != BitSet::empty() {
                match candidates.extra.contains(&sq.index) {
                    true => themove.add_used_to_solve(puz.upgrade_index(sq.index), values),
                    false => themove.add_removed_potential(puz.upgrade_index(sq.index), values),
                }
            }
        }
        Some(themove)
    } // End Claiming
}

#[cfg(test)]
mod claiming {
    use crate::move_change::Move;
    use std::error::Error;

    use super::*;
    use crate::grid::grid_traits::Rotate;
    use crate::*;

    #[test]
    fn locked_claiming() -> Result<(), Box<dyn Error>> {
        // Test puzzle found here: http://hodoku.sourceforge.net/en/tech_intersections.php

        let in_str =
            "318..54.6...6.381...6.8.5.3864952137123476958795318264.3.5..78......73.5....39641";

        let in_str2 =
            "762..8..198......615.....87478..3169526..98733198..425835..1692297685314641932758";
        let mut puzzle = Sudoku::new_with_size(in_str, SudokuSize::Three)?;
        let hint = ClaimingCandidate::claiming_candidate(&puzzle, &House::Row(1));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Claiming);
        expected.add_used_to_solve(puzzle.upgrade_index(10), BitSet::from(7u16));
        expected.add_used_to_solve(puzzle.upgrade_index(11), BitSet::from(7u16));
        expected.add_removed_potential(puzzle.upgrade_index(19), BitSet::from(7u16));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        let mut puzzle = Sudoku::new_with_size(in_str, SudokuSize::Three)?.cwrotate(1)?;
        let hint = ClaimingCandidate::claiming_candidate(&puzzle, &House::Col(7));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Claiming);
        expected.add_used_to_solve(puzzle.upgrade_index(16), BitSet::from(7u32));
        expected.add_used_to_solve(puzzle.upgrade_index(25), BitSet::from(7u32));
        expected.add_removed_potential(puzzle.upgrade_index(15), BitSet::from(7u32));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        let mut puzzle = Sudoku::new_with_size(in_str, SudokuSize::Three)?.cwrotate(2)?;
        let hint = ClaimingCandidate::claiming_candidate(&puzzle, &House::Row(7));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Claiming);
        expected.add_used_to_solve(puzzle.upgrade_index(69), BitSet::from(7u64));
        expected.add_used_to_solve(puzzle.upgrade_index(70), BitSet::from(7u64));
        expected.add_removed_potential(puzzle.upgrade_index(61), BitSet::from(7u64));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        // Ensure the puzzle is still solvable.
        let copy = Sudoku::<u64>::new_with_size(in_str, SudokuSize::Three)?;
        let _copy = copy.unique_solution()?.cwrotate(2)?;
        assert!(puzzle.compare_with_solution().unwrap());

        let mut puzzle = Sudoku::new_with_size(in_str2, SudokuSize::Three)?;
        let hint = ClaimingCandidate::claiming_candidate(&mut puzzle, &House::Col(5));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Claiming);
        expected.add_used_to_solve(puzzle.upgrade_index(14), BitSet::from(4u128));
        expected.add_used_to_solve(puzzle.upgrade_index(23), BitSet::from(4u128));
        let changes = [3, 4, 12, 13, 21, 22];
        for c in &changes {
            expected.add_removed_potential(puzzle.upgrade_index(*c), BitSet::from(4u128));
        }

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        // Ensure the puzzle is still solvable.
        assert!(puzzle.compare_with_solution()?);

        Ok(())
    }
}
