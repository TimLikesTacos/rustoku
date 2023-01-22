use crate::bitset::{Bit, BitSet, BitSetInt};
use crate::errors::SudError;
use crate::grid::SudokuSize;

pub trait InputValue: BitSetInt {
    fn into_bit(self) -> Bit<Self> {
        self.into()
    }

    fn into_set(self) -> BitSet<Self> {
        self.into_bit().into()
    }

    fn from_bit(bit: Bit<Self>) -> Self {
        bit.into()
    }

    fn checked_into_bit(self, size: SudokuSize) -> Result<Bit<Self>, SudError> {
        let v = self.into_bit();
        if <BitSet<Self>>::from(v) < BitSet::full(size.house_size()) {
            Ok(v)
        } else {
            Err(SudError::ValueNotPossible(
                "Value exceeds maximum".to_owned(),
            ))
        }
    }
}

//todo macro and add all options

impl InputValue for u16 {}
impl InputValue for u32 {}
impl InputValue for u64 {}
impl InputValue for u128 {}
