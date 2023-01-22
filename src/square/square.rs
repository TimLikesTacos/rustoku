use crate::bitset::*;
use crate::square::possibilities::*;
use crate::sudokusize::*;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Square<Value: BitSetInt> {
    pub(crate) value: Option<Bit<Value>>,
    fixed: bool,
    pub(crate) flags: Possibilities<Value>,
    pub(crate) index: usize,
}

impl<Value: BitSetInt> Square<Value> {
    pub(crate) fn new(value: Bit<Value>, size: SudokuSize, index: usize) -> Res<Self> {
        if value != Bit::zero() {
            let mut s = Square {
                value: Some(value),
                fixed: true,
                flags: Possibilities::empty(),
                index,
            };
            s.set_original(BitSet::full(size.house_size()), size);
            Ok(s)
        } else {
            Ok(Square {
                value: None,
                fixed: false,
                flags: Possibilities::full(size)?,
                index,
            })
        }
    }

    #[inline(always)]
    pub(crate) fn set_original(&mut self, remove: BitSet<Value>, size: SudokuSize) {
        self.flags.set_original(size, remove);
    }

    #[inline(always)]
    /// Returns true if the value was set as a initial given value
    pub(crate) fn is_fixed(&self) -> bool {
        self.fixed
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        self.index
    }

    #[inline(always)]
    pub(crate) fn inc(&mut self) -> bool {
        if self.fixed {
            false
        } else {
            match self.flags.next() {
                Some(v) => {
                    self.value = Some(v);
                    true
                }
                None => false,
            }
        }
    }

    #[inline]
    pub(crate) fn poss_contains(&self, flag: Bit<Value>) -> bool {
        self.flags.poss_contains(flag)
    }

    #[inline]
    pub(crate) fn remove_set(&mut self, values: BitSet<Value>) {
        self.flags.remove_multi(values)
    }

    #[inline]
    pub(crate) fn remove_pot(&mut self, val: Bit<Value>) {
        self.flags.remove(val);
    }

    #[inline]
    pub(crate) fn insert_set(&mut self, set: BitSet<Value>) {
        self.flags.insert_multi(set);
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn insert_pot(&mut self, val: Bit<Value>) {
        self.flags.insert(val);
    }
    #[inline]
    pub(crate) fn set_value(&mut self, value: Option<Bit<Value>>) {
        // if value == Bit::zero() {
        //     self.value = None;
        // } else {
        //     self.value = Some(value);
        // }
        self.value = value;
        self.flags = Possibilities::clear(&self.flags);
    }

    #[inline]
    pub(crate) fn reset(&mut self) {
        if !self.fixed {
            self.value = None;
            self.flags.reset();
        }
    }

    #[inline]
    pub(crate) fn poss(&self) -> &BitSet<Value> {
        self.flags.get()
    }

    #[inline]

    pub(crate) fn val(&self) -> Option<Bit<Value>> {
        self.value
    }

    // #[inline]
    // /// Returns the value in the square. A value of zero means that no value has been set.
    // pub fn num(&self) -> Value {
    //     if let Some(v) = self.value {
    //         v
    //     } else {
    //         Value::zero()
    //     }
    // }

    #[inline]
    pub(crate) fn count(&self) -> u32 {
        self.flags.count()
    }
}

impl<V1: BitSetInt> PartialEq<Bit<V1>> for Square<V1> {
    fn eq(&self, other: &Bit<V1>) -> bool {
        self.val().unwrap_or_else(|| Bit::zero()) == *other
    }
}

impl<V: BitSetInt> PartialEq<Square<V>> for Square<V> {
    fn eq(&self, other: &Square<V>) -> bool {
        self.value == other.value
    }
}

impl<Value: BitSetInt> Debug for Square<Value> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self.value {
            Some(v) => v,
            None => Bit::zero(),
        };
        let poss = self.flags.as_vec();
        write!(f, "Value: {:?}, Poss: {:?}", value, poss)
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;

    use std::error::Error;

    const SIZE: SudokuSize = SudokuSize::Three;

    type TSquare = Square<u16>;
    type Res = Result<(), Box<dyn Error>>;

    fn empty_square() -> TSquare {
        let mut sq = TSquare::new(0.into(), SIZE, 0).unwrap();
        sq.set_original(BitSet::empty(), SIZE);
        sq
    }

    fn value_square() -> TSquare {
        TSquare::new(3.into(), SIZE, 0).unwrap()
    }

    fn test_set(vals: &[u16]) -> BitSet<u16> {
        let mut set = BitSet::new();
        for val in vals {
            set = BitSet::insert(set, *val);
        }
        set
    }

    fn test_square(vals: &[u16]) -> TSquare {
        let mut sq = empty_square();
        let set = test_set(vals);
        sq.set_original(set, SIZE);
        sq
    }
    #[test]
    fn new_empty_test() -> Res {
        let sq = TSquare::new(0.into(), SIZE, 0)?;
        assert_eq!(sq.value, None);
        assert!(!sq.fixed);
        assert_eq!(sq.count(), 9u32);
        assert_eq!(sq.flags.get(), &BitSet::full(SIZE.house_size()));
        assert_eq!(sq.flags.get().count_u32(), SIZE.house_size() as u32);
        Ok(())
    }

