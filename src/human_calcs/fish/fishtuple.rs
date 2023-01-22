use crate::bitset::*;
use crate::grid::SudokuSize;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub(super) struct FishCtr<T: BitSetInt> {
    pub(super) primary: BitSet<T>,
    pub(super) pri_count: u32,
    pub(super) indices: BitSet<T>,
    pub(super) ind_count: u32,
}

impl<T: BitSetInt> FishCtr<T> {
    pub(super) fn new(val: Bit<T>) -> FishCtr<T> {
        let bs: BitSet<T> = val.into();
        FishCtr {
            primary: bs,
            pri_count: BitSet::count_u32(&bs),
            indices: BitSet::empty(),
            ind_count: 0,
        }
    }

    pub(crate) fn icount(&self) -> u32 {
        self.ind_count
    }

    pub fn merge(&self, other: &FishCtr<T>) -> FishCtr<T> {
        let c_ind = self.indices.union(other.indices);
        let p = self.primary.union(other.primary);
        FishCtr {
            primary: p,
            pri_count: BitSet::count_u32(&p),
            indices: c_ind,
            ind_count: BitSet::count_u32(&c_ind),
        }
    }

    pub fn merge_disjoint_ind_only(&self, other: &FishCtr<T>) -> FishCtr<T> {
        let c_ind = self.indices.difference(other.indices);

        FishCtr {
            primary: self.primary,
            pri_count: self.pri_count,
            indices: c_ind,
            ind_count: BitSet::count_u32(&c_ind),
        }
    }

    pub fn add_primary_intersect_indicies(&self, other: &FishCtr<T>) -> FishCtr<T> {
        let c_ind = self.indices.intersect(other.indices);
        let p = self.primary.union(other.primary);
        FishCtr {
            primary: p,
            pri_count: BitSet::count_u32(&p),
            indices: c_ind,
            ind_count: BitSet::count_u32(&c_ind),
        }
    }

    #[inline]
    /// Do not offset index. This function will take care of that.
    pub fn insert_index(&mut self, index: usize) {
        //"Inserting indices should start at 1, not 0.  0 is \
        //indiscernible from empty"
        let old = self.indices;
        let indexbit = <T>::from_usize(index + 1);
        self.indices = self.indices.insert(indexbit);

        if old != self.indices {
            self.ind_count += 1;
        }
    }

    #[inline]
    pub fn index_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = usize> + 'b {
        self.indices
            .iter()
            .map(|bit| <T>::from(bit).try_into().unwrap_or(1) - 1)
    }

    pub fn primary_iter<'a, 'b: 'a>(&'b self) -> impl Iterator<Item = usize> + 'a {
        self.primary
            .iter()
            .map(|v| <T>::from(v).try_into().unwrap_or(1) - 1)
    }
}

impl<T: BitSetInt> PartialEq for FishCtr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.primary == other.primary && self.indices == other.indices
    }
}
//
// pub(crate) struct FishCtrIterator {
//     indicies: BitSet<T>,
//     current: Option<usize>,
// }
//
// impl Iterator for FishCtrIterator {
//     type Item = usize;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let zero = bs!(0);
//         let one = bs!(0b1);
//
//         while self.indicies != zero {
//             let new = match self.current {
//                 None => 0usize,
//                 Some(i) => i + 1,
//             };
//             self.current = Some(new);
//             if self.indicies.intersect(one) == one {
//                 self.indicies = self.indicies >> one;
//                 return self.current;
//             }
//             self.indicies = self.indicies >> one;
//         }
//         None
//     }
// }

/// Struct used to count values and occurances.  Contains a vector of size MAX_NUM (9 for
/// a typical 9x9 sudoku puzzle. The vector stores Ctr types.  NOTE:: Ctr stores indicies, these
/// must be begin at 1 based, since a index of 0 is indiscernible from empty.
#[derive(Clone, Debug)]
pub(super) struct FishTupleCtr<T: BitSetInt> {
    pub(super) array: Vec<FishCtr<T>>,
}

impl<T: BitSetInt> Deref for FishTupleCtr<T> {
    type Target = Vec<FishCtr<T>>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
impl<T: BitSetInt> FishTupleCtr<T> {
    /// Sets up an new tuple couter with an array of MAX_NUM length.  Each element
    /// has a single flag value. 0b0001, 0b0010, 0b0100, ect.
    pub(super) fn new(size: SudokuSize) -> FishTupleCtr<T> {
        let mut res = FishTupleCtr {
            array: Vec::with_capacity(size.house_size()),
        };
        for v in <BitSet<T>>::full(size.house_size()).iter() {
            res.array.push(FishCtr::new(v));
        }
        res
    }

    // pub(super) fn from_puzzle(puz: &Sudoku, dir: GroupType, value: usize) -> FishTupleCtr {
    //     let mut tuplectr = Self::new();
    //     for primary in 0..MAX_NUM {
    //         for (sec_ind, secondary) in puz.grid.dir_iter(dir)(primary).enumerate() {
    //             if secondary.poss().contains(&SingleBit::new(value as u8)) {
    //                 tuplectr.insert(sec_ind, (primary + 1).into());
    //             }
    //         }
    //     }
    //     tuplectr
    // }

    pub(super) fn insert(&mut self, index: usize, flag: BitSet<T>) {
        let zero = BitSet::empty();
        for val in &mut self.array {
            if flag.intersect(val.primary) != zero {
                val.insert_index(index);
            }
        }
    }

    pub(super) fn primary_from_ind(&self, indicies: BitSet<T>) -> BitSet<T> {
        self.ctr_iter().fold(BitSet::empty(), |acc, x| {
            if x.indices.intersect(indicies) != BitSet::empty() {
                acc.union(x.primary)
            } else {
                acc
            }
        })
    }

    pub(super) fn ctr_iter<'a, 'b: 'a>(&'b self) -> impl Iterator<Item = &'a FishCtr<T>> + 'a {
        self.array.iter()
    }
}

impl<T: BitSetInt> IntoIterator for FishTupleCtr<T> {
    type Item = FishCtr<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}
