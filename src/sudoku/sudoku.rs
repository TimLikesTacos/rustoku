//! This library solves sudoku puzzles using human style techniques, along with traditional brute force.
use crate::bitset::{Bit, BitSet, BitSetInt};
use crate::brute::BruteForce;
use crate::grid::grid_traits::*;
use crate::sudoku::initial_input::*;

use std::fmt::{Debug, Display, Formatter};

// Public Exports
pub use crate::errors::SudError;

pub use crate::move_change::*;
pub(crate) use crate::square::Square;
// pub use brute::BruteForce;
// pub use solution::*;
//
//pub use crate::human_calcs::humansolve::HumanSolve;
// pub use crate::human_calcs::technique::Technique;

use crate::grid::grid::Grid;
use crate::grid::gridcoord::{HouseCoord, InputCoord};
use crate::grid::house::House;
use crate::grid::types::Index;
use crate::grid::{Col, Coord, GridBox, Row};
use crate::hint::Hint;
use crate::human_calcs::humansolve::solver::HumanSolver;
use crate::human_calcs::technique::Technique;
//use crate::human_calcs::technique::Technique;
use crate::solution::Solution;
use crate::sudoku::setinputs::InputValue;
// use crate::sudoku::sudvalue::SudokuValue;
use crate::sudokusize::{Res, SudokuSize};
//use crate::human_calcs::technique;

/// A `Grid` contains `Squares` and all functions required to maneuver through the `Grid`.
/// Data is stored in a 1-D vector, which simulates a 2-D square grid, normally 9 x 9.
/// The 1-D vector is row dominate (squares in the same row are next to each other in the vector (and
/// therefore memory)).
///
/// Indexing into the puzzle is done by types that implement `InputCoord`, which is either `usize` or `(usize, usize)`.
/// A single usize represents the sudoku puzzle if represented in a row dominate 1-D vector.
/// The tuple is in `row, column` format.  So for a normal 9x9 sudoku, and index of `10` is the same as `(1, 1)`.
/// For a 16x16 puzzle, and tuple of `(1, 1)` would be an index of 17.
///
/// Any index outside the range of the puzzle boundary will return an error, which is why most functions
/// return a `Result`
///
/// `Sudoku` is designed to use different sized integers for different sized puzzles.  For a 9x9, a u16 is sufficient.
/// It is recommended to use the type definitions is the `rustoku::basic`, `rusoku::medium`,
/// `rustoku::large`, or `rustoku::xlarge` for the appropriate puzzle size. This will allow specifying the integer type to use.
#[derive(Debug, Clone)]
pub struct Sudoku<V: BitSetInt> {
    // 'grid' element is where the squares are stored.
    pub(crate) grid: Grid<Square<V>>,
    // The next three elements store common row/col/box items
    rows: Vec<BitSet<V>>,
    cols: Vec<BitSet<V>>,
    boxes: Vec<BitSet<V>>,
    pub(crate) unsolved: usize,
    moves: Vec<Move<V>>,
    solution: Solution<V>,
    human_solver: HumanSolver<V>,
}

impl<V: BitSetInt> Sudoku<V> {
    /// Creates a new Rustoku puzzle.
    /// Takes as input a `&str` with empty squares represented by either a 0 or a non numeric character.
    /// Also accepts a  one dimensional `Vec` of non negative integers.
    /// A `SudError` is returned if the length of the input is invalid or the number exceed the max for the puzzle (nominally 9).
    /// A solution is calculated by performing brute force automatically.
    ///
    /// Takes as input either a string (for 9x9 puzzles only for now), 1-D vectors, or 2-D vectors
    /// ```
    /// use rustoku::basic::*;
    ///
    /// let input_string= "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
    /// let sudoku = Sudoku::new(input_string).expect("Invalid sudoku input");
    /// assert!(sudoku.unique_solution().is_ok());
    /// ```
    pub fn new<T: InitialInput<V>>(input: T) -> Res<Sudoku<V>> {
        let input_vec: Vec<V> = input.initial()?;
        let size = SudokuSize::size_from_input_length(input_vec.len())?;
        Self::new_with_size(input_vec, size)
    }

