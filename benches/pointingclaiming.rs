use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::custom::Sudoku;
use rustoku::human_calcs::technique::*;
use utils::*;

hinthardest!(dhard, "Pointing - Hardest", Technique::Pointing);
hintgroup!(doubleh, "Pointing", Technique::Pointing);

hinthardest!(thard, "Claiming - Hardest", Technique::Claiming);
hintgroup!(tripleh, "Claiming", Technique::Claiming);

criterion_group!(benches, dhard, doubleh, thard, tripleh);
criterion_main!(benches);
