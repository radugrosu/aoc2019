use std::io::BufRead;

pub fn run<R>(input: R) -> usize
where
    R: BufRead,
{
    input
        .lines()
        .map(|l| {
            l.unwrap()
                .parse::<usize>()
                .expect("Cannot parse {l} as usize")
        })
        .map(|n| n / 3 - 2)
        .sum()
}