    /// Build sudoku. Validates input length and determines solution (if any)
    pub(crate) fn new_with_size<T: InitialInput<V>>(input: T, size: SudokuSize) -> Res<Sudoku<V>> {
        let input_vec: Vec<V> = input.initial()?;

        if input_vec.len() != size.total() {
            return Err(SudError::InputLengthError(input_vec.len()));
        }

        // Construct new grid with values and fixed established for the squares
        let mut squares = Vec::with_capacity(size.total());
        for sq in input_vec
            .iter()
            .enumerate()
            .map(|(i, v)| Square::new((*v).into(), size, i))
        {
            match sq {
                Ok(s) => squares.push(s),
                Err(e) => return Err(e),
            }
        }

        let grid = Grid::new(squares, size);

        // This will build the groups, and identify what groups have the numbers taken.
        // This will help in marking the initial potential values for each square

        let rows = build_house::<Row, V>(&grid);
        let cols = build_house::<Col, V>(&grid);
        let boxes = build_house::<GridBox, V>(&grid);

        let mut newgrid = Sudoku {
            grid,
            rows,
            cols,
            boxes,
            unsolved: size.total(), // will be updated later
            moves: vec![],
            solution: Solution::NotSet,
            human_solver: HumanSolver::new(),
        };

        // Go over each square and find which possiblities that can be removed.
        let mut ind = 0;
        let totalsize = size.total();
        while ind < totalsize {
            if newgrid.grid[ind].is_fixed() {
                newgrid.unsolved -= 1;
                ind += 1;
                continue;
            }

            let to_remove = newgrid.all_groups_values(ind);
            newgrid.grid[ind].set_original(to_remove, size);
            ind += 1;
        }

        newgrid.check_for_conflicts().map_err(|_count| {
            SudError::IllegalOperation("There are conflicting values in the input puzzle")
        })?;
        newgrid.solution = BruteForce::solve(&newgrid.grid)?;
        Ok(newgrid)
    }

    /// Sets the value in the square.  Checks that the value is a valid possibility, returns `SudError::ValueNotPossible` if not.
    /// Returns `SudError::InvalidLocation` if index outside bounds.  Accepts usize 1-D index or `(usize, usize)` pair as `(rol, col)`
    /// Accepts as input an integer that implements `GridInput`, or a `SingleBit`,
    /// ```
    /// # fn main () -> Result<(), Box<dyn std::error::Error>>{
    /// use rustoku::basic::*;
    ///
    /// let input_string= "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
    /// let mut sudoku = Sudoku::new(input_string)?;
    /// // The twelfth square is r1c3
    /// assert_eq!(sudoku.get(12), Ok(0)); // Index of 1-d
    /// assert_eq!(sudoku.get((1, 3)), Ok(0)); // Index of 2-d
    /// let remaining_before = sudoku.remaining();
    /// // Valid possibilities are 4, 6, 7, 9
    /// sudoku.set(12, 4).expect("Should have set 4");
    /// assert_eq!(sudoku.get(12), Ok(4)); // Index of 1-d
    /// assert_eq!(sudoku.get((1, 3)), Ok(4)); // Index of 2-d
    /// assert_eq!(sudoku.remaining(), remaining_before - 1);
    /// # Ok(())}
    /// ```
    #[inline]
    pub fn set<C: InputCoord, I: Into<Bit<V>>>(&mut self, index: C, value: I) -> Res<&Move<V>> {
        self.set_with_technique(index, value, None)
    }

    /// Sets the value in the square.  Checks that the value is a valid possibility, returns `SudError::ValueNotPossible` if not.
    /// Returns `SudError::InvalidLocation` if index outside bounds.  Accepts usize 1-D index or `(usize, usize)` pair as `(rol, col)`
    /// Accepts as input an integer that implements `GridInput`, or a `SingleBit`,
    pub(crate) fn set_with_technique<C: InputCoord, I: Into<Bit<V>>>(
        &mut self,
        index: C,
        value: I,
        technique: Option<Technique>,
    ) -> Res<&Move<V>> {
        let value = value.into();
        let index = index.to_usize(&self.grid.size)?;
        // Direct indexing is okay here as it is checked in the line above.

        if !self.grid[index].poss_contains(value) {
            Err(SudError::ValueNotPossible(format!("{:?}", value)))
        } else {
            // Set the value.  Also clears the possibilities.
            if self.grid[index].val().is_none() {
                self.unsolved -= 1;
            }
            // Get the possibilites that will be removed so that can be added to the `move`
            let possibils = *self.grid[index].poss();

            // Set the value and clear the possibilities
            self.grid[index].set_value(Some(value));

            let mut themove: Move<V> = Move::new();
            themove.set_method(technique);

            // Add the square that we are setting the value of to the move.
            themove.set_value(self.upgrade_index(index), value.into(), technique.is_none());
            // Remove the potential values from the square
            themove.add_removed_potential(self.upgrade_index(index), possibils);

            // Update any square affected by this.
            self.update_potentials(index, &value, &mut themove);
            self.moves.push(themove);
            Ok(&self.moves[self.moves.len() - 1])
        }
    }

