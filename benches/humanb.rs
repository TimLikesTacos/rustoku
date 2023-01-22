use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
extern crate rustoku;
pub mod utils;
use rustoku::custom::Sudoku;
use utils::*;

fn benchmark(c: &mut Criterion) {
    let input = ".4.69.5..3.....2..61...48.7....38.2.....5.76.12497.3...9856..7....2..6...3...1..4";

    c.bench_with_input(BenchmarkId::new("Human1", "puzzle1"), &input, |b, &i| {
        b.iter(|| black_box(Sudoku::<u64>::new(i).unwrap().human_solve()))
    });

    let puzzles = get_puzzles(10);
    let mut group = c.benchmark_group("Human Solve");
    for puzzle in puzzles {
        group.bench_with_input(
            BenchmarkId::from_parameter(&puzzle[0..10]),
            &puzzle,
            |b, s| {
                b.iter(|| black_box(Sudoku::<u64>::new(s.as_str()).unwrap().human_solve()));
            },
        );
    }
    group.finish();

    let puzzles = get_puzzles(usize::MAX);
    let c = Criterion::default();
    let mut c = c.sample_size(10);
    c.bench_with_input(BenchmarkId::new("Bunch o puzzles", ""), &puzzles, |b, s| {
        b.iter(|| {
            {
                let s = s.clone();
                for str in s.iter() {
                    Sudoku::<u64>::new(str.as_str())
                        .unwrap()
                        .human_solve()
                        .unwrap();
                }
            };
            black_box(())
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
