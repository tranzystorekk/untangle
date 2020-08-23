use std::fmt;

use itertools::Itertools;

use crate::solve::Move;

pub struct Solution {
    moves: Vec<Move>,
    wildcards: Vec<usize>,
}

impl Solution {
    pub fn new(moves: Vec<Move>, unused: &Vec<bool>) -> Self {
        Self {
            moves,
            wildcards: unused
                .iter()
                .enumerate()
                .filter(|&(_, &value)| value)
                .map(|(i, _)| i)
                .collect(),
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.wildcards.is_empty() {
            let wild_print = self.wildcards.iter().join(" ");
            writeln!(f, "{} => *", wild_print)?;
        }

        for &(index, color) in &self.moves {
            writeln!(f, "{} => {}", index, color)?;
        }

        Ok(())
    }
}
