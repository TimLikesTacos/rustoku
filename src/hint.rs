use crate::bitset::BitSetInt;
use crate::{Move, Sudoku};
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Hint<V: BitSetInt>(pub(crate) Move<V>);

// A wrapper around Move. Makes clear that it has not occured yet.  Hint turns into a Move when `apply`ed.
impl<V: BitSetInt> Hint<V> {
    /// Consumes the hint, applies the hint to the sudoku and returns the move
    pub fn apply(self, sudoku: &mut Sudoku<V>) -> Move<V> {
        if let Some(value) = &self.0.value_set {
            return sudoku
                .set_with_technique(value.index(), *value.value(), *self.0.technique())
                .unwrap()
                .clone();
        }

        sudoku
            .remove_potentials_from_hint(self)
            .expect("Unanticipated conflict in hint and puzzle")
    }
}

// todo: impl its own methods instead of impl deref
impl<V: BitSetInt> Deref for Hint<V> {
    type Target = Move<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
