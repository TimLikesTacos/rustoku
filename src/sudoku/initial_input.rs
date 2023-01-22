use crate::errors::SudError;

use crate::bitset::BitSetInt;
use std::convert::TryFrom;

/// Trait to convert the initial input values into the puzzle.  This can be used to configure strings,
/// vectors, and other sources to be the input to the puzzle.
pub trait InitialInput<V: BitSetInt> {
    fn initial(self) -> Result<Vec<V>, SudError>;
}

impl<T: BitSetInt> InitialInput<T> for Vec<T> {
    fn initial(self) -> Result<Vec<T>, SudError> {
        Ok(self)
    }
}

impl<T: BitSetInt> InitialInput<T> for &Vec<T> {
    fn initial(self) -> Result<Vec<T>, SudError> {
        Ok(self.clone())
    }
}

impl<T: BitSetInt> InitialInput<T> for &[T] {
    fn initial(self) -> Result<Vec<T>, SudError> {
        Ok(self.to_vec())
    }
}
/// This only works when the puzzle size is the normal size of 9x9 or less, as each digit is parsed.
/// and assumes a base 10 number.  If using a larger puzzle, use other methods to develop the input.
impl<'a, T: BitSetInt + TryFrom<u32>> InitialInput<T> for &'a str {
    fn initial(self) -> Result<Vec<T>, SudError> {
        let radix = 10;
        let v = self
            .chars()
            .into_iter()
            .map(|n| T::try_from(n.to_digit(radix).unwrap_or(0)).map_err(|_| SudError::InputParse))
            .collect::<Result<Vec<_>, SudError>>()?;

        Ok(v)
    }
}
