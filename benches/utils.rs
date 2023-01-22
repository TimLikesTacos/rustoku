pub fn get_puzzles(max: usize) -> Vec<String> {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;
    let f = File::open("tests/data/unsolved/kaggle_t0.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut strings: Vec<String> = vec![];
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.chars().next().unwrap() != '#' {
            strings.push(line);
        }
        if strings.len() >= max {
            break;
        }
    }
    strings
}

#[macro_export]
macro_rules! hardest {
    () => {
        Sudoku::new(
            "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..",
        )
        .unwrap()
    };
}

#[macro_export]
macro_rules! bench1 {
    ($name : ident, $other: expr, $target : path) => {
        fn $name(c: &mut Criterion) {
            c.bench_function($other, |b| b.iter(|| black_box($target())));
        }
    };
    ($name : ident, $other: expr, $target : expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function($other, |b| b.iter(|| black_box($target)));
        }
    };
}

#[macro_export]
macro_rules! bench1param {
    ($name : ident, $other: expr, $target : path, $param: expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_with_input(BenchmarkId::new($other, "macro"), &$param, |b, &i| {
                b.iter(|| black_box($target(i)))
            });
        }
    };
}

#[macro_export]
macro_rules! benchself {
    // ($name : ident, $other: expr, $target : path, $param: expr) => {
    //         fn $name (c: &mut Criterion) {
    //              c.bench_with_input(BenchmarkId::new($other, "macro"), & $param, |b, &i| {
    //                  b.iter(||black_box($target(i)))
    //              });
    //
    //         }
    // }
    ($name : ident, $other: expr, $target : expr, $param: expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_with_input(BenchmarkId::new($other, "macro"), &$param, |b, &i| {
                b.iter(|| black_box($target(i)))
            });
        }
    };

    ($name : ident, $other: expr, $target : expr) => {
        fn $name(c: &mut Criterion) {
            c.bench_function($other, |b| b.iter(|| black_box($target())));
        }
    };
}

#[macro_export]
macro_rules! benchselffn {
    ($name : ident, $other: expr, $target : expr, $func: ident) => {
        fn $name(c: &mut Criterion) {
            let thself = $target;
            c.bench_with_input(BenchmarkId::new($other, "macro"), &thself, |b, &i| {
                b.iter(|| black_box(i.$func()))
            });
        }
    };

    ($name : ident, $other: expr, $target : expr, $func: ident, $rhs: expr) => {
        fn $name(c: &mut Criterion) {
            let thself = $target;
            c.bench_with_input(BenchmarkId::new($other, "macro"), &thself, |b, &i| {
                b.iter(|| black_box(i.$func($rhs)))
            });
        }
    };
}

#[macro_export]
macro_rules! hintgroup {
    ($name : ident, $other: expr, $func: path) => {
        fn $name(c: &mut Criterion) {
            let puzzles = get_puzzles(10);
            let mut group = c.benchmark_group($other);
            for puzzle in puzzles {
                group.bench_with_input(
                    BenchmarkId::from_parameter(&puzzle[0..10]),
                    &puzzle,
                    |b, s| {
                        b.iter(|| {
                            black_box({
                                let p = Sudoku::<u64>::new(s.as_str()).unwrap();
                                let _ = $func.solver().tech_hint(&p);
                            })
                        });
                    },
                );
            }
            group.finish();
        }
    };
}

#[macro_export]
macro_rules! hinthardest {
    ($name : ident, $other: expr, $func : path) => {
        fn $name(c: &mut Criterion) {
            c.bench_function($other, |b| {
                b.iter(|| black_box($func.solver::<u32>().tech_hint(&hardest!())))
            });
        }
    };
}
