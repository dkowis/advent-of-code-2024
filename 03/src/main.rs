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

    let part2_result = part2(&result)?;
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(input: &[String]) -> Result<isize, DayError> {
    let sum = input.iter().map(|line| line_mult(line).unwrap()).sum();

    Ok(sum)
}

enum Op {
    Multiply(isize, isize),
    Do(),
    Dont(),
}

fn line_mult(input: &str) -> Result<isize, DayError> {
    let regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let sum: isize = regex
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

fn line_opcodes(input: &str) -> Result<Vec<Op>, DayError> {
    let opcode_regex: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap());
    let mult_regex: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

    let mut ops: Vec<Op> = Vec::new();
    opcode_regex.captures_iter(input).for_each(|capture| {
        let s = capture.get(1).unwrap().as_str();
        if s.contains("mul") {
            debug!("MULT: {}", s);
            let captures = mult_regex.captures(s).unwrap();
            let a = captures.get(1).unwrap().as_str().parse::<isize>().unwrap();
            let b = captures.get(2).unwrap().as_str().parse::<isize>().unwrap();
            debug!("a: {}  b: {}", a, b);
            ops.push(Op::Multiply(a, b));
        } else if s.contains("don't") {
            ops.push(Op::Dont());
        } else if s.contains("do") {
            ops.push(Op::Do())
        } else {
            panic!("UNRECOGNIZED OP!")
        }
    });
    Ok(ops)
}

fn part2(input: &[String]) -> Result<isize, DayError> {
    //build the list of all lines
    let ops: Vec<Op> = input
        .iter()
        .flat_map(|x| line_opcodes(x))
        .flatten()
        .collect_vec();

    let mut enabled = true;
    let mut sum: isize = 0;
    for op in ops {
        match op {
            Op::Multiply(a, b) => {
                if enabled {
                    sum += a * b;
                }
            }
            Op::Do() => {
                enabled = true;
            }
            Op::Dont() => {
                enabled = false;
            }
        }
    }

    Ok(sum)
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
    use pretty_assertions::assert_eq;
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let vec = vec![input.to_string()];

        let result = part2(&vec)?;
        assert_eq!(result, 48);
        Ok(())
    }
}
