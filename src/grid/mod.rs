// Re-exports
pub(crate) use crate::grid::gridcoord::GridCoord;
pub(crate) use crate::grid::types::{Col, Coord, GridBox, Index, Row};
pub(crate) use crate::sudokusize::SudokuSize;

//pub mod box_iter;
pub mod grid;
pub(crate) mod grid_iterators;
pub(crate) mod grid_traits;
pub mod gridcoord;
pub(crate) mod house;
pub(crate) mod types;

pub use grid::Grid;
