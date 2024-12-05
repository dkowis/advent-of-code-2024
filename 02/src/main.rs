pub use shared::prelude::*;
use std::fmt::Debug;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let loaded = load_input(2, 1, parse_word)?;
    let matrix: Vec<Vec<usize>> = loaded
        .iter()
        .map(|row| row.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();
    let part1_result = part1(&matrix)?;
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&matrix)?;
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(matrix: &[Vec<usize>]) -> Result<usize, DayError> {
    let transformed = transform_reports(matrix);
    debug!("transformed: {:#?}", transformed);
    //if any of the changes in levels is more than 3 it's a fail

    let count = transformed.iter().filter(|report| report.is_safe()).count();

    Ok(count)
}

#[derive(Debug)]
struct Report {
    row: Vec<usize>,
    levels: Vec<ReactorLevel>,
}

struct ReactorLevel {
    start: usize,
    end: usize,
    diff: isize,
}
impl Debug for ReactorLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Level {{ start: {}, end: {}, diff: {}, is_decreasing: {}, is_increasing: {} }}",
            self.start,
            self.end,
            self.diff,
            self.is_decreasing(),
            self.is_increasing()
        )
    }
}
impl ReactorLevel {
    fn new(start: usize, end: usize) -> ReactorLevel {
        let diff = start as isize - end as isize;
        ReactorLevel { start, end, diff }
    }

    fn is_safe(&self) -> bool {
        let abs = self.diff.abs();
        abs > 0 && abs <= 3
    }

    fn is_increasing(&self) -> bool {
        self.diff > 0
    }

    fn is_decreasing(&self) -> bool {
        self.diff < 0
    }
}

impl Report {
    fn transform_report(row: &[usize]) -> Vec<ReactorLevel> {
        let mut result_vec: Vec<ReactorLevel> = Vec::with_capacity(row.len() - 1);
        debug!("result_vec: {:#?}", result_vec);
        for i in 0..row.len() - 1 {
            result_vec.push(ReactorLevel::new(row[i], row[i + 1]));
        }
        result_vec
    }
    pub fn new(row: Vec<usize>) -> Report {
        let levels = Report::transform_report(&row);
        Report { row, levels }
    }
    pub fn is_safe(&self) -> bool {
        self.levels.iter().all(|x| x.is_safe())
            && (self.levels.iter().all(|x| x.is_increasing())
                || self.levels.iter().all(|x| x.is_decreasing()))
    }
    pub fn dampened(&self) -> Option<Report> {
        if self.is_safe() {
            panic!("This report is already safe");
        } else {
            //Create a new report from this one, that has been dampened, and that is safe.
            //Or none, because nothing could fix it
            for i in 0..self.row.len() {
                //Work through removing all the levels
                let mut removed = self.row.clone();
                removed.remove(i);
                let trial = Report::new(removed);
                if trial.is_safe() {
                    return Some(trial);
                }
            }
            //If we couldn't find one, nothing is safe
            None
        }
    }
}

fn transform_reports(matrix: &[Vec<usize>]) -> Vec<Report> {
    let transformed: Vec<Report> = matrix
        .iter()
        .map(|row| {
            debug!("row: {:#?}", row);
            Report::new(row.clone())
        })
        .collect();
    transformed
}

fn part2(matrix: &[Vec<usize>]) -> Result<usize, DayError> {
    let transformed = transform_reports(matrix);
    debug!("transformed: {:#?}", transformed);
    //if any of the changes in levels is more than 3 it's a fail
    let mut count: usize = 0;
    for report in transformed.iter() {
        //Do the transformation trying to dampen a report
        if !report.is_safe() {
            if let Some(dampened) = report.dampened() {
                debug!("dampened: {:?} to {:?}", report, dampened);
                count += 1;
            };
        } else {
            count += 1;
        }
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
    use pretty_assertions::assert_eq;
    use shared::prelude::*;

    #[test]
    fn day2_part_one() -> Result<(), DayError> {
        initialize_logger(None);

        let input_string = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#
        .trim();

        let input: Vec<Vec<usize>> = input_string
            .split("\n")
            .map(|y| y.split(" ").map(|a| a.parse().unwrap()).collect())
            .collect();

        debug!("input: {:?}", input);
        let result = part1(&input)?;
        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn day2_part_two() -> Result<(), DayError> {
        initialize_logger(None);

        let input_string = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
        "#
        .trim();

        let input: Vec<Vec<usize>> = input_string
            .split("\n")
            .map(|y| y.split(" ").map(|a| a.parse().unwrap()).collect())
            .collect();

        debug!("input: {:?}", input);
        let result = part2(&input)?;
        assert_eq!(result, 4);
        Ok(())
    }
}
