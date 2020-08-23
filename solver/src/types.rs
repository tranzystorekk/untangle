use crate::color::Color;

use ndarray::Array1;

pub type Move = (usize, Color);
pub type Revert = (usize, Array1<Color>);
