#[macro_use]
mod macros;

pub mod day01;
pub mod day02;
pub mod output {
    use std::fmt;
    #[derive(Debug)]
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
}

pub mod error {

    use std::fmt;
    #[derive(Debug)]
    pub enum Error {
        Custom(String),
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Custom(s) => write!(f, "{}", s),
            }
        }
    }
    impl std::error::Error for Error {}
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
