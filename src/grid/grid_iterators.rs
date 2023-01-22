use crate::grid::grid::Grid;
use crate::grid::gridcoord::HouseCoord;

pub(crate) struct GridIter<'a, T, C>
where
    C: HouseCoord,
{
    pub(crate) grid: &'a Grid<T>,
    pub(crate) location: Option<C>,
}

pub(crate) struct MutGridIter<'a, T, C>
where
    C: HouseCoord,
{
    pub(crate) grid: &'a mut Grid<T>,
    pub(crate) location: Option<C>,
}

impl<'a, T, C: HouseCoord> Iterator for GridIter<'a, T, C> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if let Some(current_pos) = self.location {
            self.location = current_pos.inc();
            Some(&self.grid.items[current_pos.to_usize()])
        } else {
            None
        }
    }
}

impl<'a, T, C: HouseCoord> Iterator for MutGridIter<'a, T, C> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        if let Some(current_pos) = self.location {
            self.location = current_pos.inc();
            let ptr: *mut T = &mut self.grid.items[current_pos.to_usize()];

            // self.grid.items should not reallocate when using this iterator and therefore safe
            unsafe { Some(&mut *ptr) }
        } else {
            None
        }
    }
}
