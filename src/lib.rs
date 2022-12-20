mod day06;
mod day07;

use anyhow::{Context, Result};
pub use day06::*;
pub use day07::*;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

const LINE_LENGTH: usize = 40;

#[derive(Debug)]
pub struct InputArgs {
    pub problem_num: (usize, usize),
    pub input_file: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing arguments")]
    MissingArgument,

    #[error("Input doesn't satisfy assumed guarantees or is malformed")]
    BadInput,

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

pub fn read_args() -> Result<InputArgs> {
    let day = env::args().nth(1);
    let part = env::args().nth(2);
    let filename = env::args().nth(3);
    match (day, part, filename) {
        (Some(d), Some(p), Some(f)) => Ok(InputArgs {
            problem_num: (
                d.parse().context("Problem parsing problem day")?,
                p.parse().context("Problem parsing problem part")?,
            ),
            input_file: f,
        }),
        _ => Err(AppError::MissingArgument.into()),
    }
}

struct InputReader<P, T>
where
    P: Fn(&str) -> Result<T, anyhow::Error>,
{
    buf_reader: BufReader<File>,
    string_buffer: String,
    parser: P,
    finished: bool,
}

impl<P, T> InputReader<P, T>
where
    P: Fn(&str) -> Result<T, anyhow::Error>,
{
    pub fn new(filename: &str, parser: P) -> Result<InputReader<P, T>> {
        let buf_reader =
            BufReader::new(File::open(filename).context("Problem opening specified file")?);
        let string_buffer = String::with_capacity(LINE_LENGTH);
        Ok(InputReader {
            buf_reader,
            string_buffer,
            parser,
            finished: false,
        })
    }
}

impl<P, T> Iterator for InputReader<P, T>
where
    P: Fn(&str) -> Result<T, anyhow::Error>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.finished {
            match self.buf_reader.read_line(&mut self.string_buffer) {
                Ok(bytes) => {
                    self.string_buffer
                        .truncate(self.string_buffer.trim_end().len());
                    match (self.parser)(&self.string_buffer) {
                        Ok(parsed_val) => {
                            if bytes == 0 {
                                self.finished = true;
                            }
                            self.string_buffer.clear();
                            Some(parsed_val)
                        }
                        Err(_) => {
                            self.finished = true;
                            None
                        }
                    }
                }
                Err(_io_error) => {
                    panic!("Fatal error while reading input");
                    //self.finished = true;
                    //Some(Err(
                    //    Into::<anyhow::Error>::into(io_error).context("Unknown IO error")
                    //))
                }
            }
        } else {
            None
        }
    }
}
