use aoc2019;
use aoc2019::reader::Reader;
use std::{fs, io};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    day: usize,
    input: Option<PathBuf>,
}

fn main() {
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
        2 => aoc2019::day02::run(reader),
        n if n > 1 && n < 26 => panic!("Day {} is not yet implemented", n),
        _ => panic!("Day must be between 1 and 25"),
    };
    println!("{}", r);
}
