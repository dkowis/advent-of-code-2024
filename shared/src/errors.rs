use flexi_logger::FlexiLoggerError;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Couldn't parse input")]
    IoProblem(#[from] std::io::Error),
    #[error("Unable to start logger")]
    FlexiLoggerProblem(#[from] FlexiLoggerError),
    #[error("Input Parsing problem!")]
    ParsingError(#[from] ParseError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Cannot parse an integer")]
    IntParse(#[from] ParseIntError),
    #[error("Couldn't parse input")]
    IoProblem(#[from] std::io::Error),
}
