use crate::bitset::{BitSet, BitSetInt};
use crate::grid::house::House;
use crate::human_calcs::basic::SinglePossibility;
use crate::human_calcs::technique::Technique;
use crate::human_calcs::TechStruct;
use crate::move_change::Move;
use crate::sudoku::sudoku::Sudoku;
use std::marker::PhantomData;

impl<V: BitSetInt> TechStruct for SinglePossibility<V> {
    type SquareValue = V;
    fn tech_hint(&self, puz: &Sudoku<V>) -> Option<Move<V>> {
        Self::_hint(puz)
    }
}

impl<V: BitSetInt> SinglePossibility<V> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }

    fn _hint(puz: &Sudoku<V>) -> Option<Move<V>> {
        /*
         * Ones: Bitwise OR, starting from all 0.  If it has been used at least once, it be 1,
         * Multis: Selfassign bitwise or with (Ones(n-1) bitwiseAND current)
         * Ones:  0000
         * Multi: 0000
         * S1  :  0100
         * Ones:  0100
         * Multi: 0000
         * S2:    1010
         * Ones:  1110
         * Multi: 0000
         * S3:    1100
         * Ones:  1110
         * Multi: 1100
         *
         * Single possiblities will be 1's in Ones, but not in Multi
         **/
        for rcb in 0..puz.house_size() {
            let directions = [House::Row(rcb), House::Col(rcb), House::Box(rcb)];
            for dir in directions {
                let it = puz.house_iter(dir.clone());

                let (multi, ones): (BitSet<V>, BitSet<V>) = it.fold(
                    (BitSet::empty(), BitSet::empty()),
                    |(mut mul, mut ones), s| {
                        mul = mul.union(ones.intersect(*s.poss()));
                        ones = ones.union(*s.poss());
                        (mul, ones)
                    },
                );
                let singles = ones.difference(multi);
                if singles == BitSet::empty() {
                    continue;
                }

                let it = puz.house_iter(dir);
                let (ind, vali) = it
                    .map(|sq| (sq.index(), sq.poss().intersect(singles)))
                    .find(|(_, bs)| bs != &BitSet::empty() && bs.count_u32() == 1)
                    .unwrap_or((0, BitSet::empty()));

                if vali == BitSet::empty() {
                    continue;
                }

                let val = vali
                    .iter()
                    .next()
                    .unwrap_or_else(|| panic!("There should be only one value at this point"));

                let mut themove = Move::new();
                themove.set_method(Some(Technique::SinglePossibility));
                themove.set_value(puz.upgrade_index(ind), val.into(), false);
                return Some(themove);
            }
        }
        None
    }
}
#[cfg(test)]
mod singles {
    use super::*;
    use crate::grid::SudokuSize;
    use crate::solution::Solution;
    use std::error::Error;

    #[test]
    fn single_possibility_test() -> Result<(), Box<dyn Error>> {
        let mut puz = <Sudoku<u16>>::new_with_size(
            vec![
                vec![5, 3, 4, 0, 7, 0, 0, 0, 0],
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
            .collect::<Vec<u16>>(),
            SudokuSize::Three,
        )?;

        let _expected: Vec<u16> = (vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ])
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();

        loop {
            if let Some(m) = SinglePossibility::_hint(&puz) {
                m.apply(&mut puz);
            } else {
                break;
            }
        }

        assert_eq!(puz.remaining(), 0);

        let inp =
            "...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9...";
        let mut puz: Sudoku<u16> = Sudoku::new_with_size(inp, SudokuSize::Three)?;
        let mut count = 0;
        loop {
            if let Some(m) = SinglePossibility::_hint(&puz) {
                m.apply(&mut puz);
                count += 1;
            } else {
                break;
            }
        }
        assert!(count > 4);
        match puz.solution() {
            Solution::One(s) => assert!(s.iter().all(|v| *v > 0 && *v <= 9)),
            _ => assert!(false, "{:?}", &puz),
        }

        Ok(())
    }
}
