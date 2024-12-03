use crate::errors::ParseError;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn load_input<R>(
    day: usize,
    part: usize,
    f: fn(String) -> Result<R, Error>,
) -> Result<Vec<R>, Error> {
    let file = File::open(format!("inputs/{:02}/{}.txt", day, part))?;
    let br = BufReader::new(file);

    br.lines()
        .map(|line| line.and_then(|v| f(v).map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn load_input2<R>(
    day: usize,
    part: usize,
    f: fn(String) -> Result<R, ParseError>,
) -> Result<Vec<R>, ParseError> {
    let file = File::open(format!("inputs/{:02}/{}.txt", day, part))?;
    let br = BufReader::new(file);

    br.lines()
        .map(|line| line.and_then(|v| f(v).map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .map(|item| item.map_err(ParseError::IoProblem))
        .collect()
}

pub fn parse_word(s: String) -> Result<String, Error> {
    Ok(s)
}

pub fn parse_i32(s: String) -> Result<i32, Error> {
    s.parse::<i32>()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub fn parse_isize(s: String) -> Result<isize, Error> {
    s.parse::<isize>()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub fn parse_usize(s: String) -> Result<usize, Error> {
    s.parse::<usize>().map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

pub fn parse_string(s: String) -> Result<String, ParseError> {
    Ok(s)
}
