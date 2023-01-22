use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::custom::Sudoku;
use rustoku::human_calcs::technique::*;
use utils::*;

hinthardest!(dhard, "DoublesN - Hardest", Technique::NakedDouble);
hintgroup!(doubleh, "DoubleNaked", Technique::NakedDouble);

hinthardest!(thard, "TriplesN - Hardest", Technique::NakedTriple);
hintgroup!(tripleh, "TripleNaked", Technique::NakedTriple);

hinthardest!(qhard, "QuadN - Hardest", Technique::NakedQuad);
hintgroup!(quadh, "QuadNaked", Technique::NakedQuad);

criterion_group!(benches, dhard, doubleh, thard, tripleh, qhard, quadh);
criterion_main!(benches);
