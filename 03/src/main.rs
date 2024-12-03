use itertools::Itertools;
use once_cell::sync::Lazy;
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

    let result = load_input(3, 1, parse_word)?;
    let part1_result = part1(&result)?;
    println!("Part 1: {}", part1_result);
    
    // let result2 = load_input(3, 2, parse_i32)?;
    // for x in result2 {
    //     println!("ITS A NUMBER: {}", x);
    //     let _ = part2();
    // }

    Ok(())
}

fn part1(input: &[String]) -> Result<isize, DayError> {
    let sum = input.iter()
        .map(|line| line_mult(line).unwrap())
        .sum();
    
    Ok(sum)
}

fn line_mult(input: &str) -> Result<isize, DayError> {
    let REGEX: Lazy<Regex> = Lazy::new( || Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let sum: isize = REGEX
        .captures_iter(input)
        .map(|x| {
            debug!("CAPTURE: {:?}", x);
            (
                x.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                x.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            )
        })
        .map(|(a, b)| {
            debug!("a: {:?}  b: {:?}", a, b);
            a * b
        })
        .sum();

    debug!("SUM: {}", sum);

    Ok(sum)

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
    fn day3_part_one() -> Result<(), DayError> {
        initialize();
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let vec = vec![input.to_string()];

        let result = part1(&vec)?;
        assert_eq!(result, 161);

        Ok(())
    }

    #[test]
    fn day3_part_two() -> Result<(), DayError> {
        initialize();

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
