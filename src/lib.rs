#[macro_use]
mod macros;

pub mod day01;
pub mod day02;
pub mod day03;

pub mod output {
    use std::fmt;
    #[derive(Debug, PartialEq)]
    pub enum Output {
        Number(usize),
        String(String),
    }
    impl fmt::Display for Output {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Number(s) => write!(f, "{}", s),
                Self::String(s) => write!(f, "{}", s),
            }
        }
    }
    impl Default for Output {
        fn default() -> Self {
            Self::String("<NOT DONE>".to_string())
        }
    }
    #[derive(Debug, Default)]
    pub struct DayOutput {
        pub one: Output,
        pub two: Option<Output>,
    }
    impl fmt::Display for DayOutput {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut out = format!("part one: {}", self.one);
            if let Some(s) = &self.two {
                out = format!("{}\npart two: {}", out, s)
            }
            write!(f, "{}", out)
        }
    }
}

pub mod error {

    use std::fmt;
    use std::io;
    #[derive(Debug)]
    pub enum Error {
        Custom(String),
        Io(io::Error),
        ParseInt(std::num::ParseIntError),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
                Self::Io(s) => write!(f, "{}", s),
                Self::ParseInt(s) => write!(f, "{}", s),
            }
        }
    }
    impl std::error::Error for Error {}

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Self::Io(e)
        }
    }
    impl From<std::num::ParseIntError> for Error {
        fn from(e: std::num::ParseIntError) -> Self {
            Self::ParseInt(e)
        }
    }
}
pub mod reader {
    use std::{
        fs::File,
        io::{BufRead, BufReader, Read, Result, StdinLock},
    };
    pub enum Reader<'a> {
        BufReader(BufReader<File>),
        Stdin(StdinLock<'a>),
    }
    impl<'a> BufRead for Reader<'a> {
        fn fill_buf(&mut self) -> Result<&[u8]> {
            match self {
                Self::BufReader(reader) => reader.fill_buf(),
                Self::Stdin(reader) => reader.fill_buf(),
            }
        }

        fn consume(&mut self, amt: usize) {
            match self {
                Self::BufReader(reader) => reader.consume(amt),
                Self::Stdin(reader) => reader.consume(amt),
            }
        }
    }
    impl<'a> Read for Reader<'a> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            match self {
                Self::BufReader(reader) => reader.read(buf),
                Self::Stdin(reader) => reader.read(buf),
            }
        }
    }
}
