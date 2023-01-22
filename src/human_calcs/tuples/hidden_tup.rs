use crate::bitset::*;
use crate::grid::house::House;
use crate::grid::{Col, GridBox, Row};
use crate::human_calcs::technique::Technique;
use crate::human_calcs::tuple_ctr::{Ctr, TupleCtr};
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudoku::sudoku::Sudoku;
use std::collections::HashSet;
use std::marker::PhantomData;

pub struct HiddenTuple<V> {
    pub(crate) size: usize,
    phantom: PhantomData<V>,
}
impl<V: BitSetInt> TechStruct for HiddenTuple<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}

impl<V: BitSetInt> HiddenTuple<V> {
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
        if size < 2 || size > max / 2 {
            return None;
        }
        let solvet = match size {
            4 => Technique::HiddenQuad,
            3 => Technique::HiddenTriple,
            2 => Technique::HiddenDouble,
            n => Technique::HiddenTuple(n),
        };
        for rcb in 0..max {
            let dirs = [House::Row(rcb), House::Col(rcb), House::Box(rcb)];
            for dir in dirs.iter() {
                // If a tuple finds a valid move, stop and return
                if let Some(m) = Self::findtuples(puz, dir, size, solvet) {
                    return Some(m);
                }
            }
        }
        None
    }

    fn findtuples(
        puz: &Sudoku<V>,
        house: &House<usize>,
        tuple_size: usize,
        method: Technique,
    ) -> Option<Vec<Move<V>>> {
        let tups = match house {
            House::Row(n) => TupleCtr::find_tuples(
                puz,
                Row::new(*n, 0, puz.sudoku_size()),
                Ctr::remove_values,
                tuple_size,
            ),
            House::Col(n) => TupleCtr::find_tuples(
                puz,
                Col::new(*n, 0, puz.sudoku_size()),
                Ctr::remove_values,
                tuple_size,
            ),
            House::Box(n) => TupleCtr::find_tuples(
                puz,
                GridBox::new(*n, 0, puz.sudoku_size()),
                Ctr::remove_values,
                tuple_size,
            ),
        };

        let mut it = tups
            .iter()
            .filter(|t| t.values.count_u32() == t.ind_count && t.ind_count != 0);

        let mut vec = Vec::new();

        loop {
            // Hidden tuples are where the number of indices equals the number of values.
            if let Some(res) = it.next() {
                let mut themove = Move::new();
                themove.method = Some(method);

                // Hidden flag will have the `involved` values in the same square where potential
                // values will be removed.

                let indices: HashSet<usize> = res.index_iter().collect();
                for (i, sq) in puz
                    .grid
                    .enum_house_iter(house, puz.sudoku_size())
                    .enumerate()
                {
                    if indices.contains(&i) {
                        themove.add_used_to_solve(
                            puz.upgrade_index(sq.index()),
                            res.values.intersect(*sq.poss()),
                        );
                        let rm = sq.poss().difference(res.values);
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
mod hidden_tup {
    use super::*;

    use crate::grid::gridcoord::HouseCoord;
    use crate::grid::SudokuSize;
    use crate::human_calcs::technique::Technique::HiddenDouble;
    use bitset::BitSet;
    use std::collections::HashSet;
    use std::error::Error;

    #[test]
    fn hidden_tuple1() -> Result<(), Box<dyn Error>> {
        let inp =
            ".49132....81479...327685914.96.518...75.28....38.46..5853267...712894563964513...";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        //Check initial conditions
        assert_eq!(
            puz.grid[Col::new(8, 4, SudokuSize::Three)].poss(),
            &BitSet::from_binary(0b1_0010_0001)
        );
        assert_eq!(
            puz.grid[Col::new(8, 6, SudokuSize::Three)].poss(),
            &BitSet::from_binary(0b1_0000_0001)
        );
        let hint = HiddenTuple::findtuples(&puz, &House::Col(8), 2, Technique::HiddenDouble);
        assert!(hint.is_some());

        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let changes = tmove.removed_potentials_vec();
        assert_eq!(changes.len(), 1);

        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(ivec.contains(&Col::new(8, 4, SudokuSize::Three).to_usize()));
        assert!(ivec.contains(&Col::new(8, 6, SudokuSize::Three).to_usize()));

        assert_eq!(
            puz.grid[Col::new(8, 4, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b1_0000_0001)
        );
        Ok(())
    }

    #[test]
    fn hidden_tuple2() -> Result<(), Box<dyn Error>> {
        let inp =
            "....6........42736..673..4..94....68....964.76.7.5.9231......85.6..8.271..5.1..94";

        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        //Check initial conditions
        assert_eq!(
            puz.grid[GridBox::new(0, 0, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b1_1101_1110)
        );
        assert_eq!(
            puz.grid[GridBox::new(0, 1, SudokuSize::Three).to_usize()].poss(),
            &BitSet::from_binary(0b0_1101_1111)
        );
        let hint = HiddenTuple::findtuples(&puz, &House::Box(0), 2, Technique::HiddenDouble);
        assert!(hint.is_some());

        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let changes = tmove.removed_potentials_vec();
        assert_eq!(changes.len(), 2);

        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(ivec.contains(&0));
        assert!(ivec.contains(&1));

        assert_eq!(puz.grid[1].poss(), &BitSet::from_binary(0b100_1000));

        // Same as row
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = HiddenTuple::findtuples(&puz, &House::Row(0), 2, Technique::HiddenDouble);
        assert!(hint.is_some());

        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let changes = tmove.removed_potentials_vec();
        assert_eq!(changes.len(), 2);

        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(ivec.contains(&0));
        assert!(ivec.contains(&1));

        assert_eq!(puz.grid[1].poss(), &BitSet::from_binary(0b100_1000));
        Ok(())
    }

    #[test]
    fn hidden_double3() -> Result<(), Box<dyn Error>> {
        let inp =
            ".3.....1...8.9....4..6.8......57694....98352....124...276..519....7.9....95...47.";
        let puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = HiddenTuple::findtuples(&puz, &House::Row(0), 2, HiddenDouble);
        assert_eq!(puz.grid[6].poss(), &BitSet::from_binary(0b1110_0010));
        assert_eq!(puz.grid[8].poss(), &BitSet::from_binary(0b1_1111_1010));

        assert!(hint.is_none());

        Ok(())
    }

    #[test]
    fn hidden_triple1() -> Result<(), Box<dyn Error>> {
        let inp =
            "28....473534827196.71.34.8.3..5...4....34..6.46.79.31..9.2.3654..3..9821....8.937";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = HiddenTuple::findtuples(&puz, &House::Box(6), 3, Technique::HiddenTriple);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ind1 = GridBox::new(6, 7, SudokuSize::Three).to_usize();
        let ind2 = GridBox::new(6, 8, SudokuSize::Three).to_usize();
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        assert!(ivec.contains(&ind1));
        assert!(ivec.contains(&ind2));
        assert_eq!(ivec.len(), 3);
        let cvec = tmove.removed_potentials_vec();
        assert_eq!(cvec.len(), 2, "{:?}", tmove);

        assert_eq!(tmove.technique(), &Some(Technique::HiddenTriple));
        Ok(())
    }

    #[test]
    fn hidden_triple2() -> Result<(), Box<dyn Error>> {
        let inp =
            "5..62..37..489........5....93........2....6.57.......3.....9.........7..68.57...2";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = HiddenTuple::findtuples(&puz, &House::Col(5), 3, Technique::HiddenTriple);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec = tmove.involved_vec();
        let cvec = tmove.removed_potentials_vec();
        assert_eq!(ivec.len(), 3);
        assert_eq!(
            cvec.len(),
            3,
            "involved: {:?}, \n\nchanges: {:?}",
            ivec,
            cvec
        );
        assert_eq!(tmove.technique(), &Some(Technique::HiddenTriple));
        Ok(())
    }

    #[test]
    fn hidden_quad() -> Result<(), Box<dyn Error>> {
        let inp =
            ".3.....1...8.9....4..6.8......57694....98352....124...276..519....7.9....95..247.";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let hint = HiddenTuple::findtuples(&puz, &House::Col(8), 4, Technique::HiddenQuad);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec = tmove.involved_vec();
        let cvec = tmove.removed_potentials_vec();
        assert_eq!(ivec.len(), 4);
        assert_eq!(cvec.len(), 4);
        assert_eq!(tmove.technique(), &Some(Technique::HiddenQuad));
        Ok(())
    }
}
