use crate::bitset::{Bit, BitSetInt};
use crate::errors::SudError;
use crate::grid::grid::Grid;
use crate::solution::Solution;
use crate::square::Square;

static MAX_SOLUTIONS: usize = 5;

/// Brute force solving.  Returns multiple solutions if they exist
pub struct BruteForce {}

impl BruteForce {
    /// Moves to the next non-fixed square. Returns None if end of puzzle.
    #[inline(always)]
    fn move_right<V: BitSetInt>(p: &Grid<Square<V>>, c: usize) -> Option<usize> {
        let max = p.size.total();
        let mut cur = c;
        loop {
            if cur + 1 < max {
                cur += 1;
                if p.items.get(cur).unwrap().is_fixed() {
                    continue;
                } else {
                    return Some(cur);
                }
            } else {
                return None;
            }
        }
    }

    #[inline(always)]
    fn inc<V: BitSetInt>(puz: &mut Grid<Square<V>>, index: usize) -> bool {
        if let Some(sq) = puz.items.get_mut(index) {
            sq.inc()
        } else {
            false
        }
    }

    #[inline(always)]
    fn reset<V: BitSetInt>(puz: &mut Grid<Square<V>>, index: usize) {
        if let Some(sq) = puz.items.get_mut(index) {
            sq.reset()
        }
    }
    /// Solves the Sudoku puzzle.  Returns a SolutionReport reflecting the number of solutions.
    pub(crate) fn solve<V: BitSetInt>(grid: &Grid<Square<V>>) -> Result<Solution<V>, SudError> {
        // This vector will store the solutions
        let mut to_return: Vec<Grid<V>> = Vec::new();

        // Make a mutable clone of the puzzle to solve.
        let mut puz: Grid<Square<V>> = grid.clone();

        let mut position = 0;
        // Stores solutions

        // move position to non-fixed point
        if puz.items.get(position).unwrap().is_fixed() {
            position = match Self::move_right(&puz, position) {
                Some(v) => v,
                None => puz.num_squares() - 1,
            };
        }

        Self::inc(&mut puz, position);
        // set backmarker to the last cell
        let mut back_marker = puz.num_squares() - 1;

        // This loop increments, checks, determine if solved, and adjust the backmarker to check
        // for additional solutions.
        'solving: loop {
            // check valid
            if puz.was_valid_entry(position) {
                // if valid, check solved. Solved if the position is valid and in the last square.
                if position == puz.num_squares() - 1 {
                    // Copy cell numbers into a new vector to be added to the solutions.
                    let solvec: Vec<V> = puz
                        .value_iter()
                        .map(|sq| sq.val().unwrap_or_else(|| <Bit<V>>::zero()))
                        .map(|bit| <V>::from(bit))
                        .collect();
                    let sol: Grid<V> = Grid::new(solvec, puz.sudoku_size());

                    to_return.push(sol);
                    if to_return.len() > MAX_SOLUTIONS {
                        return Err(SudError::ExcessiveSolutions(MAX_SOLUTIONS));
                    }

                    // reset all after backmarker
                    while position > back_marker {
                        // reset starting position, but not the backmarker
                        Self::reset(&mut puz, position);
                        // decrement the position.  If less than 0, this means that all possible solutions are found.
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => {
                                return match to_return.len() {
                                    0 => Ok(Solution::None),
                                    1 => Ok(Solution::One(to_return[0].to_owned())),
                                    x if x <= MAX_SOLUTIONS => Ok(Solution::Multi(to_return)),
                                    _ => Err(SudError::ExcessiveSolutions(MAX_SOLUTIONS)),
                                }
                            }
                        }
                    }

                    assert_eq!(position, back_marker);

                    //increment the position
                    while !Self::inc(&mut puz, position) {
                        // while !puz.inc_square(position) {
                        Self::reset(&mut puz, position);
                        position = match position.checked_sub(1) {
                            Some(v) => v,
                            None => break 'solving,
                        };

                        back_marker = position;
                    }
                } else {
                    // if valid but not solved,
                    // move to next non-fixed position
                    match Self::move_right(&puz, position) {
                        Some(v) => position = v,
                        // if last cell is fixed, this will check if the puzzle is valid.
                        None => {
                            position = puz.num_squares() - 1;
                            continue 'solving;
                        }
                    };

                    // Increment the position, if possible
                    Self::inc(&mut puz, position);
                }
            } else {
                // if not valid
                // if not at max
                // increment position
                while !Self::inc(&mut puz, position) {
                    // else reset position
                    // move position to next previous non-fixed
                    Self::reset(&mut puz, position);
                    position = match position.checked_sub(1) {
                        Some(v) => v,
                        None => break 'solving,
                    };
                }
            }
        }

        match to_return.len() {
            0 => Ok(Solution::None),
            1 => Ok(Solution::One(to_return[0].to_owned())),
            x if x <= MAX_SOLUTIONS => Ok(Solution::Multi(to_return)),
            _ => Err(SudError::ExcessiveSolutions(MAX_SOLUTIONS)),
        }
    }
}

