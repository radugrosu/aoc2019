use crate::error::Error;
use crate::output::{DayOutput, Output};
use std::io::BufRead;

pub fn run_part<F>(input: &String, func: F) -> Result<Output, Error>
where
    F: Fn(usize) -> usize,
{
    let out = input
        .lines()
        .map(|l| l.parse::<usize>().expect("Cannot parse {l} as usize"))
        .map(func)
        .sum();
    return Ok(Output::Number(out));
}

fn fuel_req_one(n: usize) -> usize {
    (n / 3).checked_sub(2).map_or(0, |x| x)
}

fn fuel_req_two(n: usize) -> usize {
    let mut total: usize = 0;
    let mut current: usize = fuel_req_one(n);
    while current > 0 {
        total += current;
        current = fuel_req_one(current);
    }
    total
}

pub fn run<R>(input: &mut R) -> Result<DayOutput, Error>
where
    R: BufRead,
{

    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    Ok(DayOutput {
        one: run_part(&buf, fuel_req_one)?,
        two: Some(run_part(&buf, fuel_req_two)?),
    })
}

#[test]
fn test_part_one() {
    // For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
    // For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
    // For a mass of 1969, the fuel required is 654.
    // For a mass of 100756, the fuel required is 33583.
    assert_eq!(fuel_req_one(12), 2);
    assert_eq!(fuel_req_one(14), 2);
    assert_eq!(fuel_req_one(1969), 654);
    assert_eq!(fuel_req_one(100756), 33583);
}

#[test]
fn test_part_two() {
    // A module of mass 14 requires 2 fuel. This fuel requires no further fuel (2 divided by 3 and rounded down is 0, which would call for a negative fuel), so the total fuel required is still just 2.
    // At first, a module of mass 1969 requires 654 fuel. Then, this fuel requires 216 more fuel (654 / 3 - 2). 216 then requires 70 more fuel, which requires 21 fuel, which requires 5 fuel, which requires no further fuel. So, the total fuel required for a module of mass 1969 is 654 + 216 + 70 + 21 + 5 = 966.
    // The fuel required by a module of mass 100756 and its fuel is: 33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2 = 50346
    assert_eq!(fuel_req_two(12), 2);
    assert_eq!(fuel_req_two(14), 2);
    assert_eq!(fuel_req_two(1969), 966);
    assert_eq!(fuel_req_two(100756), 50346);
}
