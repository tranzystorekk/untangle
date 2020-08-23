use crate::color::Color;
use crate::grid::Grid;
use crate::solution::Solution;

use ndarray::{Array1, ArrayView1};

fn check_monocolor(ribbon: &ArrayView1<Color>) -> Option<Color> {
    ribbon
        .iter()
        .try_fold(None, |state, &c| match c {
            Color::Blank => Some(state),
            valid_color => match state {
                None => Some(Some(valid_color)),
                st => st
                    .filter(|&current_color| current_color == valid_color)
                    .map(Some),
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

pub type Move = (usize, Color);
type Revert = (usize, Array1<Color>);

pub struct Solver {
    unused: Vec<bool>,
    reverts: Vec<Revert>,
    solved: Grid,
}

impl Solver {
    pub fn solve(grid: Grid) -> Vec<Solution> {
        let solver = Self::new(grid);
        solver.solve_internal()
    }

    fn new(grid: Grid) -> Self {
        Self {
            unused: vec![true; grid.n_ribbons()],
            reverts: Vec::new(),
            solved: grid,
        }
    }

    fn prepare_reversion(&mut self, sequence: &Vec<Move>) {
        for &(index, _) in sequence {
            self.unused[index] = false;

            let mut reverted = self.solved.ribbon_mut(index);
            self.reverts.push((index, reverted.to_owned()));

            reverted.fill(Color::Blank);
        }
    }

    fn revert(&mut self) {
        while let Some((index, colors)) = self.reverts.pop() {
            self.unused[index] = true;
            self.solved.ribbon_mut(index).assign(&colors);
        }
    }

    fn solve_internal(mut self) -> Vec<Solution> {
        let mut searchspace = init_searchspace(&self.solved);
        let mut solutions = Vec::new();

        while let Some(mut sequence) = searchspace.pop() {
            self.prepare_reversion(&sequence);

            if self.solved.is_uncolored() {
                sequence.reverse();
                let solution = Solution::new(sequence, &self.unused);
                solutions.push(solution);
            } else {
                for (i, ref ribbon) in self.solved.ribbons().enumerate() {
                    if let Some(color) = check_monocolor(ribbon) {
                        let mut next = sequence.clone();
                        next.push((i, color));

                        searchspace.push(next);
                    }
                }
            }

            self.revert();
        }

        solutions
    }
}

#[cfg(test)]
mod tests {
    use ndarray::ArrayView1;

    use super::check_monocolor;
    use crate::color::Color::*;

    #[test]
    fn check_monocolor_identifies_correct_ribbons() {
        let r = &ArrayView1::from(&[Red, Blank, Red, Red, Red]);

        assert_eq!(check_monocolor(r), Some(Red));
    }

    #[test]
    fn check_monocolor_fails_on_multicolor_ribbons() {
        let r = &ArrayView1::from(&[Red, Green, Blank, Red]);

        assert_eq!(check_monocolor(r), None);
    }

    #[test]
    fn check_monocolor_fails_on_empty_ribbons() {
        let r = &ArrayView1::from(&[Blank, Blank, Blank]);

        assert_eq!(check_monocolor(r), None);
    }
}