#[cfg(test)]
pub mod brute_unit {
    use super::*;
    use crate::sudokusize::SudokuSize;
    use crate::*;
    const SIZE: SudokuSize = SudokuSize::Three;
    type TSudoku = Sudoku<u16>;
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    use crate::errors::SudError;

    fn get_example() -> Vec<Vec<u16>> {
        vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]
    }

    fn test_sud() -> TSudoku {
        TSudoku::new_with_size(
            get_example()
                .iter()
                .flatten()
                .cloned()
                .collect::<Vec<u16>>(),
            SIZE,
        )
        .unwrap()
    }

    fn assert_vector<T: PartialEq>(lhs: Vec<T>, rhs: Vec<T>) {
        assert_eq!(lhs.len(), rhs.len());
        for left in lhs {
            assert!(rhs.contains(&left));
        }
    }

    #[test]
    fn simple_checks() -> Result<()> {
        let sud = test_sud();
        let poss1 = sud.possibilities(2)?;
        let expected = vec![1, 2, 4];
        assert_vector(poss1, expected);

        let poss1 = sud.possibilities(3)?;
        let expected = vec![2, 6];
        assert_vector(poss1, expected);
        Ok(())
    }

    #[test]
    fn simple_brute() -> Result<()> {
        let input: Vec<u16> = vec![1, 0, 2, 0, 2, 0, 3, 0, 3, 0, 4, 0, 4, 0, 1, 0];
        let sud = TSudoku::new_with_size(input, SudokuSize::Two)?;
        let _sol = sud.solution();

        Ok(())
    }
    #[test]
    fn move_right_test() -> Result<()> {
        let grid = test_sud().grid;
        let mut pos = BruteForce::move_right(&grid, 0);
        assert_eq!(pos, Some(2));
        pos = BruteForce::move_right(&grid, pos.unwrap());
        assert_eq!(pos, Some(3));

        let pos = BruteForce::move_right(&grid, 77);
        assert_eq!(pos, Some(78));
        let pos = BruteForce::move_right(&grid, pos.unwrap());
        assert_eq!(pos, None);

        Ok(())
    }

    #[test]
    fn increment() -> Result<()> {
        let mut grid = test_sud().grid;
        let ind = 77;
        assert_eq!(grid.items.get(ind).unwrap().val(), None);
        let mut inced = BruteForce::inc(&mut grid, ind);
        assert!(inced);

        assert_eq!(grid.items.get(ind).unwrap().val(), Some(2u16.into()));
        inced = BruteForce::inc(&mut grid, ind);
        assert!(inced);

        assert_eq!(grid.items.get(ind).unwrap().val(), Some(6u16.into()));
        inced = BruteForce::inc(&mut grid, ind);
        assert!(!inced);

        Ok(())
    }

    #[test]
    fn reset_test() -> Result<()> {
        // Just like last test
        let mut grid = test_sud().grid;
        let ind = 77;
        BruteForce::inc(&mut grid, ind);
        BruteForce::inc(&mut grid, ind);
        BruteForce::inc(&mut grid, ind);

        // But now reset
        BruteForce::reset(&mut grid, ind);

        assert_eq!(grid.items.get(ind).unwrap().val(), None);
        let mut inced = BruteForce::inc(&mut grid, ind);
        assert!(inced);

        assert_eq!(grid.items.get(ind).unwrap().val(), Some(2u16.into()));
        inced = BruteForce::inc(&mut grid, ind);
        assert!(inced);

        assert_eq!(grid.items.get(ind).unwrap().val(), Some(6u16.into()));
        inced = BruteForce::inc(&mut grid, ind);
        assert!(!inced);
        Ok(())
    }

    #[test]
    fn mini_brute() -> Result<()> {
        let mut sud = test_sud();
        let mut position = 2;
        assert!(BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 1);
        assert!(sud.was_valid_entry(position));

        position = BruteForce::move_right(&sud.grid, position).unwrap();
        assert_eq!(position, 3);

        assert!(BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 2);
        assert!(sud.was_valid_entry(position));

        position = BruteForce::move_right(&sud.grid, position).unwrap();
        assert_eq!(position, 5);

        assert!(BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 2);
        assert!(!sud.was_valid_entry(position));

        assert!(BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 4);
        assert!(sud.was_valid_entry(position));

        BruteForce::reset(&mut sud.grid, position);
        assert_eq!(sud.get(position)?, 0);
        let expected = vec![2, 4, 6, 8];
        assert_vector(sud.possibilities(position).unwrap(), expected);

        position = 3;
        assert!(BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 6);
        assert!(sud.was_valid_entry(position));

        assert!(!BruteForce::inc(&mut sud.grid, position));
        assert_eq!(sud.get(position)?, 6);
        Ok(())
    }
    #[test]
    fn sudoku_test() -> Result<()> {
        let example = get_example();

        let expected: Vec<u16> = (vec![
            vec![5, 3, 4, 6, 7, 8, 9, 1, 2],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![3, 4, 5, 2, 8, 6, 1, 7, 9],
        ])
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u16>>();

        let already_solved = TSudoku::new_with_size(expected.clone(), SudokuSize::Three)?;
        let solved = already_solved.solution();
        match solved.clone() {
            Solution::One(v) => assert_eq!(expected, v.items),
            _ => assert!(false, "Not a direct match:\n{:?}", solved),
        }

        let res = TSudoku::new_with_size(
            example.iter().flatten().cloned().collect::<Vec<u16>>(),
            SIZE,
        )?;
        let res = res.unique_solution()?;

        assert!(res.iter().zip(expected.iter()).all(|(v, e)| { *v == *e }));

        Ok(())
    }

    #[test]
    fn two_solutions() {
        let example: Vec<Vec<u16>> = vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 0, 0],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 0, 0],
        ];

        let expected1: Vec<u16> = (vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 2, 7],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 7, 2],
        ])
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u16>>();

        let expected2: Vec<u16> = (vec![
            vec![2, 9, 5, 7, 4, 3, 8, 6, 1],
            vec![4, 3, 1, 8, 6, 5, 9, 7, 2],
            vec![8, 7, 6, 1, 9, 2, 5, 4, 3],
            vec![3, 8, 7, 4, 5, 9, 2, 1, 6],
            vec![6, 1, 2, 3, 8, 7, 4, 9, 5],
            vec![5, 4, 9, 2, 1, 6, 7, 3, 8],
            vec![7, 6, 3, 5, 2, 4, 1, 8, 9],
            vec![9, 2, 8, 6, 7, 1, 3, 5, 4],
            vec![1, 5, 4, 9, 3, 8, 6, 2, 7],
        ])
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u16>>();

        if let Solution::Multi(vec) = TSudoku::new_with_size(
            example.iter().flatten().cloned().collect::<Vec<u16>>(),
            SIZE,
        )
        .unwrap()
        .solution()
        {
            assert_eq!(vec.len(), 2, "{:?}", &vec);
            if vec[0].iter().zip(expected1.iter()).all(|(a, e)| *a == *e) {
                assert!(vec[1].iter().zip(expected2.iter()).all(|(a, e)| *a == *e));
            } else {
                assert!(vec[0].iter().zip(expected2.iter()).all(|(a, e)| *a == *e));
                assert!(vec[1].iter().zip(expected1.iter()).all(|(a, e)| *a == *e));
            }
        } else {
            assert!(false, "There was only 1 solution")
        }
    }

    #[test]
    fn oh_no_test() {
        let example: Vec<Vec<u16>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![6, 7, 2, 1, 9, 5, 3, 4, 8],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![2, 8, 7, 4, 1, 9, 6, 3, 5],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        let res = TSudoku::new_with_size(
            example.iter().flatten().cloned().collect::<Vec<u16>>(),
            SIZE,
        )
        .unwrap();
        assert!(
            match res.solution() {
                Solution::Multi(vec) => vec.len() == 2,
                _ => false,
            },
            "{:?}",
            res.solution()
        );

        let example: Vec<Vec<u16>> = vec![
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![1, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        // used https://www.thonky.com/sudoku/solution-count to verify solution count
        let res = TSudoku::new_with_size(
            example.iter().flatten().cloned().collect::<Vec<u16>>(),
            SIZE,
        );
        if let Err(SudError::ExcessiveSolutions(_)) = res {
        } else {
            assert!(false, "expected SudError::ExcessiveSolutions")
        }

        let example: Vec<Vec<u16>> = vec![
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 9, 8, 3, 4, 2, 5, 6, 7],
            vec![8, 5, 9, 7, 6, 1, 4, 2, 3],
            vec![4, 2, 6, 8, 5, 3, 7, 9, 1],
            vec![7, 1, 3, 9, 2, 4, 8, 5, 6],
            vec![9, 6, 1, 5, 3, 7, 2, 8, 4],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

        // used https://www.thonky.com/sudoku/solution-count to verify solution count
        let res = TSudoku::new_with_size(
            example.iter().flatten().cloned().collect::<Vec<u16>>(),
            SIZE,
        );

        if let Err(SudError::ExcessiveSolutions(_)) = res {
        } else {
            assert!(false, "expected SudError::ExcessiveSolutions")
        }
    }
}