    /// Removes potential value from a square
    /// ```
    ///# fn main () -> Result<(), Box<dyn std::error::Error>>{
    ///use rustoku::basic::*;
    ///
    /// let input_string= "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
    /// let mut sudoku = Sudoku::new(input_string)?;
    /// assert_eq!(sudoku.possibilities((2, 2))?, vec![2, 6, 9]);
    /// sudoku.remove_potential((2,2), 6);
    /// assert_eq!(sudoku.possibilities((2, 2))?, vec![2, 9]);
    /// # Ok(())}
    /// ```
    pub fn remove_potential<C: InputCoord, I: Into<Bit<V>>>(
        &mut self,
        index: C,
        value: I,
    ) -> Res<Move<V>> {
        let value = value.into();
        let index = index.to_usize(&self.grid.size)?;
        if !self._get(index).poss_contains(value) {
            Err(SudError::ValueNotPossible(format!("{:?}", index)))
        } else {
            let mut themove: Move<V> = Move::new();
            themove.set_method(None);
            // Remove the potential values from the square
            themove.add_removed_potential(self.upgrade_index(index), BitSet::new().insert(value));

            self.get_mut(index).unwrap().remove_pot(value);

            // todo: When implementing the row,col,box tuplectrs, need to update those for any potential removal.
            self.moves.push(themove.clone());
            Ok(themove)
        }
    }

    pub(crate) fn remove_potentials_from_hint(&mut self, hint: Hint<V>) -> Res<Move<V>> {
        let _move = hint.0;
        for pair in _move.removed_potentials.iter() {
            let index = pair.index();
            self.get_mut(index)?.remove_set(*pair.value());
        }
        self.moves.push(_move.clone());

        Ok(_move)
    }
    /// Determines if the value can be a valid entry.  Used before the value is set.
    /// Can be used to determine if a value will be a valid entry.
    /// ```
    /// # fn main () -> Result<(), Box<dyn std::error::Error>>{
    ///use rustoku::basic::*;
    ///
    /// let input_string= "..53.....8......2..7..1.5..4....53...1..7...6..32...8..6.5....9..4....3......97..";
    /// let mut sudoku = Sudoku::new(input_string)?;
    ///
    /// // Possible values for r1c3 are 4, 6, 7, 9.
    /// assert!(sudoku.is_valid_entry((1, 3), 4)?);
    /// assert!(!sudoku.is_valid_entry((1, 3), 8)?);
    /// # Ok(())}
    pub fn is_valid_entry<C: InputCoord>(&self, index: C, value: V) -> Res<bool>
    where
        V: BitSetInt + InputValue,
    {
        let v = value.checked_into_bit(self.sudoku_size())?;
        let index = index.to_usize(&self.grid.size)?;
        Ok(self._get(index).poss_contains(v))
    }

    /// Returns the number of unsolved squares.  Does not check for correct placement
    #[inline]
    pub fn remaining(&self) -> usize {
        self.unsolved
    }

    #[inline]
    #[allow(dead_code)]
    /// Currently only used in testing. todo remove
    fn get_square<C: InputCoord>(&self, coord: C) -> Result<&Square<V>, SudError> {
        Ok(self._get(coord.to_usize(&self.sudoku_size())?))
    }

    #[inline]
    pub(crate) fn _get(&self, index: usize) -> &Square<V> {
        &self.grid[index]
    }

    #[inline]
    pub fn get<C: InputCoord>(&self, index: C) -> Result<V, SudError> {
        let index = index.to_usize(&self.grid.size)?;
        Ok(self
            ._get(index)
            .val()
            .unwrap_or_else(|| <Bit<V>>::zero())
            .into())
    }
    #[inline]
    pub(crate) fn get_mut<C: InputCoord>(&mut self, index: C) -> Res<&mut Square<V>> {
        Ok(self._get_mut(index.to_usize(&self.grid.size)?))
    }

    #[inline]
    pub(crate) fn _get_mut(&mut self, index: usize) -> &mut Square<V> {
        &mut self.grid[index]
    }
    #[inline]
    /// Checks if the amount of unsolved squares is zero.  Does not check for correct placement
    pub fn is_solved(&self) -> bool {
        self.unsolved == 0 && self.compare_with_solution().unwrap_or(false)
    }

    #[inline]
    /// Human technique solves the puzzle. Returns a tuple of the solution, and a Vec\<Moves\>
    pub fn human(&self, solver: &mut HumanSolver<V>) -> Solution<V> {
        match solver.solve(self) {
            Ok((sol, moves)) => Solution::HumanSolved(sol.to_inner(), moves),
            Err(_) => Solution::None,
        }
    }

    #[inline]
    /// Returns `Ok` of the possible values for a certain square.  If the vector is empty, there
    /// are no possibilities, which would represent either a square that has a value set, or an invalidly solved puzzle
    pub fn possibilities<T: InputCoord>(&self, index: T) -> Res<Vec<V>> {
        let index = index.to_usize(&self.grid.size)?;
        Ok(self
            ._get(index)
            .poss()
            .iter()
            .map(|bit| bit.into())
            .collect())
    }

