use crate::errors::SudError;
use crate::grid::grid::Grid;
use crate::grid::types::*;
use crate::sudokusize::SudokuSize;
use std::fmt::Debug;
use std::hash::Hash;

pub trait GridCoord: Copy + Default + Debug + Hash + Eq {
    /// Converts input of an index number to be used by the Grid struct. Returns Err if outside
    /// parameters of puzzle
    fn index(self) -> Index;

    /// Coverts output of index number to row and column tuple
    fn coord(self) -> Coord;

    fn row(self) -> Row;

    fn col(self) -> Col;

    // Convert to GridBox
    fn to_box(self) -> GridBox;

    fn inc(self) -> Option<Self>;

    fn beginning(self) -> Self;

    fn equal<C: GridCoord>(&self, other: &C) -> bool {
        self.coord() == other.coord()
    }
}

pub trait HouseCoord: GridCoord {
    fn start(number: usize, size: SudokuSize) -> Self;
    fn restart(&self) -> Self;
    fn to_usize(&self) -> usize {
        self.index().inner_index
    }
}

impl HouseCoord for Row {
    fn start(number: usize, size: SudokuSize) -> Self {
        Row::new(number, 0, size)
    }
    fn restart(&self) -> Self {
        Row::new(self.row, 0, self.size)
    }
}
impl HouseCoord for Col {
    fn start(number: usize, size: SudokuSize) -> Self {
        Col::new(number, 0, size)
    }
    fn restart(&self) -> Self {
        Col::new(self.col, 0, self.size)
    }
}
impl HouseCoord for GridBox {
    fn start(number: usize, size: SudokuSize) -> Self {
        GridBox::new(number, 0, size)
    }
    fn restart(&self) -> Self {
        GridBox::new(self.box_num, 0, self.size)
    }
}

pub trait InputCoord: Debug {
    fn to_usize(self, size: &SudokuSize) -> Result<usize, SudError>;
}

impl InputCoord for usize {
    fn to_usize(self, size: &SudokuSize) -> Result<usize, SudError> {
        if self < size.total() {
            Ok(self)
        } else {
            Err(SudError::InvalidLocation((self, 0)))
        }
    }
}
impl InputCoord for (usize, usize) {
    fn to_usize(self, size: &SudokuSize) -> Result<usize, SudError> {
        let ind = Coord::new(self.0, self.1, *size).index().to_usize();
        if ind < size.total() {
            Ok(ind)
        } else {
            Err(SudError::InvalidLocation(self))
        }
    }
}

impl InputCoord for Index {
    fn to_usize(self, size: &SudokuSize) -> Result<usize, SudError> {
        if self.inner_index < size.total() {
            Ok(self.inner_index)
        } else {
            Err(SudError::InvalidLocation((self.inner_index, 0)))
        }
    }
}

impl<T, C: GridCoord> std::ops::Index<C> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: C) -> &Self::Output {
        &self.items[index.index().to_usize()]
    }
}

impl GridCoord for Index {
    #[inline(always)]
    fn index(self) -> Index {
        Index::new(self.inner_index, self.size())
    }

    #[inline]
    fn coord(self) -> Coord {
        let size = self.size();
        Coord::new(
            self.inner_index / size.house_size(),
            self.inner_index % size.house_size(),
            size,
        )
    }

    #[inline]
    fn row(self) -> Row {
        let size = self.size();
        Row::new(
            self.inner_index / size.house_size(),
            self.inner_index % size.house_size(),
            size,
        )
    }

    #[inline]
    fn col(self) -> Col {
        let size = self.size();
        Col::new(
            self.inner_index % size.house_size(),
            self.inner_index / size.house_size(),
            size,
        )
    }

    fn to_box(self) -> GridBox {
        let size = self.size();
        let coord = self.coord();
        let width = size.size();
        let box_num = (coord.row / width) * width + (coord.col / width);
        let offset = (coord.row % width) * width + coord.col % width;

        GridBox::new(box_num, offset, size)
    }

    #[inline]
    fn inc(self) -> Option<Self> {
        if self.inner_index + 1 >= self.size.house_size() {
            None
        } else {
            Some(Index::new(self.inner_index + 1, self.size()))
        }
    }

    fn beginning(self) -> Self {
        Index::new(0, self.size())
    }
}

impl GridCoord for Coord {
    #[inline]
    fn index(self) -> Index {
        let max = self.size().house_size();

        Index::new(self.row * max + self.col, self.size())
    }

    #[inline]
    fn coord(self) -> Coord {
        self
    }

    #[inline]
    fn row(self) -> Row {
        Row::new(self.row, self.col, self.size())
    }

