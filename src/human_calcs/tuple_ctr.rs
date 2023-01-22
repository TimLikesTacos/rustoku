use crate::grid::gridcoord::HouseCoord;

use crate::bitset::*;
use crate::sudokusize::SudokuSize;
use crate::Sudoku;

use std::ops::Deref;

#[derive(Clone, Debug)]
/// Struct used to count occurrences of a value. Internally, indices are offset by one to make
/// them 'starting at 1' based. The Ctr struct will take care of this during input and output.
pub(crate) struct Ctr<T: BitSetInt> {
    pub(crate) values: BitSet<T>,
    pub(crate) indices: BitSet<T>,
    pub(crate) ind_count: u32,
}

impl<T: BitSetInt> Ctr<T> {
    pub(crate) fn new(bit: Bit<T>) -> Ctr<T> {
        Ctr {
            values: bit.into(),
            indices: BitSet::empty(),
            ind_count: 0,
        }
    }

    pub(crate) fn merge(&self, other: &Ctr<T>) -> Ctr<T> {
        let c_ind = self.indices.union(other.indices);
        Ctr {
            values: self.values.union(other.values),
            ind_count: BitSet::count_u32(&c_ind),
            indices: c_ind,
        }
    }

    #[inline]
    /// Do not offset index. This function will take care of that.
    pub(crate) fn insert_index(&mut self, index: usize) {
        //"Inserting indicies should start at 1, not 0.  0 is \
        //indiscernible from empty"
        let old = self.indices;
        let indexbit = <T>::from_usize(index + 1);
        self.indices = self.indices.insert(indexbit);

        if old != self.indices {
            self.ind_count += 1;
        }
    }

    #[inline]
    #[allow(dead_code)]
    /// Do not offset index. This funciton will take care of that. i.e. 0 is a valid index
    pub(crate) fn contains_index(&self, index: usize) -> bool {
        let old = self.indices;
        let indexbit = <T>::from_usize(index + 1);
        old == old.insert(indexbit)
    }

    #[inline]
    /// Do not offset index. This funciton will take care of that. i.e. 0 is a valid index
    pub(crate) fn contains_index_value(&self, index: usize) -> bool {
        let old = self.values;
        let indexbit = <T>::from_usize(index + 1);
        old == old.insert(indexbit)
    }

    #[inline]
    pub(crate) fn index_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = usize> + 'b {
        self.indices
            .iter()
            .map(|bit| <T>::from(bit).try_into().unwrap_or(1) - 1usize)
    }

    #[inline]
    pub(crate) fn value_integer_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = usize> + 'b {
        self.values
            .iter()
            .map(|bit| <T>::from(bit).try_into().unwrap_or(0))
    }

    #[inline]
    pub(crate) fn remove_values(lhs: &Ctr<T>, rhs: &Ctr<T>) -> Ctr<T> {
        Ctr {
            values: lhs.values.difference(rhs.values),
            indices: lhs.indices,
            ind_count: lhs.ind_count,
        }
    }

    #[inline]
    pub(crate) fn remove_indicies(lhs: &Ctr<T>, rhs: &Ctr<T>) -> Ctr<T> {
        let diff = lhs.indices.difference(rhs.indices);
        Ctr {
            values: lhs.values,
            indices: diff,
            ind_count: BitSet::count_u32(&diff),
        }
    }
}

impl<T: BitSetInt> PartialEq for Ctr<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.indices == other.indices
    }
}

pub(crate) struct CtrIndexIterator<T: BitSetInt> {
    indicies: BitSet<T>,
    current: Option<Bit<T>>,
}

impl<T: BitSetInt> Iterator for CtrIndexIterator<T> {
    type Item = Bit<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let zero: BitSet<T> = <BitSet<T>>::empty();
        let _one = <Bit<T>>::one();

        //let to_return = self.current;
        self.current = self.current.unwrap_or_else(|| <Bit<T>>::zero()).inc();
        while let Some(current) = self.current {
            if self.indicies.difference(current.into()) > zero {
                return self.current;
            }
            if <BitSet<T>>::from(current) > self.indicies {
                self.current = None;
                return None;
            }
            self.current = current.inc();
        }
        None
    }
}

/// Struct used to count values and occurances.  Contains a vector of size MAX_NUM (9 for
/// a typical 9x9 sudoku puzzle. The vector stores Ctr types.  NOTE:: Ctr stores indicies, these
/// must be begin at 1 based, since a index of 0 is indiscernible from empty.
#[derive(Clone, Debug)]
pub(crate) struct TupleCtr<T: BitSetInt> {
    pub(crate) array: Vec<Ctr<T>>,
    _size: SudokuSize,
}

