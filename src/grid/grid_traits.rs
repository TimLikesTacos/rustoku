//! Grid inputs are either usize, u8, or Bit.  This will allow any of these to be used to input into the grid
use crate::bitset::*;
use crate::errors::SudError;
use crate::grid::grid::Grid;
use crate::square::Square;
use crate::sudokusize::*;
use std::convert::TryInto;

pub trait GridTrait {}
impl<T> GridTrait for Grid<T> {}

pub trait Rotate {
    type PuzzleInp;
    fn cwrotate(self, quarter_rotations: usize) -> Result<Self::PuzzleInp, SudError>;
}
//todo: rename and move out of grid. This is not a grid trait.
pub(crate) trait GridInput
where
    Self: Sized,
{
    fn input(self, size: SudokuSize) -> Res<Self>;
}

impl<T: TryInto<usize> + BitSetInt> GridInput for T {
    fn input(self, size: SudokuSize) -> Res<Self> {
        if self.try_into().unwrap_or_else(|_| size.house_size()) > size.house_size() {
            return Err(SudError::ValueNotPossible(format!("{:?}", &self)));
        } else {
            Ok(self)
        }
    }
}

impl<V: BitSetInt> PartialEq<Grid<V>> for Grid<Square<V>> {
    fn eq(&self, other: &Grid<V>) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(l, r)| l.value == Some((*r).into()))
    }
}
