use crate::error::Error;
use crate::output::Output;
use std::io::BufRead;

pub fn run<R>(input: R) -> Result<Output, Error>
where
    R: BufRead,
{
    let out = input
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<usize>()
                .expect("Cannot parse {l} as usize")
        })
        .map(|n| n / 3 - 2)
        .sum();
    return Ok(Output::Number(out));
}
