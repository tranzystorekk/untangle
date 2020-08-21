use crate::color::Color;
use crate::grid::Grid;

use ndarray::ArrayView1;

type Move = (usize, Color);

fn check_monocolor(ribbon: &ArrayView1<Color>) -> Option<Color> {
    ribbon
        .iter()
        .try_fold(None, |state, &c| match c {
            Color::Blank => Some(state),
            valid_color => match state {
                None => Some(Some(valid_color)),
                st => st
                    .filter(|&current_color| current_color == valid_color)
                    .map(|color| Some(color)),
            },
        })
        .flatten()
}

fn init_searchspace(grid: &Grid) -> Vec<Vec<Move>> {
    grid.ribbons()
        .enumerate()
        .filter_map(|(i, ref ribbon)| check_monocolor(ribbon).map(|color| vec![(i, color)]))
        .collect()
}

pub fn solve(mut grid: Grid) {
    let mut searchspace = init_searchspace(&grid);
    let mut unused_ribbons = vec![true; grid.n_ribbons()];

    while !searchspace.is_empty() {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use ndarray::ArrayView1;

    use super::check_monocolor;
    use crate::color::Color::*;

    #[test]
    fn check_monocolor_identifies_correct_ribbons() {
        let ref r = ArrayView1::from(&[Red, Blank, Red, Red, Red]);

        assert_eq!(check_monocolor(r), Some(Red));
    }

    #[test]
    fn check_monocolor_fails_on_multicolor_ribbons() {
        let ref r = ArrayView1::from(&[Red, Green, Blank, Red]);

        assert_eq!(check_monocolor(r), None);
    }

    #[test]
    fn check_monocolor_fails_on_empty_ribbons() {
        let ref r = ArrayView1::from(&[Blank, Blank, Blank]);

        assert_eq!(check_monocolor(r), None);
    }
}
