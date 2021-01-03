use crate::color::Color;

use ndarray::{ArrayView1, ArrayViewMut1};

pub type Move = (usize, Color);

pub type Ribbon<'a> = ArrayView1<'a, Color>;
pub type RibbonMut<'a> = ArrayViewMut1<'a, Color>;
