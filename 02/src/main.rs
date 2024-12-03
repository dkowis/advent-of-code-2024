pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    Logger::try_with_env_or_str("warn")?.start()?;

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
    assert_eq!(part1_result, 472);

    Ok(())
}

fn part1(matrix: &Vec<Vec<usize>>) -> Result<usize, DayError> {
    let transformed = transform_reports(matrix);
    debug!("transformed: {:#?}", transformed);
    //if any of the changes in levels is more than 3 it's a fail

    let count = transformed.iter().filter(|report| report.is_safe() ).count();

    Ok(count)
}

#[derive(Debug)]
struct Report {
    row: Vec<usize>,
    transformed: Vec<isize>,
}

impl Report {
    fn transform_report(row: &Vec<usize>) -> Vec<isize> {
        let mut result_vec: Vec<isize> = Vec::with_capacity(row.len() - 1);
        debug!("result_vec: {:#?}", result_vec);
        for i in 0..row.len() - 1 {
            result_vec.push(row[i] as isize - row[i + 1] as isize);
        }
        result_vec
    }
    pub fn new(row: Vec<usize>) -> Report {
        let transformed = Report::transform_report(&row);
        Report { row, transformed }
    }
    pub fn is_safe(&self) -> bool {
        self.transformed.iter().all(|x| x.abs() > 0 && x.abs() <= 3)
            && (self.transformed.iter().all(|x| x.is_negative())
                || self.transformed.iter().all(|x| x.is_positive()))
    }
}

fn transform_reports(matrix: &Vec<Vec<usize>>) -> Vec<Report> {
    let transformed: Vec<Report> = matrix
        .iter()
        .map(|row| {
            debug!("row: {:#?}", row);
            Report::new(row.clone())
        })
        .collect();
    transformed
}


fn dampener(row: &Vec<usize>) -> Result<Vec<isize>, DayError> {
    //return only the fixed one, or none if it cannot be fixed.
    //somehow, I need to figure out if any one problem would break things, does it fix things?
    //find any item in the array that doesn't meet the requirements, remove it and try again

    todo!()
}

fn part2(matrix: &Vec<Vec<usize>>) -> Result<usize, DayError> {
    let transformed = transform_reports(matrix);

    todo!()
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialize() {
        INIT.call_once(|| {
            Logger::try_with_env_or_str("debug")
                .unwrap()
                .start()
                .unwrap();
        });
    }

    #[test]
    fn day2_part_one() -> Result<(), DayError> {
        initialize();

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
        initialize();

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
