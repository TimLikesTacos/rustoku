use crate::bitset::*;
use crate::grid::gridcoord::HouseCoord;
use crate::grid::house::House;
use crate::grid::{Col, GridBox, Row};
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::human_calcs::technique::Technique;
use crate::human_calcs::tuple_ctr::{Ctr, TupleCtr};
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudokusize::SudokuSize;
use crate::Sudoku;

pub struct BasicFish<V> {
    pub(crate) size: usize,
    phantom: PhantomData<V>,
}

impl<V: BitSetInt> TechStruct for BasicFish<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}

impl<V: BitSetInt> BasicFish<V> {
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
            4 => Technique::Jellyfish,
            3 => Technique::Swordfish,
            2 => Technique::XWing,
            n => Technique::FishN(n),
        };

        let mut vec = Vec::new();
        let dirs = [House::Row(()), House::Col(())];
        for dir in dirs.iter() {
            // If a tuple finds a valid move, stop and return
            if let Some(mut m) = Self::fish(puz, dir, size, solvet) {
                vec.append(&mut m)
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn switch_dir<T: Clone>(house: &House<T>) -> House<T> {
        match house {
            House::Row(x) => House::Col(x.clone()),
            House::Col(x) => House::Row(x.clone()),
            _ => panic!("Not indeded to use box for this solve technique"),
        }
    }

    fn add_to_house(house: &House<()>, rcb: usize) -> House<usize> {
        match house {
            House::Row(()) => House::Row(rcb),
            House::Col(()) => House::Col(rcb),
            _ => panic!("Not indeded to use box for this solve technique"),
        }
    }

    fn index_from_what<T: Clone>(
        house: &House<T>,
        primary: usize,
        secondary: usize,
        size: SudokuSize,
    ) -> usize {
        match house {
            House::Row(_) => Row::new(primary, secondary, size).to_usize(),
            House::Col(_) => Col::new(primary, secondary, size).to_usize(),
            House::Box(_) => GridBox::new(primary, secondary, size).to_usize(),
        }
    }

    fn fish(
        puz: &Sudoku<V>,
        house: &House<()>,
        size: usize,
        method: Technique,
    ) -> Option<Vec<Move<V>>> {
        let max = puz.house_size();
        let mut vec = Vec::new();
        for val in <BitSet<V>>::full(max).iter() {
            // Make a TupleCtr where the values is primary, and the indicies are the secondary that contian the value
            let mut tuplectr: TupleCtr<V> = TupleCtr::new(puz.sudoku_size());
            for (i, primary) in <BitSet<V>>::full(max).iter().enumerate() {
                for (sec_ind, secondary) in puz
                    .grid
                    .enum_house_iter(&Self::add_to_house(house, i), puz.sudoku_size())
                    .enumerate()
                {
                    if secondary.poss().contains(val) {
                        tuplectr.insert(sec_ind, primary.into());
                    }
                }
            }

            if let Some(t) = Self::basic_combo(&tuplectr, size, max) {
                for item in t {
                    let mut themove = Move::new();
                    themove.method = Some(method);
                    let other_dir = Self::switch_dir::<()>(house);
                    let mut secondaries = HashSet::new();
                    for primary in item.value_integer_iter() {
                        for secondary in item.index_iter() {
                            let coord = Self::index_from_what(
                                house,
                                primary - 1,
                                secondary,
                                puz.sudoku_size(),
                            );

                            if puz.grid[coord].poss_contains(val) {
                                themove.add_used_to_solve(puz.upgrade_index(coord), val.into());
                            }
                            secondaries.insert(secondary);
                        }
                    }
                    // Now go through the secondaries.  Remove potential value if the (row) is not in primaries
                    for secondary in secondaries {
                        for (ind, _) in puz
                            .grid
                            .enum_house_iter(
                                &Self::add_to_house(&other_dir, secondary),
                                puz.sudoku_size(),
                            )
                            .enumerate()
                            .filter(|(i, sq)| {
                                sq.poss_contains(val) && !item.contains_index_value(*i)
                            })
                        {
                            let coord = Self::index_from_what(
                                &other_dir,
                                secondary,
                                ind,
                                puz.sudoku_size(),
                            );
                            themove.add_removed_potential(puz.upgrade_index(coord), val.into());
                        }
                    }

                    // Only return a move that makes a change
                    if !themove.removed_potentials_vec().is_empty() {
                        vec.push(themove);
                    }
                }
            }
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn basic_combo(tuples: &TupleCtr<V>, size: usize, max: usize) -> Option<Vec<Ctr<V>>> {
        Self::basic_combo_rec(tuples, size, Ctr::new(Bit::zero()), 0, None, max)
    }

    fn basic_combo_rec(
        tuples: &TupleCtr<V>,
        size: usize,
        current_val: Ctr<V>,
        current_size: usize,
        current_ind: Option<usize>,
        max: usize,
    ) -> Option<Vec<Ctr<V>>> {
        if current_size == size {
            if current_val.ind_count == size as u32 {
                return Some(vec![current_val]);
            }
            return None;
        }

        let mut ind = match current_ind {
            None => 0,
            Some(v) => v + 1,
        };

        if ind == max {
            return None;
        }

        // Skip empty values. Improves average runtime but not worst case
        while tuples[ind].ind_count == 0 {
            ind += 1;
            if ind == max {
                return None;
            }
        }

        let merged = current_val.merge(&tuples[ind]);

        let mut vec = Vec::new();

        // Merged
        if let Some(mut t) =
            Self::basic_combo_rec(tuples, size, merged, current_size + 1, Some(ind), max)
        {
            vec.append(&mut t);
            //return Some(t);
        }
        // Not merged
        if let Some(mut t) =
            Self::basic_combo_rec(tuples, size, current_val, current_size, Some(ind), max)
        {
            vec.append(&mut t);
        }

        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }
}

#[cfg(test)]
mod basicfish {
    use super::*;
    use crate::Sudoku;
    use std::error::Error;

    //* Tests from http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn xwing() -> Result<(), Box<dyn Error>> {
        let inp =
            ".41729.3.769..34.2.3264.7194.39..17.6.7..49.319537..24214567398376.9.541958431267";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Row(()), 2, Technique::XWing);
        let tmove = hint.as_ref().unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let inv: [usize; 4] = [13, 16, 40, 43];
        for i in inv.iter() {
            assert!(ivec.contains(i), "{:?}\n{:?}", ivec, &hint);
        }

        Ok(())
    }

    #[test]
    fn xwing2() -> Result<(), Box<dyn Error>> {
        let inp =
            "98..62753.65..3...327.5...679..3.5...5...9...832.45..9673591428249.87..5518.2...7";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Col(()), 2, Technique::XWing);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 4] = [9, 13, 36, 40];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }

        assert_eq!(cvec.len(), 9, "{:?}", &cvec);
        assert_eq!(ivec.len(), 4, "{:?}", &ivec);
        Ok(())
    }

    #[test]
    fn swordfish() -> Result<(), Box<dyn Error>> {
        let inp =
            "16.543.7..786.14354358.76.172.458.696..912.57...376..4.16.3..4.3...8..16..71645.3";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Row(()), 3, Technique::Swordfish);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 6] = [9, 13, 22, 25, 72, 79];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }

        assert_eq!(cvec.len(), 2, "{:?}", &tmove);
        assert_eq!(ivec.len(), 6, "{:?}", &tmove);
        Ok(())
    }

    #[test]
    fn swordfish2() -> Result<(), Box<dyn Error>> {
        let inp =
            "1.85..2345..3.2178...8..5698..6.5793..59..4813....865298.2.631.......8.....78.9..";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Row(()), 3, Technique::Swordfish);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 8] = [10, 11, 13, 28, 29, 31, 56, 58];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }

        assert_eq!(cvec.len(), 11, "{:?}", &tmove);
        assert_eq!(ivec.len(), 8, "{:?}", &tmove);
        Ok(())
    }

    #[test]
    fn jellyfish() -> Result<(), Box<dyn Error>> {
        let inp =
            "2.......3.8..3..5...34.21....12.54......9......93.86....25.69...9..2..7.4.......1";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Row(()), 4, Technique::Jellyfish);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 15] = [18, 19, 22, 26, 27, 28, 31, 35, 45, 46, 49, 53, 54, 55, 58];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }

        assert_eq!(cvec.len(), 9, "{:?}", &tmove);
        assert_eq!(ivec.len(), 15, "{:?}", &tmove);
        Ok(())
    }

    #[test]
    fn jellyfish2() -> Result<(), Box<dyn Error>> {
        let inp =
            "2.41.358.....2.3411.34856..732954168..5.1.9..6198324....15.82..3..24.....263....4";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = BasicFish::fish(&puz, &House::Row(()), 4, Technique::Jellyfish);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        // let cvec = tmove.changes_vec();
        let inv: [usize; 10] = [1, 4, 8, 19, 25, 26, 52, 53, 55, 58];
        for i in inv.iter() {
            assert!(ivec.contains(i), "{:?} \n themove: {:?}", &ivec, tmove);
        }
        // let chg: [usize; 2] = [10, 76];
        // for c in chg.iter() {
        //     assert!(
        //         cvec.contains(&Change::new(*c, ChangeType::RemovedPot(7))),
        //         "{}",
        //         c
        //     );
        // }
        // This puzzle contains for than two changes, as opposed to what the webiste that I pulled the
        // example comes from.  The website assumes other solving techniques have been performed
        //assert_eq!(cvec.len(), 2, "{:?}", &tmove);
        //assert_eq!(ivec.len(), 10, "{:?}", &tmove);
        Ok(())
    }
}
