use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::custom::Sudoku;
use utils::*;

use rustoku::human_calcs::technique::*;

hinthardest!(dhard, "FinnedXwing - Hardest", Technique::FinnedXWing);
hintgroup!(doubleh, "FinnedXwing", Technique::FinnedXWing);

hinthardest!(
    thard,
    "FinnedSwordfish - Hardest",
    Technique::FinnedSwordfish
);
hintgroup!(tripleh, "FinnedSwordfish", Technique::FinnedSwordfish);

hinthardest!(qhard, "FinnedJelly - Hardest", Technique::FinnedJellyfish);
hintgroup!(quadh, "FinnedJellyfish", Technique::FinnedJellyfish);

criterion_group!(benches, dhard, doubleh, thard, tripleh, qhard, quadh);
criterion_main!(benches);
