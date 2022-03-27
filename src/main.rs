use aoc2019::{bail, error::Error, output::DayOutput, reader::Reader};
use std::path::PathBuf;
use std::{error, fs, io, process};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    day: usize,
    input: Option<PathBuf>,
}
fn log_error_chain(e: Error) {
    eprintln!("error: {}", e);
    let mut e = &e as &dyn error::Error;
    while let Some(source) = e.source() {
        e = source;
        eprintln!("error: {}", e);
    }
    process::exit(1);
}

fn run() -> Result<DayOutput, Error> {
    let opt = Opt::from_args();
    let input = std::io::stdin();
    let mut reader = match opt.input {
        Some(path) => {
            let file = fs::File::open(path)?;
            Reader::BufReader(io::BufReader::new(file))
        }
        None => Reader::Stdin(input.lock()),
    };
    let r = match opt.day {
        1 => aoc2019::day01::run(&mut reader),
        n if n > 1 && n < 26 => bail!("Day {} is not yet implemented", n),
        _ => bail!("Day must be between 1 and 25"),
    };
    r
}

fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => log_error_chain(e),
    }
}
