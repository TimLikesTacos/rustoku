use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
mod utils;
use rustoku::basic::Sudoku;
use utils::*;

fn benchmark(c: &mut Criterion) {
    let input = ".4.69.5..3.....2..61...48.7....38.2.....5.76.12497.3...9856..7....2..6...3...1..4";
    c.bench_with_input(
        BenchmarkId::new("Brute_Single", "puzzle1"),
        &input,
        |b, &i| b.iter(|| black_box(Sudoku::new(i))),
    );

    let puzzles = get_puzzles(10);
    let mut group = c.benchmark_group("Brute Solve");
    for puzzle in puzzles {
        group.bench_with_input(
            BenchmarkId::from_parameter(&puzzle[0..10]),
            &puzzle,
            |b, s| {
                b.iter(|| black_box(Sudoku::new(s.as_str())));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
