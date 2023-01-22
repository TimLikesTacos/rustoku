use std::convert::TryInto;

use crate::bitset::Bit;
use std::fmt::{Debug, Display};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not,
    Rem, Shl, Shr, Sub, SubAssign,
};

pub trait BitSetInt:
    Debug
    + Display
    + Copy
    + Clone
    + BitXor<Output = Self>
    + BitAnd<Output = Self>
    + BitXorAssign
    + BitAndAssign
    + Sized
    + BitOr<Output = Self>
    + BitOrAssign
    + Shr<Output = Self>
    + Shl<Output = Self>
    + Not<Output = Self>
    + SubAssign
    + AddAssign
    + PartialEq
    + PartialOrd
    + Ord
    + Eq
    + Ord
    + Rem<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + TryInto<usize>
    + Send
    + Sync
    + From<Bit<Self>>
    + Into<Bit<Self>>
    + 'static
{
    fn max_single_bit() -> Self;
    fn is_zero(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn inc(&self) -> Option<Self>;
    fn dec(&self) -> Option<Self>;
    fn count_u32(&self) -> u32;
    fn count(&self) -> Self;
    fn all_ones() -> Self;
    fn from_usize(value: usize) -> Self;
}

macro_rules! implBitSetInt {
    ($t: ty) => {
        impl BitSetInt for $t {
            #[inline(always)]
            fn max_single_bit() -> Self {
                <$t>::MAX - <$t>::MAX / 2
            }

            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }

            #[inline(always)]
            fn is_zero(self) -> bool {
                self == 0
            }

            #[inline]
            fn count_u32(&self) -> u32 {
                self.count_ones()
            }

            #[inline]
            fn count(&self) -> Self {
                self.count_ones() as Self
            }

            #[inline]
            fn all_ones() -> Self {
                <$t>::MAX
            }

            fn inc(&self) -> Option<Self> {
                if self == &0 {
                    return Some(1);
                }
                let new = self << 1;
                if new > *self {
                    Some(new)
                } else {
                    None
                }
            }
            fn dec(&self) -> Option<Self> {
                let new = self >> 1;
                if new == *self {
                    None
                } else {
                    Some(new)
                }
            }

            fn from_usize(value: usize) -> Self {
                value as $t
            }
        }
    };
}

implBitSetInt!(u8);
implBitSetInt!(u16);
implBitSetInt!(u32);
implBitSetInt!(u64);
implBitSetInt!(u128);
implBitSetInt!(usize);

#[cfg(test)]
mod trait_tests {
    use super::*;

    #[test]
    fn zero() {
        assert_eq!(0u8, 0);
        assert!(0u8.is_zero());
        assert_eq!(0u16, 0);
        assert!(0u16.is_zero());
        assert_eq!(0u32, 0);
        assert!(0u32.is_zero());
        assert_eq!(0u64, 0);
        assert!(0u64.is_zero());
        assert_eq!(0u128, 0);
        assert!(0u128.is_zero());
    }

    #[test]
    fn max_single_bit_bit() {
        assert_eq!(<u8>::max_single_bit(), 0b1000_0000);
        assert_eq!(<u16>::max_single_bit(), 0b1000_0000_0000_0000);
        assert_eq!(
            <u32>::max_single_bit(),
            0b1000_0000_0000_0000_0000_0000_0000_0000
        );
        assert_eq!(
            <u64>::max_single_bit(),
            0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
        );
        assert_eq!(<u128>::max_single_bit(), 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000);
    }

    #[test]
    fn should_inc_until_max() {
        let zero = <u8>::zero();
        assert_eq!(zero.inc(), Some(<u8>::one()));
        assert_eq!(zero.inc().unwrap().inc(), Some(0b10));
        assert_eq!(<u8>::max_single_bit().inc(), None);
    }

    #[test]
    fn should_dec_until_zero() {
        assert_eq!(<u8>::one().dec(), Some(<u8>::zero()));
        assert_eq!(<u8>::zero().dec(), None);
        assert_eq!(<u8>::max_single_bit().dec(), Some(0b100_0000));
    }
}
