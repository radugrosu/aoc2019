pub mod day01;
pub mod day02;

pub mod reader {
    use std::{
        fs::File,
        io::{BufRead, Read, BufReader, Result, StdinLock},
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
    impl <'a> Read for Reader<'a>{
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            match self {
                Self::BufReader(reader) => reader.read(buf),
                Self::Stdin(reader) => reader.read(buf),
            }
        }
    }
}