impl<T: BitSetInt> Deref for TupleCtr<T> {
    type Target = Vec<Ctr<T>>;

    fn deref(&self) -> &Self::Target {
        &self.array
    }
}
impl<T: BitSetInt> TupleCtr<T> {
    /// Sets up an new tuple counter with an array of MAX_NUM length.  Each element
    /// has a single flag value. 0b0001, 0b0010, 0b0100, ect.
    pub fn new(size: SudokuSize) -> TupleCtr<T> {
        let mut res = TupleCtr {
            array: Vec::with_capacity(size.house_size()),
            _size: size,
        };

        let mut f = Some(<Bit<T>>::one());
        for _ in 0..size.house_size() {
            if let Some(item) = f {
                res.array.push(Ctr::new(item));
                f = item.inc();
            }
        }
        res
    }

    pub fn insert(&mut self, index: usize, flag: BitSet<T>) {
        let zero = BitSet::empty();
        for val in &mut self.array {
            if flag.intersect(val.values) != zero {
                val.insert_index(index);
            }
        }
    }

    fn combo_fn(&self, tuple_size: usize, diff_fn: fn(&Ctr<T>, &Ctr<T>) -> Ctr<T>) -> Vec<Ctr<T>> {
        let mut results: Vec<Ctr<T>> = Vec::new();
        self.combo_rec_fn(
            &mut results,
            diff_fn,
            Ctr::new(Bit::zero()),
            Ctr::new(Bit::zero()),
            None,
            0,
            tuple_size,
        );
        results
    }
    // Recursive function to get tuples
    #[allow(clippy::too_many_arguments)]
    fn combo_rec_fn(
        &self,
        tups: &mut Vec<Ctr<T>>,
        diff_fn: fn(&Ctr<T>, &Ctr<T>) -> Ctr<T>,
        lhs: Ctr<T>,
        rhs: Ctr<T>,
        current_ind: Option<usize>,
        current_tuple: usize,
        tuple_size: usize,
    ) {
        //Increment current_ind
        let mut ind = match current_ind {
            None => 0,
            Some(v) => v + 1,
        };
        // Keep moving so that flag with no indicies are not added
        loop {
            if ind >= self.array.len() {
                // One last check for the previously merged ctr.
                break;
            }
            // merging flags without indicies will cause errors
            if self.array[ind].ind_count != 0 {
                break;
            }
            ind += 1;
        }

        // Potential tuple of appropriate size calculated.  Determine if it is a tuple.
        if current_tuple == tuple_size {
            // Tuples where the only option is to be a tuple is not needed for counting
            let mut r_rhs = rhs;
            // add the remaining in the array to the rhs.
            for i in ind..self.array.len() {
                r_rhs = r_rhs.merge(&self.array[i]);
            }
            if r_rhs.indices == BitSet::empty() {
                return;
            }

            let diff = diff_fn(&lhs, &r_rhs);
            // diff will be equal to lhs if all the elements in lhs are not in rhs.
            if diff.ind_count == tuple_size as u32 {
                tups.push(diff);
            }
            return;
        }
        if ind >= self.array.len() {
            return;
        }

        let added_to_left = self.array[ind].merge(&lhs);
        let added_to_right = self.array[ind].merge(&rhs);

        // continue with building tuple
        // Add to left
        self.combo_rec_fn(
            tups,
            diff_fn,
            added_to_left,
            rhs,
            Some(ind),
            current_tuple + 1,
            tuple_size,
        );

        // A tuple has been met, so start with the original values and increment the index;
        self.combo_rec_fn(
            tups,
            diff_fn,
            lhs,
            added_to_right,
            Some(ind),
            current_tuple,
            tuple_size,
        );
    }

    pub fn find_tuples(
        puz: &Sudoku<T>,
        house: impl HouseCoord,
        merge_fn: fn(&Ctr<T>, &Ctr<T>) -> Ctr<T>,
        tuple_size: usize,
    ) -> Vec<Ctr<T>> {
        let it = puz.grid.house_iter(house);

        let mut counter: TupleCtr<T> = TupleCtr::new(puz.sudoku_size());
        for (i, s) in it.enumerate() {
            counter.insert(i, *s.poss());
        }

        counter.combo_fn(tuple_size, merge_fn)
        // let mut results: Vec<Ctr> = Vec::new();
        // for uple in 2..(MAX_NUM / 2) {
        //     results.append(&mut counter.combo_fn(uple, merge_fn));
        // }
        // results
    }
}

impl<T: BitSetInt> IntoIterator for TupleCtr<T> {
    type Item = Ctr<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

#[cfg(test)]
mod ctr_tests {
    use super::*;

