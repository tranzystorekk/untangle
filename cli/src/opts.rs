use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "untangle")]
pub struct Opts {
    #[structopt(short, long, help = "Show only the first N solutions")]
    head: Option<usize>,

    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}

impl Opts {
    pub fn input(&self) -> std::io::Result<impl BufRead> {
        let result: Box<dyn BufRead> = match self.input.as_ref() {
            Some(path) => {
                let file = File::open(path)?;
                let reader = BufReader::new(file);
                Box::new(reader)
            },
            _ => {
                let reader = BufReader::new(std::io::stdin());
                Box::new(reader)
            },
        };

        Ok(result)
    }

    pub fn head(&self) -> Option<usize> {
        self.head
    }
}