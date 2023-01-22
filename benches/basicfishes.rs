use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::custom::Sudoku;
use rustoku::human_calcs::technique::*;
use utils::*;

hinthardest!(dhard, "Xwing - Hardest", Technique::XWing);
hintgroup!(doubleh, "Xwing", Technique::XWing);

hinthardest!(thard, "Swordfish - Hardest", Technique::Swordfish);
hintgroup!(tripleh, "Swordfish", Technique::Swordfish);

hinthardest!(qhard, "Jelly - Hardest", Technique::Jellyfish);
hintgroup!(quadh, "Jellyfish", Technique::Jellyfish);

criterion_group!(benches, dhard, doubleh, thard, tripleh, qhard, quadh);

criterion_main!(benches);
