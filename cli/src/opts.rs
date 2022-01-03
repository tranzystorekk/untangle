use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(about)]
pub struct Opts {
    /// Show only the first N solutions
    #[clap(short, long, value_name = "N")]
    number: Option<usize>,

    /// File to read from; defaults to STDIN if not provided
    input: Option<PathBuf>,
}

impl Opts {
    pub fn input(&self) -> std::io::Result<impl BufRead> {
        let result: Box<dyn BufRead> = match self.input.as_ref() {
            Some(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                Box::new(reader)
            }
            _ => {
                let reader = BufReader::new(std::io::stdin());
                Box::new(reader)
            }
        };

        Ok(result)
    }

    pub fn head(&self) -> Option<usize> {
        self.number
    }
}