    #[inline]
    /// Gets the width of the board.  This is equivalent to the maximum number for values. In a typical
    /// sudoku, this value is 9.  
    pub fn house_size(&self) -> usize {
        self.grid.house_size()
    }

    #[inline]
    /// The total number of squares.  For a 9X9, this is 81
    pub fn num_squares(&self) -> usize {
        self.grid.num_squares()
    }

    #[inline]
    /// The dimension of the box in one direction.  Is the square root of the board length. For
    /// a 9X9, this is 3.
    pub fn box_dimension(&self) -> usize {
        self.grid.box_size()
    }

    #[inline]
    /// Takes an index as a usize and returns a Index
    pub(crate) fn upgrade_index(&self, ind: usize) -> Index {
        Index {
            inner_index: ind,
            size: self.sudoku_size(),
        }
    }

    #[inline]
    /// Returns an iterator that iterates over each square's value in a row dominate fashion.
    pub fn value_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = V> + 'b {
        self.grid
            .value_iter()
            .map(|sq| sq.val().unwrap_or_else(|| <Bit<V>>::zero()))
            .map(|bit| <V>::from(bit))
    }

    /// Returns a Set of a union between the box, col, and row groups for the associated square
    pub(crate) fn all_groups_values(&self, index: usize) -> BitSet<V> {
        let (row, col, box_num) = self.grid.calc_house(index);
        self.rows[row]
            .union(self.cols[col])
            .union(self.boxes[box_num])
    }

    /// This requires `set()` to be used to set values to make sure that
    /// the value entered is a valid possibility
    pub fn undo(&mut self) -> Option<Move<V>> {
        if let Some(last_move) = self.moves.pop() {
            if let Some(pair) = last_move.change() {
                undo_set(self, pair);
            }

            for pair in last_move.removed_potentials_vec().iter() {
                undo_pot(self, pair);
            }

            return Some(last_move);
        } else {
            return None;
        }

        fn undo_set<B2: BitSetInt>(puz: &mut Sudoku<B2>, pair: &IndexValuePair<B2>) {
            let index = pair.index().into();
            let value: BitSet<B2> = (*pair.value()).into().into();

            let (row, col, thebox) = puz.grid.calc_house(index);
            puz.grid[index].set_value(None);
            puz.unsolved += 1;

            // Remove the value from the group trackers
            puz.boxes[thebox] = puz.boxes[thebox].difference(value);
            puz.rows[row] = puz.boxes[row].difference(value);
            puz.cols[col] = puz.boxes[col].difference(value);
        }

        fn undo_pot<B2: BitSetInt>(puz: &mut Sudoku<B2>, pair: &IndexValuePair<BitSet<B2>>) {
            let index: usize = pair.index().into();
            let value = pair.value();

            puz.grid[index].insert_set(*value);
        }
    }

    /// Removes potential values as if a value was set. All associated potentials in the row, columnm, and box will be updated.
    fn update_potentials(&mut self, index: usize, value: &Bit<V>, themove: &mut Move<V>) {
        #[inline]
        fn update_poss<'b, 'a: 'b, V1: BitSetInt>(
            size: SudokuSize,
            it: impl Iterator<Item = &'a mut Square<V1>> + 'b,
            value: &'a Bit<V1>,
            themove: &mut Move<V1>,
        ) {
            for sq in it {
                if sq.poss_contains(*value) {
                    sq.remove_pot(*value);

                    // Add this to the move
                    themove.add_removed_potential(
                        Index {
                            inner_index: sq.index(),
                            size,
                        },
                        BitSet::new().insert(*value),
                    );
                }
            }
        }

        let (row, col, box_num) = self.grid.calc_house(index);

        self.rows[row].insert(*value);
        self.cols[col].insert(*value);
        self.boxes[box_num].insert(*value);

        update_poss(
            self.sudoku_size(),
            self.grid.house_iter_mut(Row::start(row, self.grid.size)),
            value,
            themove,
        );
        update_poss(
            self.sudoku_size(),
            self.grid.house_iter_mut(Col::start(col, self.grid.size)),
            value,
            themove,
        );
        update_poss(
            self.sudoku_size(),
            self.grid
                .house_iter_mut(GridBox::start(box_num, self.grid.size)),
            value,
            themove,
        );
    }

    /// Determines if the value that is currently set is valid.  Used after the value is set.  Can
    /// be used to determine if the initial puzzle is valid.
    pub(crate) fn was_valid_entry(&self, index: usize) -> bool {
        if let Some(v) = self._get(index).val() {
            self.grid
                .single_iterator(index)
                .filter(|x| x.val() == Some(v))
                .count()
                == 3
        } else {
            true
        }
    }

    /// Checks for conflicts, returns Ok(()) if no conflicts, Err with the number of conflicting squares
    pub fn check_for_conflicts(&self) -> Result<(), usize> {
        let mut count = 0;
        for i in 0..self.grid.num_squares() {
            if !self.was_valid_entry(i) {
                count += 1;
            }
        }
        if count == 0 {
            Ok(())
        } else {
            Err(count)
        }
    }

    pub fn solution(&self) -> &Solution<V> {
        &self.solution
    }

    /// Compares that the filled in values match the solution. Skips any value that is not filled in.
    pub fn compare_with_solution(&self) -> Result<bool, SudError> {
        let sol = self.solution.get()?;
        let matches = self
            .grid
            .iter()
            .zip(sol.iter())
            .filter(|(s, _)| s.value.is_some())
            .all(|(s, sol_value)| <V>::from(s.val().unwrap()) == *sol_value);
        Ok(matches)
    }

    /// Compares sudoku with solution. Only applicable when there is only one solution. Else an `Err`
    /// will be returned.  If all squares are filled in and match the solution `Ok(())` is returned, else a
    /// `SudoError::NotSolved` is returned with the number of missing values and conflicts.
    pub fn validate_against_solution(&self) -> Result<(), SudError> {
        let grid_iter = self.value_iter();
        let sol = self.solution.get()?;

        let conflicts = grid_iter
            .zip(sol.iter())
            .filter(|(sq, sol)| sq != *sol)
            .count();
        if conflicts == 0 && self.remaining() == 0 {
            Ok(())
        } else {
            Err(SudError::NotSolved(self.remaining(), conflicts))
        }
    }
    // todo this should just be solution
    /// Returns `Ok(&Grid)` if the puzzle is valid.  If not, returns a `SudError` for either no solution or multiple solutions.
    pub fn unique_solution(&self) -> Result<&Grid<V>, SudError> {
        match &self.solution {
            Solution::None => Err(SudError::NoSolution),
            Solution::Multi(vecs) => Err(SudError::MultipleSolution(vecs.len())),
            Solution::One(grid) | Solution::HumanSolved(grid, _) => Ok(grid),
            Solution::NotSet => Err(SudError::HasNotBeenSolved),
        }
    }

    /// Returns the total number of squares
    pub(crate) fn sudoku_size(&self) -> SudokuSize {
        self.grid.size
    }

    /// Iterator over a `house`.
    pub(crate) fn house_iter<'b, 'a: 'b>(
        &'a self,
        house: House<usize>,
    ) -> Box<dyn Iterator<Item = &'a Square<V>> + 'b> {
        match house {
            House::Row(n) => Box::new(self.grid.house_iter(Row::start(n, self.sudoku_size()))),
            House::Col(n) => Box::new(self.grid.house_iter(Col::start(n, self.sudoku_size()))),
            House::Box(n) => Box::new(self.grid.house_iter(GridBox::start(n, self.sudoku_size()))),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn house_iter_mut<'b, 'a: 'b>(
        &'a mut self,
        house: House<usize>,
    ) -> Box<dyn Iterator<Item = &'a mut Square<V>> + 'b> {
        match house {
            House::Row(n) => Box::new(self.grid.house_iter_mut(Row::start(n, self.sudoku_size()))),
            House::Col(n) => Box::new(self.grid.house_iter_mut(Col::start(n, self.sudoku_size()))),
            House::Box(n) => Box::new(
                self.grid
                    .house_iter_mut(GridBox::start(n, self.sudoku_size())),
            ),
        }
    }

    /// Finds the solution and required moves to solve the current as-is puzzle
    pub fn human_solve(&self) -> Result<(Grid<V>, Vec<Move<V>>), SudError> {
        match self.human_solver.solve(self) {
            Ok((sq, moves)) => Ok((
                sq.iter()
                    .map(|sq| <V>::from(sq.val().unwrap_or_else(|| <Bit<V>>::zero())))
                    .collect(),
                moves,
            )),
            Err(e) => Err(e),
        }
    }
}

