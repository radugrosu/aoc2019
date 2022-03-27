use crate::error::Error;
use crate::output::{DayOutput, Output};
use std::io::BufRead;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| {
            n.trim()
                .parse::<usize>()
                .expect("Cannot parse {n} as usize")
        })
        .collect()
}
pub fn run<R>(input: &mut R) -> Result<DayOutput, Error>
where
    R: BufRead,
{
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let input: Vec<usize> = parse_input(&buf);

    Ok(DayOutput {
        one: Output::Number(part_one(&input)?),
        two: Some(Output::Number(part_two(&input)?)),
    })
}
pub fn part_one(input: &Vec<usize>) -> Result<usize, Error> {
    let mut input = input.clone();
    input[1] = 12;
    input[2] = 2;
    run_op(&mut input)
}
fn find(haystack: &Vec<usize>, needle: usize) -> Result<Option<(usize, usize)>, Error> {
    for i in 0..100 {
        for j in 0..100 {
            let mut input = haystack.clone();
            input[1] = i;
            input[2] = j;
            if run_op(&mut input)? == needle {
                return Ok(Some((i, j)));
            }
        }
    }
    Ok(None)
}
pub fn part_two(input: &Vec<usize>) -> Result<usize, Error> {
    match find(input, 19690720)? {
        Some((noun, verb)) => Ok(100 * noun + verb),
        None => bail!("Couldn't find suitable values"),
    }
}

pub fn run_op(input: &mut Vec<usize>) -> Result<usize, Error> {
    let mut c: usize = 0;
    loop {
        let out = if input[c] == 99 {
            return Ok(input[0]);
        } else {
            let left_pos = input[c + 1];
            let right_pos = input[c + 2];
            match input[c] {
                1 => input[left_pos] + input[right_pos],
                2 => input[left_pos] * input[right_pos],
                n => bail!("No such opcode {}", n),
            }
        };
        let out_pos = input[c + 3];
        input[out_pos] = out;
        c += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        assert_eq!(run_op(&mut parse_input("1,0,0,0,99")).unwrap(), 2);
        assert_eq!(run_op(&mut parse_input("1,1,1,4,99,5,6,0,99")).unwrap(), 30);
    }
}
