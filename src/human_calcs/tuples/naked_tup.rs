use std::collections::HashSet;
use std::marker::PhantomData;

use crate::grid::house::House;
use crate::grid::{Col, GridBox, Row};

use crate::bitset::*;
use crate::human_calcs::technique::Technique;
use crate::human_calcs::tuple_ctr::{Ctr, TupleCtr};
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::Sudoku;

pub struct NakedTuple<V> {
    pub(crate) size: usize,
    phantom: PhantomData<V>,
}
impl<V: BitSetInt> TechStruct for NakedTuple<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}

impl<V: BitSetInt> NakedTuple<V> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            phantom: PhantomData::default(),
        }
    }

    fn _hint(&self, puz: &Sudoku<V>) -> Option<Vec<Move<V>>> {
        let size = self.size;
        let max = puz.house_size();
        debug_assert!(size >= 2 && size <= max / 2);
        let solvet = match size {
            4 => Technique::NakedQuad,
            3 => Technique::NakedTriple,
            2 => Technique::NakedDouble,
            n => Technique::NakedTuple(n),
        };

        let mut vec = vec![];
        for rcb in 0..max {
            let houses = [House::Row(rcb), House::Col(rcb), House::Box(rcb)];
            for house in houses {
                // If a tuple finds a valid move, stop and return
                if let Some(mut m) = Self::findtuples(puz, house, size, solvet) {
                    vec.append(&mut m);
                }
            }
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn findtuples(
        puz: &Sudoku<V>,
        house: House<usize>,
        tuple_size: usize,
        tech: Technique,
    ) -> Option<Vec<Move<V>>> {
        let tups = match house {
            House::Row(n) => TupleCtr::find_tuples(
                puz,
                Row::new(n, 0, puz.sudoku_size()),
                Ctr::remove_indicies,
                tuple_size,
            ),
            House::Col(n) => TupleCtr::find_tuples(
                puz,
                Col::new(n, 0, puz.sudoku_size()),
                Ctr::remove_indicies,
                tuple_size,
            ),
            House::Box(n) => TupleCtr::find_tuples(
                puz,
                GridBox::new(n, 0, puz.sudoku_size()),
                Ctr::remove_indicies,
                tuple_size,
            ),
        };

        let mut it = tups
            .iter()
            .filter(|t| t.values.count_u32() == t.ind_count && t.ind_count != 0);
        let mut vec = Vec::new();

        loop {
            // Naked tuples are where the number of indices equals the number of values.
            if let Some(res) = it.next() {
                let inditer = res.index_iter();
                let puziter = puz.grid.enum_house_iter(&house, puz.sudoku_size());
                let mut themove = Move::new();
                themove.method = Some(tech);
                let indices: HashSet<usize> = inditer.collect();
                for (i, sq) in puziter.enumerate() {
                    if indices.contains(&i) {
                        themove.add_used_to_solve(
                            puz.upgrade_index(sq.index()),
                            res.values.intersect(*sq.poss()),
                        );
                    } else {
                        let rm = sq.poss().intersect(res.values);
                        if rm != BitSet::empty() {
                            themove.add_removed_potential(puz.upgrade_index(sq.index()), rm);
                        }
                    }
                }
                // Only get tuples that will remove potential values
                if !themove.removed_potentials_vec().is_empty() {
                    vec.push(themove);
                }
            } else if vec.is_empty() {
                return None;
            } else {
                return Some(vec);
            }
        }
    }
}

#[cfg(test)]
mod naked_tuple_tests {
    use super::*;

    use crate::grid::gridcoord::HouseCoord;
    use crate::grid::{Col, Coord, GridBox, Row, SudokuSize};
    use crate::sudoku::sudoku::Sudoku;
    use std::error::Error;

    #[test]
    fn naked_tuple_test1() -> Result<(), Box<dyn Error>> {
        // http://hodoku.sourceforge.net/en/tech_naked.php
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(
            vec![
                vec![7, 0, 0, 8, 4, 9, 0, 3, 0], //8
                vec![9, 2, 8, 1, 3, 5, 0, 0, 6], //17
                vec![4, 0, 0, 2, 6, 7, 0, 8, 9], //26
                vec![6, 4, 2, 7, 8, 3, 9, 5, 1], //35
                vec![3, 9, 7, 4, 5, 1, 6, 2, 8], //44
                vec![8, 1, 5, 6, 9, 2, 3, 0, 0], //53
                vec![2, 0, 4, 5, 1, 6, 0, 9, 3], //62
                vec![1, 0, 0, 0, 0, 8, 0, 6, 0], //71
                vec![5, 0, 0, 0, 0, 4, 0, 1, 0], //80
            ]
            .iter()
            .flatten()
            .cloned()
            .collect::<Vec<_>>(),
            SudokuSize::Three,
        )?;

        // Original is used to test for dropped flags after hidden tuples
        //let original = puz.clone();

        let i0 = Coord::new(7, 1, SudokuSize::Three);
        let i1 = Coord::new(7, 2, SudokuSize::Three);
        let i2 = Coord::new(7, 3, SudokuSize::Three);

        let f0 = puz._get(i0.to_usize()).poss();
        let f1 = puz._get(i1.to_usize()).poss();
        let f2 = puz._get(i2.to_usize()).poss();

        assert_eq!(f0, &BitSet::from_binary(0b0_0100_0100));
        assert_eq!(f1, &BitSet::from_binary(0b1_0000_0100));
        assert_eq!(f2, &BitSet::from_binary(0b1_0000_0100));

        // Check that tuples that do not remove potential values are not considered tuples
        let hint = NakedTuple::findtuples(&puz, House::Box(2), 2, Technique::NakedDouble);
        assert!(hint.is_none());
        let hint = NakedTuple::findtuples(&puz, House::Row(1), 2, Technique::NakedDouble);
        assert!(hint.is_none());
        let hint = NakedTuple::findtuples(&puz, House::Row(7), 2, Technique::NakedDouble);
        assert!(hint.is_some());
        hint.unwrap()[0].clone().apply(&mut puz);

        let f0 = puz._get(i0.to_usize()).poss();
        let f1 = puz._get(i1.to_usize()).poss();
        let f2 = puz._get(i2.to_usize()).poss();

        assert_eq!(f0, &BitSet::from_binary(0b0_0100_0000));
        assert_eq!(f1, &BitSet::from_binary(0b1_0000_0100));
        assert_eq!(f2, &BitSet::from_binary(0b1_0000_0100));

        // Check other values in box not affected
        let f3 = puz.grid[Row::new(8, 1, SudokuSize::Three).to_usize()].poss();
        let f4 = puz.grid[Row::new(8, 2, SudokuSize::Three).to_usize()].poss();

        assert_eq!(f3, &BitSet::from_binary(0b0_1110_0100));
        assert_eq!(f4, &BitSet::from_binary(0b1_0010_0100));

        let hint2 = NakedTuple::findtuples(&puz, House::Col(1), 2, Technique::NakedDouble);
        assert!(&hint2.is_some());
        // let hint2 = hint2.unwrap();
        // let tmove = hint2.get_involved_vec();
        // assert!(tmove.contains(&index_from_row(6, 1)));
        // assert!(tmove.contains(&index_from_row(7, 1)));
        // let tchange: Vec<_> = hint2.changes_iter().collect();

        Ok(())
    }

    #[test]
    fn naked_tuple_2() -> Result<(), Box<dyn Error>> {
        let inp =
            "687..4523953..261414235697831...724676....3.5.2....7.1.96..1.3223.....57.7.....69";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = NakedTuple::findtuples(&puz, House::Box(4), 2, Technique::NakedDouble);
        assert!(hint.is_some());
        let themove = hint.unwrap()[0].clone().apply(&mut puz);
        assert_eq!(
            puz.grid[GridBox::new(4, 0, SudokuSize::Three)].poss(),
            &BitSet::from_binary(0b1_0000)
        );
        let ivec: HashSet<usize> = themove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(ivec.contains(&GridBox::new(4, 1, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&GridBox::new(4, 5, SudokuSize::Three).to_usize()));
        assert_eq!(ivec.len(), 2);

        let cvec: Vec<usize> = themove
            .removed_potentials_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(cvec.contains(&GridBox::new(4, 0, SudokuSize::Three).to_usize()));

        assert!(cvec.contains(&GridBox::new(4, 3, SudokuSize::Three).to_usize()));

        assert!(cvec.contains(&GridBox::new(4, 4, SudokuSize::Three).to_usize()));

        assert!(cvec.contains(&GridBox::new(4, 6, SudokuSize::Three).to_usize()));

        assert!(cvec.contains(&GridBox::new(4, 7, SudokuSize::Three).to_usize()));

        assert!(cvec.contains(&GridBox::new(4, 8, SudokuSize::Three).to_usize()));

        assert_eq!(cvec.len(), 6);
        Ok(())
    }

    #[test]
    fn naked_triple1() -> Result<(), Box<dyn Error>> {
        //* This test has all three squares with possible values of 1, 2, 6 **/
        let inp =
            "39....7........65.5.7...349.4938.5.66.1.54983853...4..9..8..134..294.8654.....297";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = NakedTuple::findtuples(&puz, House::Box(1), 3, Technique::NakedTriple);
        assert!(hint.is_some());

        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        // let cvec = tmove.changes_vec();

        assert!(
            ivec.contains(&GridBox::new(1, 1, SudokuSize::Three).to_usize()),
            "{:?}",
            ivec
        );
        assert!(
            ivec.contains(&GridBox::new(1, 6, SudokuSize::Three).to_usize()),
            "{:?}",
            ivec
        );
        assert!(
            ivec.contains(&GridBox::new(1, 7, SudokuSize::Three).to_usize()),
            "{:?}",
            ivec
        );

        assert_eq!(
            puz.grid[GridBox::new(1, 1, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b10_0011)
        );
        assert_eq!(
            puz.grid[GridBox::new(1, 6, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b10_0011)
        );
        assert_eq!(
            puz.grid[GridBox::new(1, 7, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b10_0011)
        );
        Ok(())
    }

    #[test]
    fn naked_triple2() -> Result<(), Box<dyn Error>> {
        //** This test has 1 sq with 3, 6, 9; another sq with 3,6; and another with 3,9 as potentials**/
        let inp =
            "...29438....17864.48.3561....48375.1...4157..5..629834953782416126543978.4.961253";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = NakedTuple::findtuples(&puz, House::Col(1), 3, Technique::NakedTriple);
        assert!(hint.is_some());
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();

        assert!(ivec.contains(&Col::new(1, 1, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&Col::new(1, 3, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&Col::new(1, 4, SudokuSize::Three).to_usize()));

        assert_eq!(
            puz.grid[Col::new(1, 1, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b1_0000_0100)
        );
        assert_eq!(
            puz.grid[Col::new(1, 3, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b1_0010_0000)
        );
        assert_eq!(
            puz.grid[Col::new(1, 4, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b1_0010_0100)
        );

        //** There should be no other tuples other than a new triple on row 0.
        for rcb in 0..puz.house_size() {
            assert!(
                NakedTuple::findtuples(&puz, House::Row(rcb), 2, Technique::NakedDouble).is_none(),
                "num: {} Row size: {} \n",
                rcb,
                2
            );
            //assert!(NakedTuple::naked_tuple(&puz, rcb, GroupType::Row, 3).is_none(), "num: {} type: {:?} size: {} \n", rcb, GroupType::Row, 3);
            assert!(
                NakedTuple::findtuples(&puz, House::Col(rcb), 2, Technique::NakedDouble).is_none(),
                "num: {} Col size: {} \n",
                rcb,
                2
            );
            assert!(
                NakedTuple::findtuples(&puz, House::Col(rcb), 3, Technique::NakedTriple).is_none(),
                "num: {} Col size: {} \n",
                rcb,
                3
            );
            assert!(
                NakedTuple::findtuples(&puz, House::Box(rcb), 2, Technique::NakedDouble).is_none(),
                "num: {} Box size: {} \n",
                rcb,
                2
            );
            assert!(
                NakedTuple::findtuples(&puz, House::Box(rcb), 3, Technique::NakedTriple).is_none(),
                "num: {} Box size: {} \n",
                rcb,
                3
            );
        }
        Ok(())
    }

    #[test]
    fn naked_quad() -> Result<(), Box<dyn Error>> {
        let inp =
            ".1.72.563.56.3.247732546189693287415247615938581394........2...........1..587....";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = NakedTuple::findtuples(&puz, House::Row(7), 4, Technique::NakedQuad);
        assert!(hint.is_some());
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        // let cvec = tmove.changes_vec();

        assert!(
            ivec.contains(&Row::new(7, 0, SudokuSize::Three).to_usize()),
            "{:?}",
            tmove
        );
        assert!(ivec.contains(&Row::new(7, 2, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&Row::new(7, 3, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&Row::new(7, 5, SudokuSize::Three).to_usize()));
        // assert!(cvec.contains(&Change::new(
        //     index_from_row(7, 6),
        //     ChangeType::RemovedPot(3)
        // )));
        // assert!(cvec.contains(&Change::new(
        //     index_from_row(7, 7),
        //     ChangeType::RemovedPot(9)
        // )));
        Ok(())
    }

    #[test]
    fn hidden_not_detected() -> Result<(), Box<dyn Error>> {
        let inp =
            "....6........42736..673..4..94....68....964.76.7.5.9231......85.6..8.271..5.1..94";
        let puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = NakedTuple::findtuples(&puz, House::Box(0), 2, Technique::NakedDouble);
        assert!(hint.is_none());
        let hint = NakedTuple::findtuples(&puz, House::Row(0), 2, Technique::NakedDouble);
        assert!(hint.is_none());
        Ok(())
    }
}
