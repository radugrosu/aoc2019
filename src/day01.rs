use std::io::BufRead;
use crate::output::Output;

pub fn run<R>(input: R) -> Output
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
    return Output::Number(out)
}
