use crate::bitset::{BitSet, BitSetInt};

#[derive(Debug)]
pub(super) struct Candidates<V: BitSetInt, Extra> {
    pub(super) box_both: BitSet<V>,
    pub(super) box_outside: BitSet<V>,
    pub(super) outside: BitSet<V>,
    pub(super) extra: Extra,
}
