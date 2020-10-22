use crate::types::Move;

use std::fmt;

use itertools::Itertools;

pub struct Solution {
    moves: Vec<Move>,
    wildcards: Vec<usize>,
}

impl Solution {
    pub fn new(moves: Vec<Move>, unused: &[bool]) -> Self {
        Self {
            moves,
            wildcards: unused.iter().positions(|&not_used| not_used).collect(),
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
