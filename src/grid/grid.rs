use crate::bitset::*;
use crate::errors::SudError;
use crate::grid::grid_iterators::{GridIter, MutGridIter};
use crate::grid::grid_traits::Rotate;
use crate::grid::gridcoord::*;
use crate::grid::house::House;
use crate::grid::types::*;
use crate::square::Square;
use crate::sudokusize::*;
use std::fmt::Debug;
use std::ops::IndexMut;

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<S> {
    pub(crate) items: Vec<S>,
    pub(crate) size: SudokuSize,
}

impl<S: PartialEq + Clone> Grid<S> {
    #[inline]
    pub(crate) fn new(items: Vec<S>, size: SudokuSize) -> Grid<S> {
        Grid { items, size }
    }

    #[inline]
    pub(crate) fn box_size(&self) -> usize {
        self.size.size()
    }

    #[inline]
    pub(crate) fn house_size(&self) -> usize {
        self.size.house_size()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn num_cells(&self) -> usize {
        self.size.total()
    }

    #[inline]
    pub fn get<C: InputCoord>(&self, index: C) -> Result<&S, SudError> {
        let index = index.to_usize(&self.size)?;
        Ok(self._get(index))
    }

    #[inline]
    pub(crate) fn _get(&self, index: usize) -> &S {
        &self.items[index]
    }

    /// Returns row, col, box number of `'usize` index
    pub fn calc_house(&self, index: usize) -> (usize, usize, usize) {
        let size = self.size;
        let width = size.size();
        let row = self.calc_row(index);
        let col = self.calc_col(index);
        let box_num = (row / width) * width + (col / width);
        (row, col, box_num)
    }

    #[inline]
    /// Returns row of `usize` index
    pub fn calc_row(&self, index: usize) -> usize {
        index / self.size.house_size()
    }

    #[inline]
    /// Returns col number of `usize` index
    pub fn calc_col(&self, index: usize) -> usize {
        index % self.size.house_size()
    }

    #[inline]
    /// Returns box number of `usize` index
    pub fn calc_box(&self, index: usize) -> usize {
        self.calc_house(index).2
    }

    #[inline]
    pub(crate) fn house_iter<H: HouseCoord>(&self, index: H) -> impl Iterator<Item = &S> {
        GridIter {
            grid: self,
            location: Some(index.beginning()),
        }
    }

    #[inline]
    /// Returns an iterator that iterates over each square's value in a row dominate fashion.
    pub(crate) fn value_iter<'b, 'a: 'b>(&'a self) -> impl Iterator<Item = &S> + 'b {
        self.items.iter()
    }

    #[inline]
    pub(crate) fn sudoku_size(&self) -> SudokuSize {
        self.size
    }

    #[inline]
    /// The total number of squares.  For a 9X9, this is 81
    pub(crate) fn num_squares(&self) -> usize {
        self.size.total()
    }

    // pub fn value_equal<T: BitSetInt>(left: &Square<T>, right: &Square<T>) -> bool {
    //     left.value == right.value
    // }

    /// Determines if the value that is currently set is valid.  Used after the value is set.  Can
    /// be used to determine if the initial puzzle is valid.
    pub(crate) fn was_valid_entry(&self, index: usize) -> bool {
        if let Some(v) = self.items.get(index) {
            self.single_iterator(index).filter(|&x| *x == *v).count() == 3
        } else {
            true
        }
    }

    // let (row, col, box_num) = self.calc_house(index);
    // self.house_iter(Row::start(row, self.size)).chain(
    // self.house_iter(Col::start(col, self.size))
    // .chain(self.house_iter(GridBox::start(box_num, self.size))),
    // )

    #[inline]
    pub(crate) fn enum_house_iter<'b, 'a: 'b>(
        &'a self,
        house: &House<usize>,
        size: SudokuSize,
    ) -> Box<dyn Iterator<Item = &'a S> + 'b> {
        match house {
            House::Row(n) => Box::new(self.house_iter(Row::new(*n, 0, size))),
            House::Col(n) => Box::new(self.house_iter(Col::new(*n, 0, size))),
            House::Box(n) => Box::new(self.house_iter(GridBox::new(*n, 0, size))),
        }
    }

    #[inline]
    pub(crate) fn house_iter_mut<H: HouseCoord>(
        &mut self,
        index: H,
    ) -> impl Iterator<Item = &mut S> {
        MutGridIter {
            grid: self,
            location: Some(index.beginning()),
        }
    }

    pub(crate) fn house_iters<'a, 'b: 'a, H: 'a + HouseCoord>(
        &'b self,
    ) -> Vec<Box<impl Iterator<Item = &'b S> + 'a>> {
        let mut iters = Vec::with_capacity(self.size.house_size());
        for i in 0..self.size.house_size() {
            iters.push(Box::new(self.house_iter(H::start(i, self.size))))
        }
        iters
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &S> {
        self.items.iter()
    }

    /// An iterator that iterates over the row, column, and box that the cell with the parameter `index`.
    /// NOTE: The cell of parameter `index` is iterated over 3 times (once for each iterator).
    /// There is no mutable version of this as it would require 3 mutable borrows at the same time.
    /// todo: create a struct with a slice of the cells and create a mutable iterator to get around borrow issues.
    #[inline]
    pub(crate) fn single_iterator(&self, index: usize) -> impl Iterator<Item = &S> {
        //let coord = index.coord();
        let (row, col, box_num) = self.calc_house(index);
        self.house_iter(Row::start(row, self.size)).chain(
            self.house_iter(Col::start(col, self.size))
                .chain(self.house_iter(GridBox::start(box_num, self.size))),
        )
    }
}

