use crate::color::Color;

use ndarray::{Array, Array2, ArrayView1, ArrayViewMut1};

enum RibbonIndex {
    Row(usize),
    Column(usize),
}

pub struct Grid {
    colors: Array2<Color>,
}

impl Grid {
    pub fn from_vec_dims(v: Vec<Color>, rows: usize, cols: usize) -> Self {
        Grid {
            colors: Array::from_shape_vec((rows, cols), v).unwrap(),
        }
    }

    pub fn ribbon(&self, index: usize) -> ArrayView1<Color> {
        match self.ribbon_index(index) {
            RibbonIndex::Row(ind) => self.colors.row(ind),
            RibbonIndex::Column(ind) => self.colors.column(ind),
        }
    }

    pub fn ribbon_mut(&mut self, index: usize) -> ArrayViewMut1<Color> {
        match self.ribbon_index(index) {
            RibbonIndex::Row(ind) => self.colors.row_mut(ind),
            RibbonIndex::Column(ind) => self.colors.column_mut(ind),
        }
    }

    pub fn ribbons(&self) -> impl Iterator<Item = ArrayView1<Color>> {
        self.colors
            .genrows()
            .into_iter()
            .chain(self.colors.gencolumns())
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
