use crate::bitset::*;
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct BitSet<T: BitSetInt>(pub(crate) T);

impl<T: BitSetInt> BitSet<T> {
    #[inline(always)]
    pub fn new() -> BitSet<T> {
        BitSet(T::zero())
    }

    #[inline(always)]
    pub fn empty() -> BitSet<T> {
        BitSet(T::zero())
    }

    #[inline(always)]
    pub fn from_binary(input: T) -> BitSet<T> {
        BitSet(input)
    }

    #[inline(always)]
    pub fn insert<I: Into<Bit<T>>>(self, to_insert: I) -> BitSet<T> {
        BitSet(self.0 | to_insert.into().0)
    }

    #[inline(always)]
    pub fn remove<I: Into<Bit<T>>>(self, to_remove: I) -> BitSet<T> {
        BitSet(self.0 & !to_remove.into().0)
    }

    #[inline(always)]
    pub fn union(self, other: BitSet<T>) -> BitSet<T> {
        BitSet(self.0 | other.0)
    }

    #[inline(always)]
    pub fn intersect(self, other: BitSet<T>) -> BitSet<T> {
        BitSet(self.0 & other.0)
    }

    #[inline(always)]
    pub fn difference(self, other: BitSet<T>) -> BitSet<T> {
        BitSet(self.0 & !other.0)
    }

    #[inline(always)]
    pub fn disjunct_union(self, other: BitSet<T>) -> BitSet<T> {
        BitSet(self.0 ^ other.0)
    }

    #[inline(always)]
    pub fn is_disjoint(self, other: BitSet<T>) -> bool {
        self.intersect(other).0 == T::zero()
    }

    #[inline(always)]
    pub fn size_u32(&self) -> u32 {
        self.0.count_u32()
    }

    #[inline]
    pub fn size(&self) -> T {
        self.0.count()
    }

    #[inline(always)]
    pub fn contains(self, bit: Bit<T>) -> bool {
        !self.intersect(BitSet(bit.0)).0.is_zero()
    }

    #[inline(always)]
    pub fn shift_right(self) -> BitSet<T> {
        BitSet(self.0 >> T::one())
    }

    #[inline(always)]
    pub fn shift_left(self) -> BitSet<T> {
        BitSet(self.0 << T::one())
    }

    #[inline(always)]
    pub fn iter(&'_ self) -> BitSetIter<'_, T> {
        BitSetIter::new(self)
    }

    #[inline(always)]
    pub fn count_u32(&self) -> u32 {
        self.size_u32()
    }

    #[inline]
    pub fn count(&self) -> T {
        self.size()
    }

    pub fn full(max: usize) -> BitSet<T> {
        let mut value = T::all_ones();
        while value.count_u32() > max as u32 {
            value = value.dec().unwrap_or_else(|| T::zero())
        }
        BitSet(value)
    }
}

impl<T: BitSetInt> From<Bit<T>> for BitSet<T> {
    #[inline(always)]
    fn from(bit: Bit<T>) -> Self {
        BitSet(bit.0)
    }
}

impl<T: BitSetInt> Debug for BitSet<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let vec: Vec<T> = self.iter().map(|bit| bit.into()).collect();
        // Reverse it for better readability. Bitset does not implement DoubleEndedIterator
        let vec: Vec<T> = vec.into_iter().rev().collect();
        write!(f, "{:?}", vec)
    }
}

impl<T: BitSetInt> Default for BitSet<T> {
    fn default() -> Self {
        BitSet::empty()
    }
}

macro_rules! implFrom {
    ($t: ty) => {
        impl From<$t> for BitSet<$t>
        where
            $t: BitSetInt + Into<Bit<$t>>,
        {
            #[inline(always)]
            fn from(value: $t) -> Self {
                BitSet(<Bit<$t>>::from(value).0)
            }
        }
    };
}

implFrom!(u8);
implFrom!(u16);
implFrom!(u32);
implFrom!(u64);
implFrom!(u128);
implFrom!(usize);

#[cfg(test)]
mod bitset_tests {
    use super::*;

    const ITEM1: BitSet<u16> = BitSet(0b1111_0000);
    const ITEM2: BitSet<u16> = BitSet(0b1010_1010);

    #[test]
    fn inter() {
        assert_eq!(ITEM1.intersect(ITEM2), BitSet(0b1010_0000));
    }

    #[test]
    fn union() {
        assert_eq!(ITEM1.union(ITEM2), BitSet(0b1111_1010));
    }

    #[test]
    fn disjunct_union() {
        assert_eq!(ITEM1.disjunct_union(ITEM2), BitSet(0b0101_1010));
    }

    #[test]
    fn is_disjoint() {
        assert!(!ITEM1.is_disjoint(ITEM2));
        let item3 = BitSet(0b0000_1111);
        assert!(ITEM1.is_disjoint(item3));
    }

    #[test]
    fn insert() {
        assert_eq!(ITEM1.insert(1), BitSet(0b1111_0001));
        assert_eq!(ITEM1.insert(Bit(0b1)), BitSet(0b1111_0001));
        assert_eq!(ITEM1.insert(3), BitSet(0b1111_0100));
        assert_eq!(ITEM1.insert(Bit(0b100)), BitSet(0b1111_0100));
        assert_eq!(ITEM1.insert(0), BitSet(0b1111_0000));
        assert_eq!(ITEM1.insert(8), BitSet(0b1111_0000));
    }

    #[test]
    fn remove() {
        assert_eq!(ITEM1.remove(8), BitSet(0b0111_0000));
        assert_eq!(ITEM1.remove(2), BitSet(0b1111_0000));
        assert_eq!(ITEM1.remove(0), BitSet(0b1111_0000));
        assert_eq!(ITEM1.remove(Bit(0b1_0000_0000)), BitSet(0b1111_0000));
    }

    #[test]
    fn count() {
        assert_eq!(ITEM1.size(), 4);
        assert_eq!(ITEM1.insert(2).count(), 5);
        assert_eq!(ITEM1.insert(2).count_u32(), 5u32);
    }

    #[test]
    fn debug() {
        let vec = vec![8, 7, 6, 5];
        let string = format!("{:?}", vec);
        let item1_str = format!("{:?}", ITEM1);
        assert_eq!(item1_str, string);
    }

    #[test]
    fn fulls() {
        let full = <BitSet<u16>>::full(9);
        assert_eq!(full, <BitSet<u16>>::from_binary(0b1_1111_1111u16));
        let full = <BitSet<u16>>::full(4);
        assert_eq!(full, <BitSet<u16>>::from_binary(0b1111u16));
    }
}
