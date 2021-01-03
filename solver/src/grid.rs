use crate::color::Color;
use crate::types::{Ribbon, RibbonMut};

use ndarray::{Array, Array2};

enum RibbonIndex {
    Row(usize),
    Column(usize),
}

#[derive(Clone)]
pub struct Grid {
    colors: Array2<Color>,
}

impl Grid {
    pub fn from_vec_dims(v: Vec<Color>, rows: usize, cols: usize) -> Self {
        Grid {
            colors: Array::from_shape_vec((rows, cols), v).unwrap(),
        }
    }

    pub fn ribbon(&self, index: usize) -> Ribbon {
        match self.ribbon_index(index) {
            RibbonIndex::Row(ind) => self.colors.row(ind),
            RibbonIndex::Column(ind) => self.colors.column(ind),
        }
    }

    pub fn ribbon_mut(&mut self, index: usize) -> RibbonMut {
        match self.ribbon_index(index) {
            RibbonIndex::Row(ind) => self.colors.row_mut(ind),
            RibbonIndex::Column(ind) => self.colors.column_mut(ind),
        }
    }

    pub fn ribbons(&self) -> impl Iterator<Item = Ribbon> {
        let rows = self.colors.genrows();
        let cols = self.colors.gencolumns();

        itertools::chain(rows, cols)
    }

    pub fn n_ribbons(&self) -> usize {
        self.colors.nrows() + self.colors.ncols()
    }

    pub fn is_uncolored(&self) -> bool {
        self.colors.iter().all(|&c| matches!(c, Color::Blank))
    }

    fn ribbon_index(&self, index: usize) -> RibbonIndex {
        let n_rows = self.colors.nrows();
        if index >= n_rows {
            let column_index = index - n_rows;
            return RibbonIndex::Column(column_index);
        }

        RibbonIndex::Row(index)
    }
}