    #[test]
    fn ctr_test() {
        let mut c: TupleCtr<u16> = TupleCtr::new(SudokuSize::Three);

        for (i, v) in c.array.iter().enumerate() {
            let val: u16 = 1 << i as u16;
            assert_eq!(v.values, BitSet::from_binary(val));
            assert_eq!(v.indices, BitSet::empty())
        }

        c.insert(4, BitSet::from_binary(0b1101));

        for (i, v) in c.into_iter().enumerate() {
            if i == 0 || i == 2 || i == 3 {
                assert_eq!(
                    v.indices.intersect(<Bit<u16>>::try_from(5).unwrap().into()),
                    BitSet::from_binary(0b10000)
                );
                assert_eq!(v.ind_count, 1);
            } else {
                assert_eq!(
                    v.indices.intersect(<Bit<u16>>::from(4).into()),
                    BitSet::from_binary(0)
                );
                assert_eq!(v.ind_count, 0);
            }
        }
    }

    #[test]
    fn insert_and_contain() {
        let mut ctr: Ctr<u32> = Ctr::new(Bit::one());
        ctr.insert_index(2);
        assert_eq!(ctr.indices, BitSet::from_binary(0b100));
        assert!(ctr.contains_index(2));
        assert!(!ctr.contains_index(1));
        assert!(!ctr.contains_index(3));
    }
    #[test]
    fn combo_test() {
        let one = BitSet::from_binary(1u16);
        let two = BitSet::from_binary(0b10u16);
        let three = BitSet::from_binary(0b100u16);
        let four = BitSet::from_binary(0b1000u16);
        let five = BitSet::from_binary(0b1_0000_u16);
        let six = BitSet::from_binary(0b10_0000_u16);
        let seven = BitSet::from_binary(0b100_0000_u16);
        let eight = BitSet::from_binary(0b1000_0000_u16);
        let nine = BitSet::from_binary(0b1_0000_0000_u16);

        let array: [BitSet<u16>; 9] = [one, two, three, four, five, six, seven, eight, nine];
        let mut the_counter = TupleCtr::new(SudokuSize::Three);
        for (i, x) in array.iter().enumerate() {
            the_counter.insert(i, *x);
        }

        let res = the_counter.combo_fn(1, Ctr::remove_indicies);

        assert_eq!(res.len(), 9);
        let res = the_counter.combo_fn(2, Ctr::remove_indicies);
        assert_eq!(res.len(), 36);
        let res = the_counter.combo_fn(3, Ctr::remove_indicies);
        assert_eq!(res.len(), 84);
        // for (i, x) in array.iter().enumerate() {
        //     //let c = if i + 1 > 9 { 9 } else { i + 1 };
        //     the_counter.insert(i, *x);
        // }

        the_counter = TupleCtr::new(SudokuSize::Three);
        the_counter.insert(0, one);
        the_counter.insert(1, two);
        let res = the_counter.combo_fn(1, Ctr::remove_indicies);
        assert_eq!(res.len(), 2);
        let res = the_counter.combo_fn(2, Ctr::remove_indicies);
        // Should have no 2-uples yet
        assert_eq!(res.len(), 0);
        the_counter.insert(0, two);
        the_counter.insert(1, one);
        the_counter.insert(8, nine);
        the_counter.insert(8, one);
        the_counter.insert(8, two);

        let res = the_counter.combo_fn(2, Ctr::remove_indicies);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].values, BitSet::from_binary(0b11));
        assert_eq!(res[0].indices, BitSet::from_binary(0b11));
        assert_eq!(res[0].ind_count, 2);
        the_counter.insert(7, eight);
        the_counter.insert(7, one);
        the_counter.insert(7, two);
        let res = the_counter.combo_fn(2, Ctr::remove_indicies);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].values, BitSet::from_binary(0b11));
        assert_eq!(res[0].indices, BitSet::from_binary(0b11));
        assert_eq!(res[0].ind_count, 2);

        the_counter.insert(6, eight);
        the_counter.insert(8, seven);
        the_counter.insert(7, nine);
        the_counter.insert(6, seven);

        let res = the_counter.combo_fn(2, Ctr::remove_indicies);

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].values, BitSet::from_binary(0b11));
        assert_eq!(res[0].indices, BitSet::from_binary(0b11));
        assert_eq!(res[0].ind_count, 2);
        let res = the_counter.combo_fn(3, Ctr::remove_indicies);

        assert_eq!(res.len(), 0);
        let res = the_counter.combo_fn(4, Ctr::remove_indicies);

        assert_eq!(res.len(), 0);
    }
}
