mod opts;

use std::io::BufRead;

use itertools::Itertools;
use structopt::StructOpt;

use opts::Opts;
use solver::{Color, Grid, Solver};

fn process_input(input: impl BufRead) -> std::io::Result<(usize, usize, Vec<Color>)> {
    let mut lines = input.lines();
    let (rows, cols) = lines
        .next()
        .expect("something")?
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect_tuple()
        .expect("something2");

    let fields = lines
        .flat_map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse().expect("something3"))
                .collect::<Vec<Color>>()
        })
        .collect();

    Ok((rows, cols, fields))
}

fn main() -> std::io::Result<()> {
    let opts = Opts::from_args();
    let input = opts.input()?;

    let (rows, cols, fields) = process_input(input)?;

    if fields.is_empty() {
        panic!("Grid has no fields!");
    }

    if fields.len() != rows * cols {
        panic!("Incorrect grid (either dimensions or missing content)");
    }

    let grid = Grid::from_vec_dims(fields, rows, cols);
    let solutions = Solver::solve(grid);
    let n_displayed = opts.head().unwrap_or_else(|| solutions.len());

    for (i, solution) in solutions.into_iter().enumerate().take(n_displayed) {
        println!("SOLUTION {}\n{}", i + 1, solution);
    }

    Ok(())
}
