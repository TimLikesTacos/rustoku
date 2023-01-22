use crate::grid::gridcoord::HouseCoord;
use crate::grid::*;
use crate::human_calcs::basic::candidates::Candidates;
use crate::human_calcs::basic::PointingCandidate;
use crate::human_calcs::technique::Technique;
use crate::move_change::Move;
use std::marker::PhantomData;

use crate::bitset::*;
use crate::grid::house::House;
use crate::human_calcs::TechStruct;
use crate::sudoku::sudoku::Sudoku;

impl<V: BitSetInt> TechStruct for PointingCandidate<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}

impl<V: BitSetInt> PointingCandidate<V> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }

    fn _hint(&self, puz: &Sudoku<V>) -> Option<Vec<Move<V>>> {
        let mut vec = vec![];
        for step in 0..puz.house_size() {
            let dirs = [House::Row(step), House::Col(step)];

            for dir in dirs.iter() {
                if let Some(h) = Self::pointing_candidate(puz, dir) {
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

    fn pointing_candidate(puz: &Sudoku<V>, dir: &House<usize>) -> Option<Move<V>> {
        for step in 0..puz.box_dimension() {
            let amove = match dir {
                House::Row(n) => {
                    let row = Row::new(*n, step * puz.box_dimension(), puz.sudoku_size());
                    let base_box = row.to_box();
                    let cands = Self::find_candidates_pointing(puz, row, base_box);
                    Self::pointing(puz, &cands, base_box, row)
                }
                House::Col(n) => {
                    let col = Col::new(*n, step * puz.box_dimension(), puz.sudoku_size());
                    let base_box = col.to_box();
                    let cands = Self::find_candidates_pointing(puz, col, base_box);
                    Self::pointing(puz, &cands, base_box, col)
                }
                _ => unreachable!("Box is not used for pointing candidate"),
            };

            if amove.is_some() {
                return amove;
            }
        }
        None
    }

    /// Finds pointing candidates for the row/col passed in as a parameter.  Returns a `Hint` to the first
    /// candidate found.
    fn find_candidates_pointing(
        puz: &Sudoku<V>,
        house: impl HouseCoord,
        gridbox: GridBox,
    ) -> Candidates<V, ()> {
        // Values within the row/col in the box
        let mut box_both = BitSet::empty();
        // Values within box, but not in the row / col
        let mut box_outside = BitSet::empty();
        // Values in row/col, but not in box
        let mut house_outside = BitSet::empty();
        // Indices that have been visited by the house iter
        let mut indices = Vec::new();

        // This is the row or column iterator, gathering values that are inside or outisde the box
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
            extra: (),
        }
    } // End candidates function

    fn pointing<'b, 'a: 'b>(
        puz: &'a Sudoku<V>,
        candidates: &'b Candidates<V, ()>,
        gridbox: GridBox,
        house: impl HouseCoord,
    ) -> Option<Move<V>> {
        /*  Pointing candidates are ones that only possiblities for a value in a box
           are also in a row.  This means that any square that has this value in the
           row / column can be eliminated. We calculate this by finding what flags are
           in both the box and row / col but not in the remaining box.
        */
        // Get values that make a point

        let pointingv = candidates.box_both.difference(candidates.box_outside);
        // See if there are any values that can be eliminated.
        let pointing = pointingv.intersect(candidates.outside);
        if pointing == BitSet::empty() {
            return None;
        }
        // Get the first value.  Only performing one at a time.

        // Create the move for this value.
        let mut themove = Move::new();
        themove.method = Some(Technique::Pointing);
        let mut index = house.restart();

        for sq in puz.grid.house_iter(house) {
            let values = sq.poss().intersect(pointing);
            if values != BitSet::empty() {
                let this_box = index.to_box();
                match this_box.box_num == gridbox.box_num {
                    true => themove.add_used_to_solve(index.index(), values),
                    false => themove.add_removed_potential(index.index(), values),
                }
            }
            if let Some(ind) = index.inc() {
                index = ind;
            }
        }

        Some(themove)
    } // End pointing function
}
//
#[cfg(test)]
mod pointing_candidates {

    use super::*;
    use crate::grid::grid_traits::Rotate;
    use crate::human_calcs::technique::Technique;
    use crate::sudoku::sudoku::Sudoku;
    use std::collections::HashSet;
    use std::error::Error;

    #[test]
    fn locked_pointing() -> Result<(), Box<dyn Error>> {
        // Test puzzle found here: http://hodoku.sourceforge.net/en/tech_intersections.php

        let in_str =
            "984........25...4...19.4..2..6.9723...36.2...2.9.3561.195768423427351896638..9751";

        let in_str2 =
            "34...6.7..8....93...2.3..6.....1.....9736485......2...............6.8.9....923785";
        let mut puzzle = <Sudoku<u64>>::new_with_size(in_str, SudokuSize::Three)?;
        let hint = PointingCandidate::pointing_candidate(&mut puzzle, &House::Row(2usize));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Pointing);
        expected.add_used_to_solve(puzzle.upgrade_index(18), BitSet::from(5));
        expected.add_used_to_solve(puzzle.upgrade_index(19), BitSet::from(5));
        expected.add_removed_potential(puzzle.upgrade_index(24), BitSet::from(5));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected, "{:?}", report);

        // Ensure the puzzle is still solvable.
        assert!(puzzle.compare_with_solution()?);

        //** Rotate
        let mut puzzle: Sudoku<u16> =
            Sudoku::new_with_size(in_str, SudokuSize::Three)?.cwrotate(1)?;
        let hint = PointingCandidate::pointing_candidate(&mut puzzle, &House::Col(6));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Pointing);

        expected.add_used_to_solve(puzzle.upgrade_index(6), BitSet::from(5u16));
        expected.add_used_to_solve(puzzle.upgrade_index(15), BitSet::from(5));
        expected.add_removed_potential(puzzle.upgrade_index(60), BitSet::from(5u16));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        // Ensure the puzzle is still solvable.
        assert!(puzzle.compare_with_solution()?);

        //** Rotate 1 more time
        let mut puzzle: Sudoku<u16> = Sudoku::new_with_size(in_str, SudokuSize::Three)
            .expect("valid puzzle")
            .cwrotate(2)
            .expect("Should rotate");
        let hint = PointingCandidate::pointing_candidate(&mut puzzle, &House::Row(6));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Pointing);
        expected.add_used_to_solve(puzzle.upgrade_index(61), BitSet::from(5u16));
        expected.add_used_to_solve(puzzle.upgrade_index(62), BitSet::from(5u16));
        expected.add_removed_potential(puzzle.upgrade_index(56), BitSet::from(5u16));

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        // Ensure the puzzle is still solvable.
        assert!(puzzle.compare_with_solution()?);

        let mut puzzle = Sudoku::new_with_size(in_str2, SudokuSize::Three)?;
        let hint = PointingCandidate::pointing_candidate(&mut puzzle, &House::Row(6));

        // Create the expected move
        let mut expected = Move::new();
        expected.method = Some(Technique::Pointing);
        expected.add_used_to_solve(puzzle.upgrade_index(57), BitSet::from(1u16));
        expected.add_used_to_solve(puzzle.upgrade_index(59), BitSet::from(1u16));

        let changes = [54, 55, 56, 60, 61, 62];
        for c in &changes {
            expected.add_removed_potential(puzzle.upgrade_index(*c), BitSet::from(1u16));
        }

        // Check that this solve technique is properly found, executed, and reported
        let report = hint.unwrap().apply(&mut puzzle);

        assert_eq!(report, expected);

        // Ensure the puzzle is still solvable.
        assert!(puzzle.compare_with_solution()?);
        let hint2 = PointingCandidate::pointing_candidate(&mut puzzle, &House::Row(6)).unwrap();
        let ivec: HashSet<usize> = hint2
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert_eq!(hint2.technique(), &Some(Technique::Pointing));
        assert!(ivec.contains(&60));
        assert!(ivec.contains(&62));
        assert!(!ivec.contains(&61));

        hint2.apply(&mut puzzle);
        let hint3 = PointingCandidate::new()._hint(&puzzle).unwrap();

        hint3[0].clone().apply(&mut puzzle);
        let hint4 = PointingCandidate::new()._hint(&puzzle);
        assert!(hint4.is_none(), "{:?}", hint4);

        // Check for false positives
        let mut puzzle: Sudoku<u32> = Sudoku::new_with_size(in_str2, SudokuSize::Three)?;
        assert_eq!(puzzle.grid[11].poss(), &BitSet::from_binary(0b11_0001));
        assert_eq!(puzzle.grid[28].poss(), &BitSet::from_binary(0b11_0110));
        let hint = PointingCandidate::pointing_candidate(&mut puzzle, &House::Col(0));
        assert!(hint.is_none(), "{:?}", hint);
        Ok(())
    }

    #[test]
    fn issuefromothertest() {
        let mut s: Sudoku<u16> = Sudoku::new_with_size(
            ".28..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SudokuSize::Three,
        )
        .unwrap();
        loop {
            if let Some(hint) = PointingCandidate::pointing_candidate(&mut s, &House::Col(5)) {
                hint.apply(&mut s);
            } else {
                break;
            }
        }
    }
}