fn build_house<C: HouseCoord, V: BitSetInt>(grid: &Grid<Square<V>>) -> Vec<BitSet<V>> {
    let mut houses = Vec::with_capacity(grid.size.house_size());
    for iter in grid.house_iters::<C>() {
        houses.push(iter.fold(BitSet::empty(), |acc, x| {
            if let Some(val) = x.value {
                acc.insert(val)
            } else {
                acc
            }
        }))
    }
    houses
}

impl<V: BitSetInt> PartialEq for Sudoku<V> {
    fn eq(&self, other: &Sudoku<V>) -> bool {
        self.grid == other.grid
    }
}

impl<V: BitSetInt> Rotate for Sudoku<V> {
    type PuzzleInp = Sudoku<V>;

    fn cwrotate(self, quarter_rotations: usize) -> Res<Self> {
        let mut ret: Vec<V> = self.value_iter().collect();
        let house: usize = self.house_size();
        let size = self.sudoku_size();

        let index_calc = |row, col, size| -> usize { Coord::new(row, col, size).to_usize() };

        for _ in 0..quarter_rotations {
            for x in 0..house / 2 {
                for y in x..house - 1 - x {
                    let ul = index_calc(x, y, size);
                    let ur = index_calc(y, house - 1 - x, size);
                    let ll = index_calc(house - 1 - y, x, size);
                    let lr = index_calc(house - 1 - x, house - 1 - y, size);

                    ret.swap(ul, ll);
                    ret.swap(ll, lr);
                    ret.swap(lr, ur);
                }
            }
        }
        Sudoku::new_with_size(ret, size)
    }
}

