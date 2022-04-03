use crate::error::Error;
use crate::output::{DayOutput, Output};
use std::convert::TryInto;
use std::io::BufRead;

fn parse_input(s: &str) -> Result<(usize, usize), Error> {
    let mut items = s.split('-');
    let start = items
        .next()
        .ok_or_else(|| error!("malformed input"))?
        .parse::<usize>()?;
    let stop = items
        .next()
        .ok_or_else(|| error!("malformed input"))?
        .parse::<usize>()?;
    Ok((start, stop))
}

fn num_to_vec(n: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(6);
    let mut c = n;
    while c > 0 {
        out.push((c % 10) as u8);
        c /= 10;
    }
    out.into_iter().rev().collect()
}

fn has_matching_consecutive(v: &[u8]) -> Result<bool, Error> {
    for item in v.windows(2) {
        let [i, j]: [u8; 2] = item.try_into()?;
        if i == j {
            return Ok(true);
        }
    }
    Ok(false)
}

fn has_matching_consecutive_2(v: &[u8]) -> Result<bool, Error> {
    for item in v.windows(4) {
        let [i, j, k, l]: [u8; 4] = item.try_into()?;
        if (i != j) && (j == k) && (k != l) {
            return Ok(true);
        }
    }

    match v[..] {
        [a, b, c, ..] => {
            if a == b && b != c {
                return Ok(true);
            }
        }
        _ => bail!("array has less than 4 elements"),
    };
    match v[..] {
        [.., a, b, c] => {
            if a != b && b == c {
                return Ok(true);
            }
        }
        _ => bail!("array has less than 4 elements"),
    };
    Ok(false)
}

fn is_monotonic(v: &[u8]) -> Result<bool, Error> {
    for item in v.windows(2) {
        let [i, j]: [u8; 2] = item.try_into()?;
        if i > j {
            return Ok(false);
        }
    }
    Ok(true)
}

fn is_valid_1(v: &[u8]) -> Result<bool, Error> {
    Ok(has_matching_consecutive(v)? && is_monotonic(v)?)
}

fn is_valid_2(v: &[u8]) -> Result<bool, Error> {
    Ok(has_matching_consecutive_2(v)? && is_monotonic(v)?)
}

fn both_parts(start: usize, stop: usize) -> Result<(usize, usize), Error> {
    let mut total_one = 0;
    let mut total_two = 0;
    for i in start..=stop {
        let v = num_to_vec(i);
        if is_valid_1(&v)? {
            total_one += 1;
        }
        if is_valid_2(&v)? {
            total_two += 1
        }
    }
    Ok((total_one, total_two))
}

pub fn run<R>(input: &mut R) -> Result<DayOutput, Error>
where
    R: BufRead,
{
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (start, stop) = parse_input(&buf)?;
    let (one, two) = both_parts(start, stop)?;

    Ok(DayOutput {
        one: Output::Number(one),
        two: Some(Output::Number(two)),
    })
}
