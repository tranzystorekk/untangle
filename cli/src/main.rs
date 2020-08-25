mod error;
mod opts;

use std::io::BufRead;

use itertools::Itertools;
use structopt::StructOpt;

use error::InputError;
use opts::Opts;
use untangle_solver::{Color, Grid, Solution};

#[macro_use]
extern crate anyhow;

fn process_input(input: impl BufRead) -> anyhow::Result<(usize, usize, Vec<Color>)> {
    let mut lines = input.lines();
    let (maybe_rows, maybe_cols) = lines
        .next()
        .ok_or_else(|| InputError::EmptyFile)??
        .split_whitespace()
        .map(|s| s.parse().map_err(|_| InputError::IncorrectShape))
        .collect_tuple()
        .ok_or(InputError::IncorrectShape)?;
    let (rows, cols) = (maybe_rows?, maybe_cols?);

    let maybe_fields = lines
        .map(|l| l.map(|s| s + " "))
        .collect::<Result<String, _>>()?;
    let fields = maybe_fields
        .split_whitespace()
        .map(|el| el.parse().map_err(|_| InputError::IncorrectGrid))
        .collect::<Result<_, _>>()?;

    Ok((rows, cols, fields))
}

fn print_solutions(solutions: Vec<Solution>, n: usize) {
    let formatter = solutions
        .into_iter()
        .enumerate()
        .take(n)
        .format_with("\n", |(i, solution), f| {
            f(&format_args!("SOLUTION {}\n{}", i + 1, solution))
        });

    print!("{}", formatter);
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let input = opts.input()?;

    let (rows, cols, fields) = process_input(input)?;

    ensure!(!fields.is_empty(), "Grid has no fields");
    ensure!(
        fields.len() == rows * cols,
        "Incorrect grid (either dimensions or missing content)"
    );

    let grid = Grid::from_vec_dims(fields, rows, cols);
    let solutions = untangle_solver::solve(grid);
    let n_displayed = opts.head().unwrap_or_else(|| solutions.len());

    print_solutions(solutions, n_displayed);

    Ok(())
}