impl<V: BitSetInt> Grid<Square<V>> {
    pub(crate) fn to_inner(&self) -> Grid<V> {
        let newgrid = self
            .items
            .iter()
            .map(|sq| sq.val().unwrap_or_else(|| Bit::zero()).into())
            .collect();
        Grid {
            items: newgrid,
            size: self.size,
        }
    }
}

impl<S: PartialEq + Send + Sync> std::ops::Index<usize> for Grid<S> {
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<S: PartialEq + Send + Sync> IndexMut<usize> for Grid<S> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl<V> FromIterator<V> for Grid<V> {
    fn from_iter<T: IntoIterator<Item = V>>(iter: T) -> Self {
        let items: Vec<V> = iter.into_iter().collect();
        // Unwrap is safe here as this should only be called from an object that was already length checked.
        let size = SudokuSize::size_from_input_length(items.len()).unwrap();
        Grid { items, size }
    }
}

impl<S: Clone + PartialEq + Send + Sync + 'static> Rotate for &Grid<S> {
    type PuzzleInp = Grid<S>;

    fn cwrotate(self, quarter_rotations: usize) -> Res<Self::PuzzleInp> {
        let max = self.size.house_size();
        let mut ret: Vec<_> = self.iter().cloned().collect();
        for _ in 0..quarter_rotations {
            for x in 0..max / 2 {
                for y in x..max - 1 - x {
                    let ul = Coord::new(x, y, self.size).to_usize();
                    let ur = Coord::new(y, max - 1 - x, self.size).to_usize();
                    let ll = Coord::new(max - 1 - y, x, self.size).to_usize();
                    let lr = Coord::new(max - 1 - x, max - 1 - y, self.size).to_usize();
                    ret.swap(ul, ll);
                    ret.swap(ll, lr);
                    ret.swap(lr, ur);
                }
            }
        }
        Ok(Grid::new(ret, self.size))
    }
}

#[cfg(test)]
mod grid_tests {
    use crate::sudokusize::SudokuSize;

    use super::*;

    // a vec with all cells of unique values 1..=size.total()
    fn unique_test_vec(size: SudokuSize) -> Vec<i32> {
        let mut v = Vec::with_capacity(size.total());
        for n in 0..size.total() {
            v.push(n as i32);
        }
        assert_eq!(v.len(), size.total());
        v
    }

    // a vec with cells 1..=size.housesize() in rows
    fn row_house_test_vec(size: SudokuSize) -> Vec<i32> {
        let mut v = Vec::with_capacity(size.total());
        for n in 0..size.total() {
            v.push((n % size.house_size()) as i32);
        }
        assert_eq!(v.len(), size.total());
        v
    }

    // Returns a tuple of grids using the two functions above.
    fn test_grid(size: SudokuSize) -> (Grid<i32>, Grid<i32>) {
        (
            Grid {
                items: unique_test_vec(size),
                size,
            },
            Grid {
                items: row_house_test_vec(size),
                size,
            },
        )
    }

    const SIZE: SudokuSize = SudokuSize::Three;

    fn ncoord(one: usize, two: usize) -> Coord {
        Coord::new(one, two, SIZE)
    }

    #[test]
    fn new_test() {
        let grid = test_grid(SudokuSize::Three).0;
        let new_grid = Grid::new(grid.items.clone(), grid.size);
        assert_eq!(grid, new_grid);

        let grid = test_grid(SudokuSize::Three).1;
        let new_grid = Grid::new(grid.items.clone(), grid.size);
        assert_eq!(grid, new_grid);
    }

