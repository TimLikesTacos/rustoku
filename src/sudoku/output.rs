use crate::bitset::BitSetInt;
use crate::{Grid, Sudoku};

/// A trait that produces a string representation of the sudoku puzzle.
pub trait OutputString {
    /// Outputs the puzzle as string. `empty_char` is what char should be used to represent an empty cell,
    /// `delimiter` is what char is used to separate cells, if any
    fn output_string(&self, empty_char: char, delimiter: Option<char>) -> String;
}

impl<T: BitSetInt> OutputString for Grid<T> {
    fn output_string(&self, empty_char: char, delimiter: Option<char>) -> String {
        let map_fun = |v: &T| {
            let value = if *v == T::zero() {
                empty_char.to_string()
            } else {
                v.to_string()
            };
            if let Some(del_char) = &delimiter {
                format!("{}{}", value, del_char)
            } else {
                value
            }
        };

        let mut str = self.iter().map(map_fun).collect::<String>();
        if delimiter.is_some() {
            str.pop();
        }
        str
    }
}

impl<T: BitSetInt> OutputString for Sudoku<T> {
    ///
    /// ```
    /// use std::error::Error;
    /// use rustoku::basic::*;
    ///
    /// fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///   use rustoku::basic::*;
    ///   use rustoku::OutputString;
    ///
    ///   let puz = "...15..3.9..4....7.58.9....31....72.4.......8.......5....24...55.......6.71..9...";
    ///   let puzzle = Sudoku::new(puz)?;
    ///   println!("--Original--\n{}", puzzle.output_string('.', None));
    ///   if let Solution::One(grid) = puzzle.solution() {
    ///      println!("--Brute--\n{}", grid.output_string('.', None));      
    ///   }
    ///  Ok(())
    /// }
    /// ```
    fn output_string(&self, empty_char: char, delimiter: Option<char>) -> String {
        let map_fun = |v: T| {
            let value = if v == T::zero() {
                empty_char.to_string()
            } else {
                v.to_string()
            };
            if let Some(del_char) = &delimiter {
                format!("{}{}", value, del_char)
            } else {
                value
            }
        };
        let mut str: String = self.value_iter().map(map_fun).collect();
        if delimiter.is_some() {
            str.pop();
        }
        str
    }
}

#[cfg(test)]
mod output_string_tests {
    use super::*;
    use crate::SudError;

    #[test]
    fn ninebynine_no_delimit() -> Result<(), SudError> {
        let sudoku = <Sudoku<u16>>::new(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )?;

        assert_eq!(
            sudoku.output_string('.', None),
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4"
                .to_owned()
        );

        let sol = sudoku.unique_solution()?.output_string('.', None);
        assert_eq!(
            sol,
            "652483917978162435314975628825736149791824563436519872269348751547291386183657294"
                .to_string()
        );

        Ok(())
    }

    #[test]
    fn ninebynine_with_delmit() -> Result<(), SudError> {
        let sudoku = <Sudoku<u16>>::new(
            ".5..83.17...1..4..3.4..56.8....3...9.9.8245....6....7...9....5...729..861.36.72.4",
        )?;

        assert_eq!(
            sudoku.output_string('0', Some(',')),
            "0,5,0,0,8,3,0,1,7,0,0,0,1,0,0,4,0,0,3,0,4,0,0,5,6,0,8,0,0,0,0,3,0,0,0,9,0,9,0,8,2,4,5,0,0,0,0,6,0,0,0,0,7,0,0,0,9,0,0,0,0,5,0,0,0,7,2,9,0,0,8,6,1,0,3,6,0,7,2,0,4"
                .to_owned()
        );

        let sol = sudoku.unique_solution()?.output_string('.', Some(','));
        assert_eq!(
            sol,
            "6,5,2,4,8,3,9,1,7,9,7,8,1,6,2,4,3,5,3,1,4,9,7,5,6,2,8,8,2,5,7,3,6,1,4,9,7,9,1,8,2,4,5,6,3,4,3,6,5,1,9,8,7,2,2,6,9,3,4,8,7,5,1,5,4,7,2,9,1,3,8,6,1,8,3,6,5,7,2,9,4"
                .to_string()
        );

        Ok(())
    }
}
