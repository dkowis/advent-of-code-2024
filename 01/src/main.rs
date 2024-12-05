use regex::Regex;
pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let part1_input = load_input(1, 1, parse_word)?;

    let part1_result = part1(&part1_input)?;
    println!("Part 1: {}", part1_result);

    let part2_result = part2(&part1_input)?;
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(input: &[String]) -> Result<usize, DayError> {
    let (mut vec1, mut vec2) = parse_input_string(input)?;
    vec1.sort();
    vec2.sort();

    let mut result = 0;
    for i in 0..vec1.len() {
        let first = vec1[i];
        let second = vec2[i];
        debug!("PAIR: {} {}", first, second);
        let diff = (first as isize - second as isize).unsigned_abs();
        result += diff;
    }

    Ok(result)
}

fn part2(input: &[String]) -> Result<usize, DayError> {
    let (vec1, vec2) = parse_input_string(input)?;

    let result = vec1
        .iter()
        .map(|x| {
            let count = vec2.iter().filter(|y| **y == *x).count();
            x * count
        })
        .sum();

    Ok(result)
}

fn parse_input_string(input: &[String]) -> Result<(Vec<usize>, Vec<usize>), DayError> {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let actual_input = input
        .iter()
        .map(|x| {
            if let Some(captures) = re.captures(x) {
                let first: usize = captures[1].parse().unwrap();
                let second: usize = captures[2].parse().unwrap();
                (first, second)
            } else {
                panic!("No captures");
            }
        })
        .collect::<Vec<(usize, usize)>>();
    let mut vec1 = Vec::with_capacity(actual_input.len());
    let mut vec2 = Vec::with_capacity(actual_input.len());
    for x in actual_input {
        vec1.push(x.0);
        vec2.push(x.1);
    }
    Ok((vec1, vec2))
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
    use pretty_assertions::assert_eq;
    use shared::prelude::*;
    use std::sync::Once;

    #[test]
    fn day1_part_one() -> Result<(), DayError> {
        initialize_logger(None);
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#
        .trim();

        let parsed: Vec<String> = input
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let result = part1(&parsed)?;
        assert_eq!(result, 11);

        Ok(())
    }

    #[test]
    fn day1_part_two() -> Result<(), DayError> {
        initialize_logger(None);
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#
        .trim();
        let parsed: Vec<String> = input
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let result = part2(&parsed)?;
        assert_eq!(result, 31);

        Ok(())
    }
}
