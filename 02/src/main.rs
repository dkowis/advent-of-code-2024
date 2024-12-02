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

    Ok(())
}

fn part1(matrix: &Vec<Vec<usize>>) -> Result<usize, DayError> {
    let transformed: Vec<Vec<isize>> = matrix
        .iter()
        .map(|row| {
            debug!("row: {:#?}", row);
            let mut result_vec: Vec<isize> = Vec::with_capacity(row.len() - 1);
            debug!("result_vec: {:#?}", result_vec);
            for i in 0..row.len() - 1 {
                result_vec.push(row[i] as isize - row[i + 1] as isize);
            }
            result_vec
        })
        .collect();
    debug!("transformed: {:#?}", transformed);
    //if any of the changes in levels is more than 3 it's a fail

    let count = transformed
        .iter()
        .filter(|row| {
            row.iter().all(|x| x.abs() > 0 && x.abs() <= 3)
                && (row.iter().all(|x| x.is_negative()) || row.iter().all(|x| x.is_positive()))
        })
        .count();

    Ok(count)
}

fn part2() -> Result<(), DayError> {
    todo!();
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::part1;
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

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }

    #[test]
    fn day2_part_two() -> Result<(), DayError> {
        initialize();

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
