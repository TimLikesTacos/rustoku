use crate::bitset::*;
use crate::sudokusize::{Res, SudokuSize};

use std::fmt::Debug;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct Possibilities<T: BitSetInt> {
    poss: BitSet<T>,
    original: Option<BitSet<T>>,
    count: u32,
    current: Bit<T>,
}

impl<T: BitSetInt> Possibilities<T> {
    /// Create a new flag based on a known value
    pub fn empty() -> Self {
        Self {
            poss: BitSet::empty(),
            count: 0,
            current: Bit::zero(),
            original: None,
        }
    }

    /// Clears out the flags, but keeps original
    pub fn clear(&self) -> Self {
        Self {
            poss: BitSet::empty(),
            count: 0,
            current: Bit::zero(),
            original: self.original,
        }
    }

    /// Create a flag that has all positions flagged.  Used for empty squares
    pub fn full(size: SudokuSize) -> Res<Self> {
        let full = BitSet::full(size.house_size());
        let count = full.count_u32();

        Ok(Self {
            poss: full,
            count,
            current: Bit::zero(),
            original: None,
        })
    }

    #[allow(dead_code)]
    /// Inserts value into flag.  Nothing changes if value already exists.
    pub fn insert(&mut self, value: Bit<T>) {
        if value != Bit::zero() {
            let newv = self.poss.insert(value);
            if newv != self.poss {
                self.poss = newv;
                self.count += 1;
            }
        }
    }

    /// Inserts value into flag.  Nothing changes if value already exists.
    pub fn insert_multi(&mut self, value: BitSet<T>) {
        if value != BitSet::empty() {
            let newv = self.poss.union(value);
            if newv != self.poss {
                self.poss = newv;
                self.count = self.poss.count_u32();
            }
        }
    }

    /// Removes value from flag.  Nothing happens if value was not present
    pub fn remove(&mut self, value: Bit<T>) {
        if value != Bit::zero() && self.count != 0 {
            let newv = self.poss.remove(value);
            if newv != self.poss {
                self.poss = newv;
                self.count -= 1;
            }
        }
    }
    pub fn set_original(&mut self, size: SudokuSize, remove: BitSet<T>) {
        let full: BitSet<T> = BitSet::full(size.house_size());
        let to_original: BitSet<T> = full.difference(remove);
        self.count = to_original.count_u32();
        self.original = Some(to_original);
        self.poss = to_original;
    }

    #[inline]
    pub fn reset(&mut self) {
        self.poss = self.original.expect("Possiblities have not been set");
        self.current = Bit::zero();
        self.count = self.poss.count_u32();
    }

    #[inline]
    pub fn get(&self) -> &BitSet<T> {
        &self.poss
    }

    pub fn remove_multi(&mut self, flag: BitSet<T>) {
        let res = self.poss.difference(flag);
        self.count = res.count_u32();
        self.poss = res;
        self.current = Bit::zero();
    }

    pub fn as_vec(&self) -> Vec<Bit<T>> {
        self.poss.iter().collect()
    }

    #[inline]
    pub fn poss_contains(&self, other: Bit<T>) -> bool {
        self.poss.contains(other)
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Returns the next possibility in a `Option<SingleFlag>`.  Returns `None` when no more possibilities
    /// are left.  Once `None` is returned, the `currentf` is reset to 0 and can reiterate through the possibilities
    pub fn next(&mut self) -> Option<Bit<T>> {
        // Panic if flags not set.
        if self.original.is_none() {
            panic!("Possiblities have not been set")
        }
        if self.current == Bit::zero() {
            self.current = Bit::one();
            if self.poss.contains(self.current) {
                return Some(self.current);
            }
        }
        loop {
            match self.current.inc() {
                Some(sf) => {
                    if self.poss.contains(sf) {
                        self.current = sf;
                        return Some(sf);
                    }
                    self.current = sf;
                }
                None => {
                    self.current = Bit::zero();
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod possibilities_tests {
    use super::*;

    use crate::sudokusize::Res;

    type Value = u64;
    type ASet = BitSet<Value>;

    #[test]
    fn possibilities_tests() -> Res<()> {
        let mut p: Possibilities<Value> = Possibilities::empty();
        assert_eq!(p.get(), &ASet::empty());
        assert_eq!(p.count(), 0);

        p.insert(3.into());
        assert_eq!(p.get(), &ASet::new().insert(3));
        let four = Bit::from(3).inc().unwrap();
        p.insert(four);
        assert_eq!(p.get(), &ASet::new().insert(4).insert(3));
        assert_eq!(p.count(), 2);
        p.remove(3.into());
        assert_eq!(p.get(), &ASet::new().insert(4));
        assert_eq!(p.count(), 1);
        Ok(())
    }
}
