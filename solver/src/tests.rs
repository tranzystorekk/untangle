use crate::Color::*;
use crate::Grid;

use ndarray::ArrayView;

#[test]
fn ribbons_iterates_over_rows_and_columns() {
    let fields = vec![Orange, Red, Blank, Red, Green, Purple];
    let grid = Grid::from_vec_dims(fields, 2, 3);
    let mut ribbons = grid.ribbons();

    assert_eq!(ribbons.next(), Some(ArrayView::from(&[Orange, Red, Blank])));
    assert_eq!(ribbons.next(), Some(ArrayView::from(&[Red, Green, Purple])));
    assert_eq!(ribbons.next(), Some(ArrayView::from(&[Orange, Red])));
    assert_eq!(ribbons.next(), Some(ArrayView::from(&[Red, Green])));
    assert_eq!(ribbons.next(), Some(ArrayView::from(&[Blank, Purple])));
    assert_eq!(ribbons.next(), None);
}
