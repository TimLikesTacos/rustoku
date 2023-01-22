use crate::bitset::BitSetInt;
use crate::move_change::Move;
use crate::Sudoku;

pub mod basic;
pub mod fish;
pub mod guess;
pub mod humansolve;
pub mod technique;
pub(crate) mod tuple_ctr;
pub(crate) mod tuples;

pub trait TechStruct {
    type SquareValue;
    fn tech_hint(&self, puz: &Sudoku<Self::SquareValue>) -> Option<Move<Self::SquareValue>>
    where
        <Self as TechStruct>::SquareValue: BitSetInt;
}