    #[test]
    fn row_iter() -> Res<()> {
        let grid = test_grid(SudokuSize::Three).0;
        let mut iter = grid.house_iter(ncoord(0, 0).row());
        let mut not_begininning = grid.house_iter(ncoord(0, 5).row());
        let mut mid_iter = grid.house_iter(ncoord(7, 0).row());
        for expected in 0..9 {
            assert_eq!(*(iter.next().unwrap()), expected);
            assert_eq!(*(not_begininning.next().unwrap()), expected);
            assert_eq!(
                *(mid_iter.next().unwrap()),
                expected + 7 * SIZE.house_size() as i32
            );
        }
        assert!(iter.next().is_none());
        assert!(mid_iter.next().is_none());

        let mut iter = grid.house_iter(ncoord(8, 0).row());
        let mut mid_iter = grid.house_iter(ncoord(8, 8).row());
        for expected in 72..81 {
            assert_eq!(*(iter.next().unwrap()), expected);
            assert_eq!(*(mid_iter.next().unwrap()), expected);
        }
        assert!(iter.next().is_none());
        assert!(mid_iter.next().is_none());

        let grid = test_grid(SudokuSize::Eight).0;
        let mut iter = grid.house_iter(Coord::new(0, 0, SudokuSize::Eight).row());
        for expected in 0..64 {
            assert_eq!(*(iter.next().unwrap()), expected);
        }
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn col_iter() -> Res<()> {
        let grid = test_grid(SudokuSize::Three).0;
        let mut iter = grid.house_iter(ncoord(0, 0).col());
        let mut mid_iter = grid.house_iter(ncoord(7, 0).col());
        for expected in 0..9 {
            assert_eq!(*(iter.next().unwrap()), expected * 9);
            assert_eq!(*(mid_iter.next().unwrap()), expected * 9);
        }
        assert!(iter.next().is_none());
        assert!(mid_iter.next().is_none());

        let mut iter = grid.house_iter(ncoord(0, 8).col());
        let mut mid_iter = grid.house_iter(ncoord(8, 8).col());
        for expected in 0..9 {
            assert_eq!(*(iter.next().unwrap()), expected * 9 + 8);
            assert_eq!(*(mid_iter.next().unwrap()), expected * 9 + 8);
        }
        assert!(iter.next().is_none());
        assert!(mid_iter.next().is_none());

        let grid = test_grid(SudokuSize::Eight).0;
        let mut iter = grid.house_iter(Coord::new(0, 0, SudokuSize::Eight).col());
        for expected in 0..64 {
            assert_eq!(*(iter.next().unwrap()), expected * 64);
        }
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn box_iter() -> Res<()> {
        let grid = test_grid(SudokuSize::Three).1;
        let mut iter = grid.house_iter(ncoord(0, 0).to_box());
        let mut mid_iter = grid.house_iter(GridBox::new(2, 1, SIZE));
        for expected in 0..9 {
            let v = iter.next();
            assert_eq!(*(v.unwrap()), expected % 3);
            assert_eq!(*(mid_iter.next().unwrap()), expected % 3 + 6);
        }
        assert!(iter.next().is_none());
        //assert!(mid_iter.next().is_none());

        let mut iter = grid.house_iter(ncoord(0, 4).to_box());
        let mut mid_iter = grid.house_iter(ncoord(2, 5).to_box());
        for expected in 0..9 {
            assert_eq!(*(iter.next().unwrap()), expected % 3 + 3);
            assert_eq!(*(mid_iter.next().unwrap()), expected % 3 + 3);
        }
        assert!(iter.next().is_none());
        assert!(mid_iter.next().is_none());

        let grid = test_grid(SudokuSize::Eight).1;
        let mut iter = grid.house_iter(Coord::new(0, 0, SudokuSize::Eight).to_box());
        for expected in 0..64 {
            assert_eq!(*(iter.next().unwrap()), expected % 8);
        }
        assert!(iter.next().is_none());
        Ok(())
    }

    #[test]
    fn single_unique_items_iter() {
        let grid = test_grid(SIZE).0;
        for i in 0..SIZE.total() {
            let val = grid.items.get(i).unwrap();
            let count = grid.single_iterator(i).filter(|v| *v == val).count();
            assert_eq!(count, 3);
        }
    }

    #[test]
    fn row_house_single_iter() {
        let grid = test_grid(SIZE).1;
        for i in 0..SIZE.total() {
            let val = grid.items.get(i).unwrap();
            let count = grid.single_iterator(i).filter(|v| *v == val).count();
            assert_eq!(count, 13);
        }
    }
}
