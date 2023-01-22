use std::marker::PhantomData;

mod candidates;
mod claiming;
mod pointing;
mod single_cand;
mod single_poss;

pub struct ClaimingCandidate<V>(PhantomData<V>);
pub struct PointingCandidate<V>(PhantomData<V>);
pub struct SingleCandidate<V>(pub PhantomData<V>);
/// Fills in single possibilities, returns vector of what positions were filled.
/// A single possibility is where only one cell in a row/column/box has the possibility of a value.
pub struct SinglePossibility<V>(PhantomData<V>);
