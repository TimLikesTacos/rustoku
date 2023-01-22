use crate::bitset::{BitSet, BitSetInt};
use crate::grid::Index;
use crate::human_calcs::technique::Technique;
use crate::sudoku::sudoku::Sudoku;

use std::fmt::{Debug, Formatter};
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq)]
/// Convenient grouping of an index and values, typically associated with potential values of a certain square
pub struct IndexValuePair<S> {
    index: Index,
    value: S,
}

impl<S> IndexValuePair<S> {
    #[inline]
    pub fn index(&self) -> Index {
        self.index
    }

    #[inline]
    pub fn value(&self) -> &S {
        &self.value
    }
}

impl<D: std::fmt::Display> std::fmt::Display for IndexValuePair<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coord: (usize, usize) = self.index.into();
        write!(
            f,
            "Index: {}, Coordinate: r{}, c{}",
            &self.index.inner_index, coord.0, coord.1
        )?;
        write!(f, "Value: {}", self.value)
    }
}
/// Struct to document change in the puzzle
#[derive(Clone, Debug, PartialEq)]
pub struct Move<Value: BitSetInt> {
    pub method: Option<Technique>,
    pub manual: bool,
    pub used_to_solve: Vec<IndexValuePair<BitSet<Value>>>,
    pub removed_potentials: Vec<IndexValuePair<BitSet<Value>>>,
    pub value_set: Option<IndexValuePair<Value>>,
}

impl<D: std::fmt::Display + BitSetInt> std::fmt::Display for Move<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(set_pair) = &self.value_set {
            let coord: (usize, usize) = set_pair.index.into();
            write!(
                f,
                "Square {}, coordinate {}, {} was set to {}",
                set_pair.index.inner_index,
                coord.0,
                coord.1,
                set_pair.value()
            )?;
        }
        if !self.used_to_solve.is_empty() {
            let mut str = self.used_to_solve.iter().fold("".to_owned(), |acc, x| {
                format!("{}{},", acc, x.index.inner_index)
            });
            str.pop();
            write!(f, "{} indexes were used to solve", str)?;
        }
        Ok(())
    }
}
impl<S> IndexValuePair<S> {
    pub(crate) fn new(index: Index, value: S) -> IndexValuePair<S> {
        IndexValuePair { index, value }
    }
}

impl<Value: BitSetInt> Move<Value> {
    #[inline]
    pub(crate) fn new() -> Move<Value> {
        Move {
            method: None,
            manual: false,
            used_to_solve: vec![],
            removed_potentials: vec![],
            value_set: None,
        }
    }

    pub(crate) fn set_value(&mut self, ind: Index, value: Value, manual: bool) {
        self.manual = manual;
        if manual {
            self.method = None;
        }
        self.value_set = Some(IndexValuePair::new(ind, value));
    }

    #[inline]
    pub(crate) fn set_method(&mut self, tech: Option<Technique>) {
        if let Some(t) = tech {
            self.manual = false;
            self.method = Some(t);
        } else {
            self.manual = true;
            self.method = None;
        }
    }

    #[inline]
    pub fn has_set_value(&self) -> bool {
        self.value_set.is_some()
    }

    #[inline]
    pub(crate) fn add_removed_potential(&mut self, ind: Index, value: BitSet<Value>) {
        self.removed_potentials
            .push(IndexValuePair::new(ind, value));
    }

    #[inline]
    /// which squares and it's potential values used to determine a move
    pub(crate) fn add_used_to_solve(&mut self, index: Index, value: BitSet<Value>) {
        self.used_to_solve.push(IndexValuePair::new(index, value));
    }

    #[inline]
    /// Obtain the technique used for the move
    pub fn technique(&self) -> &Option<Technique> {
        &self.method
    }

    #[inline]
    /// What value was set, if any.
    pub fn change(&self) -> &Option<IndexValuePair<Value>> {
        &self.value_set
    }

    #[inline]
    #[allow(dead_code)]
    /// Returns a vector of `IndexValuePair` containing the index and the affected values
    pub(crate) fn involved_vec(&self) -> &Vec<IndexValuePair<BitSet<Value>>> {
        &self.used_to_solve
    }

    /// Returns a vector of IndexValuePairs for values used in determining the move.
    pub fn involved_values(&self) -> Vec<IndexValuePair<Vec<Value>>> {
        self.used_to_solve
            .iter()
            .map(|indpair| {
                let vec: Vec<Value> = indpair.value.iter().map(|bit| Value::from(bit)).collect();
                IndexValuePair {
                    index: indpair.index,
                    value: vec,
                }
            })
            .collect()
    }

    #[inline]
    /// Returns a vector of `IndexValuePair` containing the index and affection potienial values
    pub(crate) fn removed_potentials_vec(&self) -> &Vec<IndexValuePair<BitSet<Value>>> {
        &self.removed_potentials
    }

    /// Returns vector of IndexValuePairs with values of removed potentials
    pub fn removed_potentials(&self) -> Vec<IndexValuePair<Vec<Value>>> {
        self.removed_potentials
            .iter()
            .map(|indpair| {
                let vec: Vec<Value> = indpair.value.iter().map(|bit| Value::from(bit)).collect();
                IndexValuePair {
                    index: indpair.index,
                    value: vec,
                }
            })
            .collect()
    }

    #[allow(dead_code)]
    pub(crate) fn apply(self, puz: &mut Sudoku<Value>) -> Move<Value> {
        if let Some(value) = &self.value_set {
            return puz
                .set_with_technique(value.index, value.value.into(), *self.technique())
                .unwrap()
                .clone();
        }

        // todo: does this need to change to how Hint does it?
        for pot_pair in self.removed_potentials.iter() {
            for bit in pot_pair.value.iter() {
                puz.remove_potential(pot_pair.index, bit)
                    .expect("Index came internally: safe");
            }
        }
        self
    }
}

impl<'a, Value: BitSetInt + 'a> FromIterator<&'a Move<Value>> for Vec<Move<Value>> {
    fn from_iter<T: IntoIterator<Item = &'a Move<Value>>>(iter: T) -> Self {
        let mut ret: Vec<Move<Value>> = Vec::new();
        for v in iter {
            ret.push(v.clone())
        }
        ret
    }
}
