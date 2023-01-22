use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::custom::Sudoku;
use utils::*;

use rustoku::human_calcs::technique::*;

hinthardest!(dhard, "Doubles - Hardest", Technique::HiddenDouble);
hintgroup!(doubleh, "DoubleHidden", Technique::HiddenDouble);

hinthardest!(thard, "Triples - Hardest", Technique::HiddenTriple);
hintgroup!(tripleh, "TripleHidden", Technique::HiddenTriple);

hinthardest!(qhard, "Quad - Hardest", Technique::HiddenQuad);
hintgroup!(quadh, "QuadHidden", Technique::HiddenQuad);

criterion_group!(benches, dhard, doubleh, thard, tripleh, qhard, quadh);
criterion_main!(benches);
