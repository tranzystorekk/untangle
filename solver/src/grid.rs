use crate::color::Color;

use ndarray::{Array, Array2, ArrayView1};

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
        let n_rows = self.colors.nrows();
        if index >= n_rows {
            let column_index = index - n_rows;
            return self.colors.column(column_index);
        }

        self.colors.row(index)
    }

    pub fn ribbons(&self) -> impl Iterator<Item = ArrayView1<Color>> {
        self.colors
            .genrows()
            .into_iter()
            .chain(self.colors.gencolumns())
    }
}
