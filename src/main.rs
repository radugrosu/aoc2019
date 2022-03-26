use aoc2019::{reader::Reader, error::Error, output::Output};
use std::path::PathBuf;
use std::{fs, io};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    day: usize,
    input: Option<PathBuf>,
}


fn run() -> Result<Output, Error> {
    let opt = Opt::from_args();
    let input = std::io::stdin();
    let reader = match opt.input {
        Some(path) => {
            let file = fs::File::open(path).unwrap();
            Reader::BufReader(io::BufReader::new(file))
        }
        None => Reader::Stdin(input.lock()),
    };
    let r = match opt.day {
        1 => aoc2019::day01::run(reader),
        n if n > 1 && n < 26 => return Err(Error::Custom(format!("Day {} is not yet implemented", n))),
        _ => return Err(Error::Custom(format!("Day must be between 1 and 25")))
    };
    Ok(r)
}

fn main() {
    match run() {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
}
