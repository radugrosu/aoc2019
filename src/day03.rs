use crate::error::Error;
use crate::output::{DayOutput, Output};
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    U,
    R,
    D,
    L,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    size: usize,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = match s.chars().next().unwrap() {
            'L' => Direction::L,
            'U' => Direction::U,
            'R' => Direction::R,
            'D' => Direction::D,
            l => return Err(Error::Custom(format!("Illegal character {}", l))),
        };
        let size = s.chars().skip(1).collect::<String>().parse::<usize>()?;
        Ok(Self { dir, size })
    }
}

fn parse_line(input: &str) -> Vec<Instruction> {
    input
        .split(',')
        .map(|n| {
            n.trim()
                .parse::<Instruction>()
                .expect(&format!("Cannot parse {} as `Instruction`", n))
        })
        .collect()
}
fn parse_input(input: &str) -> Result<(Vec<Instruction>, Vec<Instruction>), Error> {
    let mut lines = input.lines();
    Ok((
        lines
            .next()
            .map(parse_line)
            .ok_or(Error::Custom(format!("Cannot parse line 1")))?,
        lines
            .next()
            .map(parse_line)
            .ok_or(Error::Custom(format!("Cannot parse line 2")))?,
    ))
}

#[derive(Default, Clone)]
struct Point {
    x: isize,
    y: isize,
    d: usize,
}
impl Eq for Point {}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.manhattan().cmp(&other.manhattan()))
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.manhattan().cmp(&other.manhattan())
    }
}

impl Point {
    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    fn inc(&self, dir: &Direction) -> Self {
        match dir {
            Direction::D => Self {
                x: self.x,
                y: self.y - 1,
                d: self.d + 1,
            },
            Direction::U => Self {
                x: self.x,
                y: self.y + 1,
                d: self.d + 1,
            },
            Direction::L => Self {
                x: self.x - 1,
                y: self.y,
                d: self.d + 1,
            },
            Direction::R => Self {
                x: self.x + 1,
                y: self.y,
                d: self.d + 1,
            },
        }
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

fn get_coordinate_list(instructions: &Vec<Instruction>) -> Vec<Point> {
    let current = Point::default();
    let mut coords = vec![current];
    for instruction in instructions {
        for _ in 0..instruction.size {
            let current = coords.last().unwrap().inc(&instruction.dir);
            coords.push(current)
        }
    }
    coords
}

fn part_one(one: &HashSet<Point>, two: &HashSet<Point>) -> Result<usize, Error> {
    let intersection: HashSet<Point> = one.intersection(two).cloned().into_iter().collect();
    let closest = intersection.into_iter().min().unwrap();
    Ok(closest.manhattan())
}

fn part_two(one: &HashSet<Point>, two: &mut HashSet<Point>) -> Result<usize, Error> {
    let mut closest = usize::MAX;
    for item in one {
        if let Some(other) = two.take(&item) {
            let d = item.d + other.d;
            if d < closest {
                closest = d;
            }
        }
    }
    Ok(closest)
}

fn get_inputs(s: &str) -> Result<(HashSet<Point>, HashSet<Point>), Error> {
    let (one, two) = parse_input(s)?;
    let p = get_coordinate_list(&one);
    let q = get_coordinate_list(&two);
    let c1: HashSet<Point> = HashSet::from_iter(p.into_iter().skip(1));
    let c2: HashSet<Point> = HashSet::from_iter(q.into_iter().skip(1));
    Ok((c1, c2))
}

pub fn run<R>(input: &mut R) -> Result<DayOutput, Error>
where
    R: BufRead,
{
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let (one, mut two) = get_inputs(&buf)?;

    Ok(DayOutput {
        one: Output::Number(part_one(&one, &two)?),
        two: Some(Output::Number(part_two(&one, &mut two)?)),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let data = "R8,U5,L5,D3\nU7,R6,D4,L4";
        let (one, mut two) = get_inputs(data).unwrap();
        assert_eq!(part_one(&one, &two).unwrap(), 6);
        assert_eq!(part_two(&one, &mut two).unwrap(), 30);

        let data = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
        let (one, mut two) = get_inputs(data).unwrap();
        assert_eq!(part_one(&one, &two).unwrap(), 159);
        assert_eq!(part_two(&one, &mut two).unwrap(), 610);

        let data =
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let (one, mut two) = get_inputs(data).unwrap();
        assert_eq!(part_one(&one, &two).unwrap(), 135);
        assert_eq!(part_two(&one, &mut two).unwrap(), 410);
    }
}
