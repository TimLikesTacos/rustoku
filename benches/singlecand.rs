use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use utils::*;

use rustoku::human_calcs::basic::SingleCandidate;

use rustoku::custom::Sudoku;
use rustoku::human_calcs::TechStruct;

fn benchmark(c: &mut Criterion) {
    let puzzles = get_puzzles(10);
    let mut group = c.benchmark_group("Single Candidates");
    for puzzle in puzzles {
        group.bench_with_input(
            BenchmarkId::from_parameter(&puzzle[0..10]),
            &puzzle,
            |b, s| {
                b.iter(|| {
                    {
                        let p = Sudoku::<u16>::new(s.as_str()).unwrap();
                        let sc = SingleCandidate::new();
                        let _hint = sc.tech_hint(&p);
                    };
                    black_box(())
                });
            },
        );
    }
    group.finish();
}

bench1!(
    hard,
    "SingleCand - Hardest",
    SingleCandidate::<u16>::new().tech_hint(&hardest!())
);

criterion_group!(benches, benchmark, hard);
criterion_main!(benches);