    #[inline]
    fn col(self) -> Col {
        Col::new(self.col, self.row, self.size())
    }

    fn to_box(self) -> GridBox {
        let size = self.size();
        let width = size.size();
        let box_num = (self.row / width) * width + (self.col / width);
        let offset = (self.col % width) * width + self.row % width;

        GridBox::new(box_num, offset, size)
    }

    #[inline]
    fn inc(self) -> Option<Self> {
        let size = self.size();
        let carry = (self.col + 1) % size.house_size() == 0;
        if carry {
            if (self.row + 1) >= size.house_size() {
                None
            } else {
                Some(Coord::new(self.row + 1, 0, size))
            }
        } else {
            Some(Coord::new(self.row, self.col + 1, size))
        }
    }

    #[inline]
    fn beginning(self) -> Self {
        Coord::new(0, 0, self.size())
    }
}
impl GridCoord for GridBox {
    #[inline(always)]
    fn index(self) -> Index {
        self.coord().index()
    }

    #[inline]
    fn coord(self) -> Coord {
        let size = self.size();
        let width = size.size();
        let row = (self.box_num / width) * width + self.inner_index / width;
        let col = (self.box_num % width) * width + self.inner_index % width;
        Coord::new(row, col, size)
    }

    #[inline]
    fn row(self) -> Row {
        self.coord().row()
    }

    #[inline]
    fn col(self) -> Col {
        self.coord().col()
    }

    #[inline]
    fn to_box(self) -> GridBox {
        self
    }

    #[inline]
    fn inc(self) -> Option<Self> {
        if self.inner_index + 1 >= self.size.house_size() {
            None
        } else {
            Some(GridBox::new(
                self.box_num,
                self.inner_index + 1,
                self.size(),
            ))
        }
    }

    #[inline]
    fn beginning(self) -> Self {
        GridBox::new(self.box_num, 0, self.size())
    }
}

impl GridCoord for Row {
    #[inline]
    fn index(self) -> Index {
        self.coord().index()
    }

    #[inline]
    fn coord(self) -> Coord {
        Coord::new(self.row, self.inner_index, self.size())
    }

    #[inline]
    fn row(self) -> Row {
        self
    }

    #[inline]
    fn col(self) -> Col {
        Col::new(self.inner_index, self.row, self.size())
    }

    #[inline]
    fn to_box(self) -> GridBox {
        self.coord().to_box()
    }

    #[inline]
    fn inc(self) -> Option<Self> {
        if self.inner_index + 1 >= self.size.house_size() {
            None
        } else {
            Some(Row::new(self.row, self.inner_index + 1, self.size()))
        }
    }

    #[inline]
    fn beginning(self) -> Self {
        Row::new(self.row, 0, self.size())
    }
}

impl GridCoord for Col {
    #[inline]
    fn index(self) -> Index {
        self.coord().index()
    }

    #[inline]
    fn coord(self) -> Coord {
        Coord::new(self.inner_index, self.col, self.size())
    }

    #[inline]
    fn row(self) -> Row {
        Row::new(self.inner_index, self.col, self.size())
    }

    #[inline]
    fn col(self) -> Col {
        self
    }

    #[inline]
    fn to_box(self) -> GridBox {
        self.coord().to_box()
    }

    #[inline]
    fn inc(self) -> Option<Self> {
        if self.inner_index + 1 >= self.size.house_size() {
            None
        } else {
            Some(Col::new(self.col, self.inner_index + 1, self.size()))
        }
    }

    #[inline]
    fn beginning(self) -> Self {
        Col::new(self.col, 0, self.size())
    }
}

#[cfg(test)]
mod grid_coord_tests {

    use super::*;
    type Res<T> = Result<T, Box<dyn std::error::Error>>;
    #[test]
    fn index() -> Res<()> {
        let size = SudokuSize::Three;
        let index = Index::new(5, size);
        assert_eq!(index.index(), Index::new(5, size));
        assert_eq!(index.row(), Row::new(0, 5, size));
        assert_eq!(index.to_box(), GridBox::new(1, 2, size));
        assert_eq!(index.coord(), Coord::new(0, 5, size));
        assert_eq!(index.col(), Col::new(5, 0, size));

        Ok(())
    }

    #[test]
    fn tousize() -> Res<()> {
        let size = SudokuSize::Four;
        let row = Row::new(2, 2, size);
        assert_eq!(row.index().to_usize(), 34);

        let col = Col::new(1, 2, size);
        assert_eq!(col.index().to_usize(), 33);

        let abox = GridBox::new(2, 2, size);
        assert_eq!(abox.index().to_usize(), 10);

        Ok(())
    }
}