    #[test]
    fn new_value_test() -> Res {
        let sq = TSquare::new(3.into(), SIZE, 0)?;
        assert_eq!(sq.value, Some(3.into()));
        assert!(sq.fixed);
        assert_eq!(sq.flags.get(), &BitSet::new());
        Ok(())
    }

    #[test]
    fn fixed() -> Res {
        let sq = empty_square();
        assert!(!sq.is_fixed());
        let sq = value_square();
        assert!(sq.is_fixed());
        Ok(())
    }

    #[test]
    fn should_not_inc() -> Res {
        let mut sq = value_square();
        let res = sq.inc();
        assert!(!res);
        assert_eq!(sq.val(), Some(3.into()));
        Ok(())
    }

    #[test]
    fn should_inc() -> Res {
        let mut sq = empty_square();
        let res = sq.inc();
        assert!(res);
        assert_eq!(sq.val(), Some(1.into()));
        Ok(())
    }

    #[test]
    fn inc_thru_range() -> Res {
        let mut sq = empty_square();
        let max = SIZE.house_size();
        for i in 1..=max {
            let res = sq.inc();
            assert!(res);
            assert_eq!(sq.val(), Some((i as u16).into()))
        }

        let res = sq.inc();
        assert!(!res);
        assert_eq!(sq.val(), Some((max as u16).into()));
        Ok(())
    }

    #[test]
    fn set_value() -> Res {
        let mut sq = empty_square();
        sq.set_value(Some(4.into()));
        assert_eq!(sq.val(), Some(4.into()));
        assert_eq!(sq.count(), 0);
        Ok(())
    }

    #[test]
    fn set_orig() -> Res {
        let sq = test_square(&[3, 5]);
        assert_eq!(sq.count(), 7);
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(sq.poss_contains(4.into()));
        Ok(())
    }

    #[test]
    fn insert_should_not_change_pot() -> Res {
        let mut sq = test_square(&[3, 5]);
        // Should not change value
        sq.insert_pot(4.into());
        assert_eq!(sq.count(), 7);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));

        Ok(())
    }

    #[test]
    fn insert_should_change_pot() -> Res {
        let mut sq = test_square(&[3, 5]);
        // Should change
        sq.insert_pot(3.into());
        assert_eq!(sq.count(), 8);
        assert!(sq.poss_contains(4.into()));
        assert!(sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));

        Ok(())
    }

    #[test]
    fn insert_set() -> Res {
        let mut sq = test_square(&[1, 2, 3, 5, 6]);
        let to_insert = test_set(&[1, 3]);
        sq.insert_set(to_insert);
        assert_eq!(sq.count(), 6);
        assert!(sq.poss_contains(4.into()));
        assert!(sq.poss_contains(3.into()));
        assert!(sq.poss_contains(1.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(!sq.poss_contains(2.into()));

        Ok(())
    }

    #[test]
    fn insert_set_no_change() -> Res {
        let mut sq = test_square(&[1, 2, 3, 5, 6]);
        let to_insert = test_set(&[4, 7]);
        sq.insert_set(to_insert);
        assert_eq!(sq.count(), 4, "{:?}", sq.flags);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(1.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(!sq.poss_contains(2.into()));

        Ok(())
    }

    #[test]
    fn insert_set_overlap() -> Res {
        let mut sq = test_square(&[1, 2, 3, 5, 6]);
        let to_insert = test_set(&[2, 7]);
        sq.insert_set(to_insert);
        assert_eq!(sq.count(), 5);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(1.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(sq.poss_contains(2.into()));

        Ok(())
    }

    #[test]
    fn remove_should_change_pot() -> Res {
        let mut sq = test_square(&[3, 5]);
        // Should change
        sq.remove_pot(2.into());
        assert_eq!(sq.count(), 6);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(!sq.poss_contains(2.into()));

        Ok(())
    }

    #[test]
    fn remove_not_should_change_pot() -> Res {
        let mut sq = test_square(&[3, 5]);
        // Should not change
        sq.remove_pot(3.into());
        assert_eq!(sq.count(), 7);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));

        Ok(())
    }

    #[test]
    fn remove_set1() -> Res {
        let mut sq = test_square(&[3, 5]);
        let to_remove = test_set(&[2, 7]);
        sq.remove_set(to_remove);
        assert_eq!(sq.count(), 5);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));
        assert!(!sq.poss_contains(2.into()));

        Ok(())
    }

    #[test]
    fn remove_set2_overlap() -> Res {
        let mut sq = test_square(&[3, 5]);
        let to_remove = test_set(&[3, 7]);
        sq.remove_set(to_remove);
        assert_eq!(sq.count(), 6);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));

        Ok(())
    }

    #[test]
    fn remove_set3_nochange() -> Res {
        let mut sq = test_square(&[3, 5]);
        let to_remove = test_set(&[3, 5]);
        sq.remove_set(to_remove);
        assert_eq!(sq.count(), 7);
        assert!(sq.poss_contains(4.into()));
        assert!(!sq.poss_contains(3.into()));
        assert!(!sq.poss_contains(5.into()));

        Ok(())
    }
}
