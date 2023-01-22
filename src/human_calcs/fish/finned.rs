use std::collections::HashSet;
use std::marker::PhantomData;

use crate::bitset::*;
use crate::grid::gridcoord::HouseCoord;
use crate::grid::house::House;
use crate::grid::{Col, GridBox, GridCoord, Index, Row};
use crate::human_calcs::fish::fishtuple::{FishCtr, FishTupleCtr};
use crate::human_calcs::technique::Technique;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudokusize::SudokuSize;
use crate::Sudoku;

pub struct FinnedFish<V> {
    pub(crate) size: usize,
    phantom: PhantomData<V>,
}

impl<V: BitSetInt> TechStruct for FinnedFish<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        self._hint(puz).map(|v| v[0].clone())
    }
}

#[derive(Debug)]
struct FinCand<T: BitSetInt> {
    sets: FishCtr<T>,
    extra: FishCtr<T>,
}

impl<V: BitSetInt> FinnedFish<V> {
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
            4 => Technique::FinnedJellyfish,
            3 => Technique::FinnedSwordfish,
            2 => Technique::FinnedXWing,
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

    fn add_to_house(house: &House<()>, rcb: usize) -> House<usize> {
        match house {
            House::Row(()) => House::Row(rcb),
            House::Col(()) => House::Col(rcb),
            _ => panic!("Not intended to use box for this solve technique"),
        }
    }

    fn from_puzzle(puz: &Sudoku<V>, house: &House<()>, value: Bit<V>) -> FishTupleCtr<V> {
        let mut tuplectr = FishTupleCtr::new(puz.sudoku_size());
        for (primary, prim_bit_offset) in <BitSet<V>>::full(puz.house_size()).iter().enumerate() {
            for (sec_ind, secondary) in puz
                .grid
                .enum_house_iter(&Self::add_to_house(house, primary), puz.sudoku_size())
                .enumerate()
            {
                if secondary.poss().contains(value) {
                    tuplectr.insert(sec_ind, prim_bit_offset.into());
                }
            }
        }
        tuplectr
    }

