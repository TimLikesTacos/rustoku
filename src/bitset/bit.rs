use crate::bitset::BitSetInt;
use std::fmt::Debug;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub struct Bit<T: BitSetInt>(pub(crate) T);

impl<T: BitSetInt> Bit<T> {
    #[inline(always)]
    fn change(self, operator: fn(T, T) -> T) -> Bit<T> {
        Bit(operator(self.0, T::one()))
    }

    #[inline(always)]
    pub fn inc(self) -> Option<Bit<T>> {
        if self.0 == T::max_single_bit() {
            None
        } else if self.0 == T::zero() {
            Some(Bit(T::one()))
        } else {
            Some(self.change(T::shl))
        }
    }

    #[inline(always)]
    pub fn dec(self) -> Option<Bit<T>> {
        if self.0 == T::zero() {
            None
        } else {
            Some(self.change(T::shr))
        }
    }

    #[inline(always)]
    pub fn zero() -> Bit<T> {
        Bit(T::zero())
    }

    #[inline(always)]
    /// Alias for `zero`
    pub fn empty() -> Bit<T> {
        Bit::zero()
    }

    #[inline(always)]
    pub fn one() -> Bit<T> {
        Bit(T::one())
    }

    #[inline(always)]
    pub fn max() -> Bit<T> {
        Bit(T::max_single_bit())
    }
}

macro_rules! implFrom {
    ($t: ty) => {
        impl From<$t> for Bit<$t>
        where
            $t: BitSetInt,
        {
            #[inline(always)]
            fn from(value: $t) -> Self {
                if value == <$t>::zero() {
                    Self::zero()
                } else {
                    Bit(<$t>::one() << (value - 1))
                }
            }
        }

        impl From<&$t> for Bit<$t>
        where
            $t: BitSetInt,
        {
            #[inline(always)]
            fn from(value: &$t) -> Self {
                if *value == <$t>::zero() {
                    Self::zero()
                } else {
                    Bit(<$t>::one() << (*value - 1))
                }
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

macro_rules! implTo {
    ($t: ty) => {
        impl From<Bit<$t>> for $t {
            fn from(value: Bit<$t>) -> Self {
                if value.0 == 0 {
                    0
                } else {
                    let mut val = 0;
                    let mut copy = value.0;
                    while copy > 0 {
                        val += 1;
                        copy >>= 1;
                    }
                    val
                }
            }
        }
    };
}

implTo!(u8);
implTo!(u16);
implTo!(u32);
implTo!(u64);
implTo!(u128);
implTo!(usize);

#[cfg(test)]
mod bit_tests {
    use super::*;

    const ITEM1: Bit<u16> = Bit(0b1000);

    #[test]
    fn inc() {
        let max: Bit<u16> = Bit::max();
        let zero: Bit<u16> = Bit::zero();

        assert_eq!(ITEM1.inc(), Some(Bit(0b1_0000)));
        let item2 = ITEM1.inc().unwrap();
        assert_eq!(item2.inc(), Some(Bit(0b10_0000)));
        assert_eq!(max.inc(), None);
        assert_eq!(zero.inc(), Some(Bit(0b1)));
    }

    #[test]
    fn dec() {
        let max: Bit<u16> = Bit::max();
        let zero: Bit<u16> = Bit::zero();

        assert_eq!(ITEM1.dec(), Some(Bit(0b100)));
        assert_eq!(max.dec(), Some(Bit(0b0100_0000_0000_0000)));
        assert_eq!(zero.dec(), None);
    }

    #[test]
    fn try_from() {
        assert_eq!(<Bit<u16>>::try_from(4).unwrap(), Bit(0b1000));
    }

    #[test]
    fn try_to() {
        assert_eq!(<u32>::from(Bit(0b1_0000)), 5u32);
    }
}