impl<V: BitSetInt> Display for Sudoku<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.value_iter().fold("".to_owned(), |acc, x| {
            if x.is_zero() {
                acc + "."
            } else {
                acc + &format!("{:?}", x)
            }
        });
        write!(f, "{}", &str)
    }
}

#[cfg(test)]
mod sudoku_tests {

    use super::*;

    type TSudoku = Sudoku<u16>;
    const SIZE: SudokuSize = SudokuSize::Three;

    fn basic_sud() -> TSudoku {
        // http://hodoku.sourceforge.net/en/tech_singles.php#h1
        // ^^^ Hidden Single
        Sudoku::<u16>::new_with_size(
            ".28..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        )
        .unwrap()
    }

    macro_rules! vecinput {
        ($int: ty) => {
            vec![
                5 as $int, 3, 0, 0, 7, 0, 0, 0, 0, 6, 0, 0, 1, 9, 5, 0, 0, 0, 0, 9, 8, 0, 0, 0, 0,
                6, 0, 8, 0, 0, 0, 6, 0, 0, 0, 3, 4, 0, 0, 8, 0, 3, 0, 0, 1, 7, 0, 0, 0, 2, 0, 0, 0,
                6, 0, 6, 0, 0, 0, 0, 2, 8, 0, 0, 0, 0, 4, 1, 9, 0, 0, 5, 0, 0, 0, 0, 8, 0, 0, 7, 9,
            ]
        };
    }

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

    #[allow(dead_code)]
    fn index(v: usize) -> Index {
        Index::new(v, SIZE)
    }

    #[test]
    fn was_valid_entry_valid_base() -> Res<()> {
        let sud = test_sud();
        for i in 0..SIZE.total() {
            assert!(sud.was_valid_entry(i), "{:?}", i);
        }
        Ok(())
    }

    #[test]
    fn set() -> Res<()> {
        let mut sud = test_sud();
        let ind = 2usize;
        sud.set(ind, 1)?;
        assert_eq!(sud._get(ind).val(), Some(1.into()));
        assert!(sud.was_valid_entry(ind));
        assert_eq!(sud.get(ind).unwrap(), 1);

        let ind = 3;
        sud.set(ind, 2)?;
        assert_eq!(sud._get(ind).val(), Some(2.into()));
        assert!(sud.was_valid_entry(ind));
        assert_eq!(sud.get(ind)?, 2);

        Ok(())
    }

    #[test]
    fn set_bad() -> Res<()> {
        let mut sud = test_sud();
        let ind = 1;

        assert!(sud.set(ind, 3).is_err());
        assert!(sud.set(ind, 9).is_err());

        let ind = 2;

        assert!(sud.set(ind, 3).is_err());
        assert!(sud.set(ind, 10).is_err());

        Ok(())
    }

    #[test]
    fn unselv() -> Res<()> {
        let mut sud = test_sud();

        let unsolved = sud.remaining();
        assert!(unsolved > 20 && unsolved < 70);
        let ind = 2;
        sud.set(ind, 2)?;

        let oneless = sud.remaining();
        assert_eq!(oneless, unsolved - 1);

        Ok(())
    }

    #[test]
    fn new() -> Res<()> {
        // Good puzzle, should not return error
        let s = TSudoku::new_with_size(
            ".28..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        )?;
        let s_sol = s.unique_solution()?;

        // With utf mutibit character
        let t = TSudoku::new_with_size("😇28😇😇7😇😇😇😇16😇83😇7😇😇😇😇😇2😇85113729😇😇😇😇😇😇😇73😇😇😇😇😇😇😇😇463😇729😇😇7😇😇😇😇😇😇😇86😇14😇😇😇😇3😇😇7😇😇", SIZE)?;
        assert_eq!(s, t);
        let t_sol = t.unique_solution()?;
        assert_eq!(s_sol, t_sol);

        // With utf mutibit character
        let t = TSudoku::new_with_size(
            "028007000016083070000020851137290000000730000000046307290070000000860140000300700",
            SIZE,
        )?;
        assert_eq!(s, t);
        let t_sol = t.unique_solution()?;
        assert_eq!(s_sol, t_sol);

        // This puzzle is invalid.
        let s = TSudoku::new_with_size(
            ".27..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        );
        assert!(s.is_err());

        // Short by one puzzle string
        let s = TSudoku::new_with_size(
            ".28.7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        );
        match s {
            Err(_e) => (),
            x => assert!(false, "{:?}", x),
        }

        let mut s = vecinput!(u16);
        s.pop();
        let s = TSudoku::new_with_size(s, SIZE);
        match s {
            Err(_e) => (),
            x => assert!(false, "{:?}", x),
        }

        // Long by one puzzle string
        let s = TSudoku::new_with_size(
            ".28...7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        );
        match s {
            Err(_e) => (),
            x => assert!(false, "{:?}", x),
        }

        let mut s = vecinput!(u16);
        s.push(2u16);
        let s = TSudoku::new_with_size(s, SIZE);
        match s {
            Err(_e) => (),
            x => assert!(false, "{:?}", x),
        }

        // negative sign
        let s = TSudoku::new_with_size(
            ".-28..7....16.83.7.....2.85113729.......73........463.729..7.......86.14....3..7..",
            SIZE,
        );
        match s {
            Err(_e) => (),
            x => assert!(false, "{:?}", x),
        }

        fn test_okay<T: InitialInput<V>, V: BitSetInt>(inputvec: T) {
            let s: Result<Sudoku<V>, _> = Sudoku::new_with_size(inputvec, SIZE);
            assert!(s.is_ok());
        }

        // Regular 1-D vector, should not error.
        test_okay(vecinput!(u16));
        test_okay(vecinput!(u32));
        test_okay(vecinput!(u128));
        test_okay(vecinput!(u64));

        Ok(())
    }

    #[test]
    fn validatetest() -> Res<()> {
        let s = basic_sud();
        assert!(s.unique_solution().is_ok(), "{:?}", s.unique_solution());

        // Should have two solutios
        let multi = TSudoku::new_with_size(
            ".28..7....16.83.7.....2.8511372........73........463.729..7.......86.14....3..7..",
            SIZE,
        )?;

        match multi.unique_solution() {
            Err(e) => match e {
                SudError::MultipleSolution(_) => (),
                x => assert!(false, "{:?}", x),
            },
            x => assert!(false, "{:?}", x),
        }

        //Invalid puzzle, conflicting in the first row
        let conflict = TSudoku::new_with_size(
            ".78..7....16.83.7.....2.8511372........73........463.729..7.......86.14....3..7..",
            SIZE,
        );

        assert!(conflict.is_err());

        Ok(())
    }

    fn expected_poss<T: PartialEq>(expected: &[T], poss: &[T]) {
        assert_eq!(poss.len(), expected.len());
        for exp in expected {
            assert!(poss.contains(exp));
        }
    }

    #[test]
    fn setv() -> Res<()> {
        let mut s = basic_sud();
        // test_oos!(s, set, 1u16);
        //test_values_oos!(s, set);

        // Index 37 or row4, col 1
        let expected: Vec<u16> = vec![4, 5, 6, 8];
        let poss = s.possibilities(Coord::new(4, 1, SIZE).to_usize()).unwrap();

        // All the possiblities are expected.
        expected_poss(&expected, &poss);
        let _remaining = s.remaining();

        assert_eq!(s._get(Coord::new(4, 1, SIZE).to_usize()).val(), None);

        //Check invalid entry. Nothing should change
        assert!(s.set(Index::new(37, SIZE), 2u16).is_err());

        //Make sure nothing has changed.
        assert_eq!(s.get_square((4, 1))?.val(), None);
        expected_poss(&expected, &s.possibilities((4, 1))?);
        assert_eq!(s.remaining(), _remaining);

        // Now change it to a valid number
        s.set(37, 5)?;

        assert_eq!(s.get_square((4, 1))?.val(), Some(Bit::from(5)));
        assert_eq!(s.possibilities((4, 1))?.len(), 0);

        //Should have removed the value of 5 from the possibilities of affected squares
        assert!(!s.possibilities((7, 1))?.contains(&5));
        assert!(!s.possibilities((5, 0))?.contains(&5));
        assert!(!s.possibilities((4, 5))?.contains(&5));

        assert_eq!(s.remaining(), _remaining - 1);

        Ok(())
    }

    #[test]
    fn undotest() -> Res<()> {
        let mut s = basic_sud();

        // Now change it to a valid number
        s.set(37, 5u16)?;

        assert_eq!(s.get((4, 1))?, 5);

        // Checks to make sure possibilties have been removed
        assert_eq!(s.possibilities((4, 1))?.len(), 0);
        assert!(!s.possibilities((7, 1))?.contains(&5));
        assert!(!s.possibilities((5, 0))?.contains(&5));
        assert!(!s.possibilities((4, 5))?.contains(&5));

        let remaining = s.remaining();

        s.undo();

        let expected: Vec<u16> = vec![4, 5, 6, 8];
        let poss = s.possibilities((4, 1)).unwrap();

        // All the possiblities are expected.
        expected_poss(&expected, &poss);

        // Should have return 5 to possiblities in row, column and box
        assert!(s.possibilities((7, 1))?.contains(&5));
        assert!(s.possibilities((5, 0))?.contains(&5));
        assert!(s.possibilities((4, 5))?.contains(&5));

        assert_eq!(s.remaining(), remaining + 1);

        Ok(())
    }

    #[test]
    fn completeundo() {
        let mut s = basic_sud();
        let original_remaining = s.remaining();
        let original_grid = s.grid.clone();

        while s.remaining() > 0 {
            if let Some(themove) = s.human_solver.next(&s) {
                themove.apply(&mut s);
            } else {
                panic!("Unsolvable Puzzle");
            }
        }
        assert_eq!(s.remaining(), 0);
        assert!(s.is_solved());

        let grid = s.grid.clone();
        let moves = s.moves.clone();

        let mut reverse_count = 0;
        // Undo the entire puzzle
        while s.undo().is_some() {
            reverse_count += 1;
        }

        assert_eq!(s.moves.len(), 0);
        assert_eq!(reverse_count, moves.len());
        assert_eq!(s.remaining(), original_remaining);
        assert_eq!(s.grid, original_grid);

        assert!(s.compare_with_solution().is_ok());

        let mut move_number = 0;
        while s.remaining() > 0 {
            if let Some(themove) = s.human_solver.next(&s) {
                let a_move = themove.apply(&mut s);

                assert_eq!(
                    a_move, moves[move_number],
                    "Original: {:?}\n\nSecond: {:?}",
                    moves[move_number], a_move
                );
                move_number += 1;
            } else {
                panic!("Unsolvable Puzzle");
            }
        }

        assert_eq!(s.remaining(), 0);
        assert_eq!(s.moves.len(), moves.len());
        assert_eq!(s.grid, grid);
        assert_eq!(s.moves, moves);
    }
    #[test]
    fn possibilities() {
        let s = test_sud();
        // Row 4, Col 2 is Index 38
        let poss = s.possibilities(Index::new(38, SIZE).to_usize()).unwrap();
        assert_eq!(poss.len(), 4, "Possiblities: {:?}", poss);
        assert!(poss.contains(&2));
        assert!(poss.contains(&5));
        assert!(poss.contains(&6));
        assert!(poss.contains(&9));

        let poss = s.possibilities(Coord::new(4, 2, SIZE).to_usize()).unwrap();
        assert_eq!(poss.len(), 4);
        assert!(poss.contains(&2));
        assert!(poss.contains(&5));
        assert!(poss.contains(&6));
        assert!(poss.contains(&9));
    }

    #[test]
    fn is_valid_entry() -> Res<()> {
        let s = basic_sud();
        // Index 41 is Row 4, Col 4
        assert!(s.is_valid_entry(41, 1)?);
        assert!(s.is_valid_entry(41, 5)?);
        assert!(s.is_valid_entry(41, 8)?);
        assert!(!s.is_valid_entry(41, 2)?);
        assert!(!s.is_valid_entry(41, 6)?);
        assert!(!s.is_valid_entry(41, 9)?);

        assert!(s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 1)?);
        assert!(s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 5)?);
        assert!(s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 8)?);
        assert!(!s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 2)?);
        assert!(!s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 6)?);
        assert!(!s.is_valid_entry(Coord::new(4, 5, SIZE).to_usize(), 9)?);

        //test_oos!(s, is_valid_entry, 4u8);
        //test_ok!(s, is_valid_entry, 4u8);

        //test_values_oos!(s, is_valid_entry);
        //test_values_ok!(s, is_valid_entry);
        Ok(())
    }

    #[test]
    fn basic_solve() -> Result<(), SudError> {
        let s = basic_sud();

        let valid_sol = s.unique_solution();
        assert!(valid_sol.is_ok(), "{:?}", valid_sol);

        let brute = s.unique_solution()?;

        assert_ne!(&s.grid, brute);
        assert!(s.remaining() > 0);

        assert!(brute.iter().all(|v| v != &0));
        Ok(())
    }
}
