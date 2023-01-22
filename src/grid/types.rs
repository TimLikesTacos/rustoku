use crate::grid::GridCoord;
use crate::sudokusize::SudokuSize;
use std::hash::Hash;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Row {
    pub(crate) row: usize,
    pub(crate) inner_index: usize,
    pub(crate) size: SudokuSize,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Col {
    pub(crate) col: usize,
    pub(crate) inner_index: usize,
    pub(crate) size: SudokuSize,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct GridBox {
    pub(crate) box_num: usize,
    pub(crate) inner_index: usize,
    pub(crate) size: SudokuSize,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
/// A struct to act as an interface for the public api and this library.
/// Important trait impletmentation is `From<usize>` and `From<(usize, usize)>`. This allows
/// converting an `Index` to and from a 1-D representation or a Row / Col tuple representation.
pub struct Index {
    pub(crate) inner_index: usize,
    pub(crate) size: SudokuSize,
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Coord {
    pub(crate) row: usize,
    pub(crate) col: usize,
    size: SudokuSize,
}

impl Row {
    #[inline]
    pub(crate) fn new(row: usize, inner_index: usize, size: SudokuSize) -> Self {
        Self {
            row,
            inner_index,
            size,
        }
    }

    #[inline]
    pub(crate) fn size(self) -> SudokuSize {
        self.size
    }
}

impl Col {
    #[inline]
    pub(crate) fn new(col: usize, inner_index: usize, size: SudokuSize) -> Self {
        Self {
            col,
            inner_index,
            size,
        }
    }

    #[inline]
    pub(crate) fn size(self) -> SudokuSize {
        self.size
    }
}

impl GridBox {
    #[inline]
    pub(crate) fn new(box_num: usize, inner_index: usize, size: SudokuSize) -> Self {
        Self {
            box_num,
            inner_index,
            size,
        }
    }

    #[inline]
    pub(crate) fn size(self) -> SudokuSize {
        self.size
    }
}

impl Index {
    #[inline(always)]
    pub(crate) fn new(inner_index: usize, size: SudokuSize) -> Self {
        Index { inner_index, size }
    }

    #[inline(always)]
    pub(crate) fn size(self) -> SudokuSize {
        self.size
    }

    #[inline(always)]
    pub(crate) fn to_usize(self) -> usize {
        self.inner_index
    }
}

impl Coord {
    #[inline]
    pub(crate) fn new(row: usize, col: usize, size: SudokuSize) -> Self {
        Coord { row, col, size }
    }

    #[inline]
    pub(crate) fn size(self) -> SudokuSize {
        self.size
    }

    #[inline]
    pub(crate) fn to_usize(&self) -> usize {
        self.row * self.size.house_size() + self.col
    }
}

impl From<Index> for usize {
    fn from(value: Index) -> Self {
        value.inner_index
    }
}

impl From<Index> for (usize, usize) {
    fn from(value: Index) -> Self {
        let coord = value.coord();
        (coord.row, coord.col)
    }
}

#[cfg(test)]
mod index_type_test {
    use super::*;
    use crate::grid::GridCoord;

    const SIZE: SudokuSize = SudokuSize::Three;

    #[test]
    fn index() {
        let ind = Index::new(0, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_some());

        let ind = Index::new(80, SudokuSize::Three);

        let ind1 = ind.inc();
        assert!(ind1.is_none());

        let coord = ind.coord();
        assert_eq!(coord, Coord::new(8, 8, SIZE));
        assert_eq!(ind.row(), Row::new(8, 8, SIZE));
        assert_eq!(ind.col(), Col::new(8, 8, SIZE));
        assert_eq!(ind.to_box(), GridBox::new(8, 8, SIZE));
    }

    #[test]
    fn coord() {
        let ind = Coord::new(0, 0, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_some());

        let ind = Coord::new(8, 8, SudokuSize::Three);

        let ind1 = ind.inc();
        assert!(ind1.is_none());

        assert_eq!(ind.index(), Index::new(80, SIZE));
        assert_eq!(ind.row(), Row::new(8, 8, SIZE));
        assert_eq!(ind.col(), Col::new(8, 8, SIZE));
        assert_eq!(ind.to_box(), GridBox::new(8, 8, SIZE));
    }

    #[test]
    fn row() {
        let ind = Row::new(0, 0, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_some());

        let ind = Row::new(8, 8, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_none());

        assert_eq!(ind.index(), Index::new(80, SIZE));
        assert_eq!(ind.coord(), Coord::new(8, 8, SIZE));
        assert_eq!(ind.col(), Col::new(8, 8, SIZE));
        assert_eq!(ind.to_box(), GridBox::new(8, 8, SIZE));
    }

    #[test]
    fn col() {
        let ind = Col::new(0, 0, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_some());

        let ind = Col::new(8, 8, SudokuSize::Three);

        let ind1 = ind.inc();
        assert!(ind1.is_none());

        assert_eq!(ind.index(), Index::new(80, SIZE));
        assert_eq!(ind.row(), Row::new(8, 8, SIZE));
        assert_eq!(ind.coord(), Coord::new(8, 8, SIZE));
        assert_eq!(ind.to_box(), GridBox::new(8, 8, SIZE));
    }

    #[test]
    fn gridbox() {
        let ind = GridBox::new(0, 0, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_some());

        let ind = GridBox::new(8, 8, SudokuSize::Three);
        let ind1 = ind.inc();
        assert!(ind1.is_none());

        assert_eq!(ind.index(), Index::new(80, SIZE));
        assert_eq!(ind.row(), Row::new(8, 8, SIZE));
        assert_eq!(ind.col(), Col::new(8, 8, SIZE));
        assert_eq!(ind.coord(), Coord::new(8, 8, SIZE));
    }
}