    fn index_from_what<C: Clone>(
        house: &House<C>,
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

    #[inline]
    fn switch_dir<H: Clone>(house: &House<H>) -> House<H> {
        match house {
            House::Row(x) => House::Col(x.clone()),
            House::Col(x) => House::Row(x.clone()),
            _ => panic!("Not intended to use box for this solve technique"),
        }
    }

    fn populate_secondary_set_with_potentials<T: BitSetInt>(
        puz: &Sudoku<T>,
        secondary_house: &House<usize>,
        map: &mut HashSet<usize>,
    ) {
        let iter = puz.grid.enum_house_iter(secondary_house, puz.sudoku_size());
        for sq in iter {
            map.insert(sq.index());
        }
    }

    pub(crate) fn fish(
        puz: &Sudoku<V>,
        dir: &House<()>,
        size: usize,
        method: Technique,
    ) -> Option<Vec<Move<V>>> {
        // A finned fish is just a basic fish with one primary set having an extra value.
        let mut vec = Vec::new();
        for val in <BitSet<V>>::full(puz.house_size()).iter() {
            // Make a TupleCtr where the potentials are primary, and the indices are the secondary that contain the value
            let tuplectr = Self::from_puzzle(puz, dir, val);

            if let Some(t) = Self::finned_combo(&tuplectr, size, &puz.sudoku_size()) {
                let mut themove = Move::new();
                themove.method = Some(method);

                let extraind = Self::index_from_what(
                    dir,
                    t.extra.primary_iter().next().unwrap(),
                    t.extra.index_iter().next().unwrap(),
                    puz.sudoku_size(),
                );
                let extra_index = Index::new(extraind, puz.sudoku_size());
                let boxnum = extra_index.to_box();

                let mut involved_ind = HashSet::new();
                let mut secondaries: HashSet<usize> = HashSet::new();

                // Add sets to involved list in the move.
                for primary in t.sets.primary_iter() {
                    for secondary in t.sets.index_iter() {
                        let coord =
                            Self::index_from_what(dir, primary, secondary, puz.sudoku_size());
                        if puz.grid[coord].poss_contains(val) {
                            themove.add_used_to_solve(puz.upgrade_index(coord), val.into());
                        }
                        involved_ind.insert(coord);
                        Self::populate_secondary_set_with_potentials(
                            puz,
                            &Self::add_to_house(&Self::switch_dir(dir), secondary),
                            &mut secondaries,
                        )
                    }
                }

                // Add the extra to the move.
                for primary in t.extra.primary_iter() {
                    for secondary in t.extra.index_iter() {
                        let coord =
                            Self::index_from_what(dir, primary, secondary, puz.sudoku_size());
                        if puz.grid[coord].poss_contains(val) {
                            themove.add_used_to_solve(puz.upgrade_index(coord), val.into());
                        }
                        involved_ind.insert(Self::index_from_what(
                            dir,
                            primary,
                            secondary,
                            puz.sudoku_size(),
                        ));
                    }
                }

                // Now go through the secondaries.  Remove potential value if the (row) is not in primaries
                for affected in puz.grid.house_iter(boxnum) {
                    if !involved_ind.contains(&affected.index())
                        && affected.poss_contains(val)
                        && secondaries.contains(&affected.index())
                    {
                        themove
                            .add_removed_potential(puz.upgrade_index(affected.index()), val.into());
                    }
                }

                // Only return a move that makes a change
                if !themove.removed_potentials_vec().is_empty() {
                    vec.push(themove);
                }
            }
        }
        if vec.is_empty() {
            None
        } else {
            Some(vec)
        }
    }

    fn finned_combo<T: BitSetInt>(
        tuples: &FishTupleCtr<T>,
        size: usize,
        sudoku_size: &SudokuSize,
    ) -> Option<FinCand<T>> {
        let fin = FinCand {
            sets: FishCtr::new(Bit::zero()),
            extra: FishCtr::new(Bit::zero()),
        };
        Self::finned_combo_rec(tuples, size, fin, 0, None, sudoku_size)
    }

    fn is_extra_covered<T: BitSetInt>(cand: &FinCand<T>, size: &SudokuSize) -> bool {
        // prove assumption
        debug_assert_eq!(cand.extra.icount(), 1);
        let extra: usize = cand.extra.index_iter().next().unwrap();
        cand.sets
            .index_iter()
            .any(|cind| cind / size.size() == extra / size.size())
    }

    fn is_not_overlapped<T: BitSetInt>(fish: &FinCand<T>, size: &SudokuSize) -> bool {
        let lhs = &fish.sets;
        let rhs = &fish.extra;
        let mut once = false;
        for left in lhs.primary_iter() {
            for right in rhs.primary_iter() {
                if (right) / size.size() == (left) / size.size() {
                    if once {
                        return false;
                    }
                    once = true;
                }
            }
        }
        true
    }

    fn finned_combo_rec<T: BitSetInt>(
        tuples: &FishTupleCtr<T>,
        size: usize,
        current_val: FinCand<T>,
        current_size: usize,
        current_ind: Option<usize>,
        sudoku_size: &SudokuSize,
    ) -> Option<FinCand<T>> {
        if current_size == size {
            if current_val.sets.icount() == size as u32
                && current_val.extra.icount() == (size as u32) + 1
            {
                // Remove the values that appear in `sets` from `extra`

                let mut newextra = current_val.extra.merge_disjoint_ind_only(&current_val.sets);

                let theprimary = tuples
                    .primary_from_ind(newextra.indices)
                    .intersect(newextra.primary);
                let count = BitSet::count_u32(&theprimary);
                debug_assert_eq!(count, 1);
                newextra.primary = theprimary;
                newextra.pri_count = count;

                let fin = FinCand {
                    sets: current_val.sets,
                    extra: newextra,
                };
                if Self::is_extra_covered(&fin, sudoku_size)
                    && Self::is_not_overlapped(&fin, sudoku_size)
                {
                    return Some(fin);
                }
            }
            return None;
        }

        let mut ind = match current_ind {
            None => 0,
            Some(v) => v + 1,
        };

        let house_size = sudoku_size.house_size();
        if ind == house_size {
            return None;
        }

        // Skip empty values. Improves average runtime but not worst case
        while tuples[ind].icount() == 0 {
            ind += 1;
            if ind == house_size {
                return None;
            }
        }

        // Extras are values that have occurred once
        // Sets are values that have occurred at least twice
        // To see what values we can add to sets, we intersect the current value with extras.
        // Any value there can be added to sets.
        let twice = current_val
            .extra
            .add_primary_intersect_indicies(&tuples[ind]);

        let newfin = FinCand {
            sets: current_val.sets.merge(&twice),
            extra: current_val.extra.merge(&tuples[ind]),
        };

        // Merged
        if let Some(t) = Self::finned_combo_rec(
            tuples,
            size,
            newfin,
            current_size + 1,
            Some(ind),
            sudoku_size,
        ) {
            return Some(t);
        }
        // Not merged
        if let Some(t) = Self::finned_combo_rec(
            tuples,
            size,
            current_val,
            current_size,
            Some(ind),
            sudoku_size,
        ) {
            return Some(t);
        }
        None
    }
}

#[cfg(test)]
mod finnedfish {
    use std::error::Error;

