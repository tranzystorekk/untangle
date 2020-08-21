use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "untangle")]
pub struct Opts {
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,
}