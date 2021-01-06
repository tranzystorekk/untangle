use crate::color::Color;
use crate::grid::Grid;
use crate::solution::Solution;
use crate::types::{Move, Ribbon};

use itertools::Itertools;

fn check_monocolor(ribbon: Ribbon) -> Option<Color> {
    ribbon
        .into_iter()
        .copied()
        .filter(Color::non_blank)
        .dedup()
        .exactly_one()
        .ok()
}

fn init_searchspace(grid: &Grid) -> Vec<Vec<Move>> {
    grid.ribbons()
        .enumerate()
        .filter_map(|(i, ribbon)| check_monocolor(ribbon).map(|color| vec![(i, color)]))
        .collect()
}

struct Solver {
    unused: Vec<bool>,
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
            solved: grid,
        }
    }

    fn execute(&mut self, sequence: &[Move]) {
        for &(index, _) in sequence {
            self.unused[index] = false;
            self.solved.ribbon_mut(index).fill(Color::Blank);
        }
    }

    fn solve_internal(mut self) -> Vec<Solution> {
        let mut searchspace = init_searchspace(&self.solved);
        let mut solutions = Vec::new();
        let initial = self.solved.clone();

        while let Some(mut sequence) = searchspace.pop() {
            self.execute(&sequence);

            if self.solved.is_uncolored() {
                sequence.reverse();
                let solution = Solution::new(sequence, &self.unused);
                solutions.push(solution);
            } else {
                for (i, ribbon) in self.solved.ribbons().enumerate() {
                    if let Some(color) = check_monocolor(ribbon) {
                        let mut next = sequence.clone();
                        next.push((i, color));

                        searchspace.push(next);
                    }
                }
            }

            self.solved.clone_from(&initial);
            self.unused.iter_mut().for_each(|el| *el = true);
        }

        solutions
    }
}

pub fn solve(grid: Grid) -> Vec<Solution> {
    Solver::solve(grid)
}

#[cfg(test)]
mod tests {
    use crate::types::Ribbon;

    use super::check_monocolor;
    use crate::color::Color::*;

    #[test]
    fn check_monocolor_identifies_correct_ribbons() {
        let r = Ribbon::from(&[Red, Blank, Red, Red, Red]);

        assert_eq!(check_monocolor(r), Some(Red));
    }

    #[test]
    fn check_monocolor_fails_on_multicolor_ribbons() {
        let r = Ribbon::from(&[Red, Green, Blank, Red]);

        assert_eq!(check_monocolor(r), None);
    }

    #[test]
    fn check_monocolor_fails_on_empty_ribbons() {
        let r = Ribbon::from(&[Blank, Blank, Blank]);

        assert_eq!(check_monocolor(r), None);
    }
}