    use super::*;
    use crate::Sudoku;

    #[test]
    fn test_index_from_what() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            FinnedFish::<u16>::index_from_what::<()>(&House::Row(()), 0, 3, SudokuSize::Three),
            3usize
        );
        assert_eq!(
            FinnedFish::<u16>::index_from_what(&House::Row(()), 3, 3, SudokuSize::Three),
            30
        );

        assert_eq!(
            FinnedFish::<u16>::index_from_what(&House::Col(()), 0, 3, SudokuSize::Three),
            27
        );
        assert_eq!(
            FinnedFish::<u16>::index_from_what(&House::Col(()), 3, 3, SudokuSize::Three),
            30
        );
        Ok(())
    }

    //* Tests from http://hodoku.sourceforge.net/en/tech_fishb.php
    #[test]
    fn fxwing() -> Result<(), Box<dyn Error>> {
        let inp =
            ".5267.3.8.3...562767..325.128...61.5.6....2.4714523869827314956.9.267483346958712";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = FinnedFish::fish(&puz, &House::Row(()), 2, Technique::FinnedXWing);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 5] = [9, 11, 13, 29, 31];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }

        assert_eq!(ivec.len(), 5);
        assert_eq!(cvec.len(), 1);
        Ok(())
    }

    #[test]
    fn swordfish() -> Result<(), Box<dyn Error>> {
        let inp =
            "2.3.1865.416753982.58.26.1.84.362.9562.8.543.53.14.826.652...483.458.26..826.457.";
        let mut puz: Sudoku<u32> = Sudoku::new_with_size(inp, SudokuSize::Three)?;

        let hint = FinnedFish::fish(&puz, &House::Col(()), 3, Technique::FinnedSwordfish);
        let tmove = hint.unwrap()[0].clone().apply(&mut puz);
        let ivec: HashSet<usize> = tmove
            .involved_vec()
            .iter()
            .map(|pair| pair.index().into())
            .collect();
        let cvec = tmove.removed_potentials_vec();
        let inv: [usize; 7] = [8, 18, 26, 40, 44, 54, 58];
        for i in inv.iter() {
            assert!(ivec.contains(i));
        }
        assert_eq!(cvec.len(), 1, "{:?}", &tmove);
        assert_eq!(ivec.len(), 7, "{:?}", &tmove);
        Ok(())
    }
}
