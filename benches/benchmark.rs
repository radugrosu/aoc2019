use aoc2019;
use criterion::{criterion_group, criterion_main, Criterion};
use std::{fs, io};

fn bench(c: &mut Criterion, day: &str) {
    let data = fs::read_to_string(format!("data/{}.txt", day)).unwrap();

    c.bench_function(day, |b| {
        let runner = match day {
            "day01" => aoc2019::day01::run,
            "day02" => aoc2019::day02::run,
            "day03" => aoc2019::day03::run,
            "day04" => aoc2019::day04::run,
            _ => unimplemented!(),
        };
        b.iter(|| {
            let mut reader = io::BufReader::new(data.as_bytes());
            runner(&mut reader).unwrap();
        })
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    for day in vec!["day01", "day02", "day03", "day04"] {
        bench(c, day)
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
