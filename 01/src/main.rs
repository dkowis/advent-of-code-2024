use regex::Regex;
pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    Logger::try_with_env_or_str("warn")?.start()?;

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let part1_input = load_input(1, 1, parse_word)?;

    let part1_result = part1(&part1_input)?;
    println!("Part 1: {}", part1_result);


    // let result2 = load_input(1, 2, parse_i32)?;
    // for x in result2 {
    //     println!("ITS A NUMBER: {}", x);
    //     let _ = part2();
    // }

    Ok(())
}

fn part1(input: &Vec<String>) -> Result<usize, DayError> {
    let (mut vec1, mut vec2) = parse_input_string(input)?;
    vec1.sort();
    vec2.sort();

    let mut result = 0;
    for i in 0..vec1.len() {
        let first = vec1[i];
        let second = vec2[i];
        debug!("PAIR: {} {}", first, second);
        let diff = (first - second).abs() as usize;
        result += diff;
    }

    Ok(result)
}

fn part2() -> Result<(), DayError> {
    todo!();
}

fn parse_input_string(input: &Vec<String>) -> Result<(Vec<isize>, Vec<isize>), DayError> {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();

    let actual_input = input
        .iter()
        .map(|x| {
            if let Some(captures) = re.captures(x) {
                let first: isize = captures[1].parse().unwrap();
                let second: isize = captures[2].parse().unwrap();
                return (first, second);
            } else {
                panic!("No captures");
            }
        })
        .collect::<Vec<(isize, isize)>>();
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
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;
    use std::sync::Once;
    use crate::{parse_input_string, part1};

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
    fn day1_part_one() -> Result<(), DayError> {
        initialize();
        let input = r#"
3   4
4   3
2   5
1   3
3   9
3   3
        "#
        .trim();

        let parsed:Vec<String> = input.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
        let result = part1(&parsed)?;
        assert_eq!(result, 11);

        Ok(())
    }

    #[test]
    fn day1_part_two() -> Result<(), DayError> {
        initialize();

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
