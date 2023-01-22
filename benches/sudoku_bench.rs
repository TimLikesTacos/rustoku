use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;

use rustoku::basic::Sudoku;
mod utils;
use utils::*;

fn criterion_benchmark(c: &mut Criterion) {
    let input = ".4.69.5..3.....2..61...48.7....38.2.....5.76.12497.3...9856..7....2..6...3...1..4";

    c.bench_with_input(BenchmarkId::new("Human1", "puzzle1"), &input, |b, &i| {
        b.iter(|| black_box(Sudoku::new(i).unwrap().human_solve()))
    });

    c.bench_with_input(BenchmarkId::new("New1", "puzzle1"), &input, |b, &i| {
        b.iter(|| black_box(Sudoku::new(i)))
    });

    let puzzles = get_puzzles(10);
    let mut group = c.benchmark_group("Different puzzles");
    for puzzle in puzzles {
        group.bench_with_input(
            BenchmarkId::from_parameter(&puzzle[0..10]),
            &puzzle,
            |b, s| {
                b.iter(|| black_box(Sudoku::new(s.as_str()).unwrap()));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
