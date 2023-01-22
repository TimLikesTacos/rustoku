use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn one_known_file() -> Vec<String> {
    // puzzles0_kaggle has 100,000 sudokus
    let f = File::open("tests/data/unsolved/kaggle_t0.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut strings: Vec<String> = vec![];
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if !line.starts_with('#') {
            strings.push(line);
        }
    }
    strings
}
