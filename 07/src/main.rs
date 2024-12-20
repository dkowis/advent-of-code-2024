use crate::Operator::{Concat, Mult, Plus};
use code_timing_macros::time_snippet;
use itertools::{repeat_n, Itertools};
pub use shared::prelude::*;
use std::fmt::Formatter;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(7, 1, parse_word)?;
    let part1_result = time_snippet!(part1(&result)?);

    println!("Part 1: {}", part1_result);

    let part2_result = time_snippet!(part2(&result)?);
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(input: &[String]) -> Result<usize, DayError> {
    let result = input
        .iter()
        .map(|x| Problem::new(x))
        .filter(|x| x.is_solved())
        .collect::<Vec<Problem>>();

    debug!("SOLVED PROBLEMS: {:?}", result);
    let total: usize = result.iter().map(|x| x.test_value).sum::<usize>();
    Ok(total)
}

fn part2(input: &[String]) -> Result<usize, DayError> {
    let result = input
        .iter()
        .map(|x| Problem::new(x))
        .filter(|x| x.is_solved_part2())
        .collect::<Vec<Problem>>();

    debug!("SOLVED PROBLEMS: {:?}", result);
    let total: usize = result.iter().map(|x| x.test_value).sum::<usize>();
    Ok(total)
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Mult,
    Concat,
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Plus => write!(f, "+"),
            Mult => write!(f, "*"),
            Concat => write!(f, "||"),
        }
    }
}

struct Problem {
    test_value: usize,
    numbers: Vec<usize>,
}

impl std::fmt::Debug for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Problem {{ test_value: {}, numbers: {:?} }}",
            self.test_value, self.numbers
        )
    }
}
impl Problem {
    fn new(line: &str) -> Self {
        let mut parts = line.split(": ");
        let test_value = parts.next().unwrap().parse::<usize>().unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Self {
            test_value,
            numbers,
        }
    }

    fn do_math(numbers: &[usize], ops: &[&Operator]) -> usize {
        #[tailcall::tailcall]
        fn do_math_inner(numbers: &[usize], ops: &[&Operator], acc: usize) -> usize {
            if numbers.is_empty() {
                return acc;
            } else if acc == 0 {
                //take the first two numbers off, and do the first ops
                let first = numbers[0];
                let second = numbers[1];
                let new_numbers = &numbers[2..];
                let new_ops = &ops[1..];
                let new_acc = match ops[0] {
                    Plus => first + second,
                    Mult => first * second,
                    Operator::Concat => {
                        let mut result = String::new();
                        result.push_str(&first.to_string());
                        result.push_str(&second.to_string());
                        result.parse::<usize>().unwrap()
                    }
                };
                do_math_inner(new_numbers, new_ops, new_acc)
            } else {
                //take the acc and one number and do the first ops
                let op = ops[0];
                let new_numbers = &numbers[1..];
                let new_ops = &ops[1..];
                let new_acc = match op {
                    Plus => acc + numbers[0],
                    Mult => acc * numbers[0],
                    Operator::Concat => {
                        let mut result = String::new();
                        result.push_str(&acc.to_string());
                        result.push_str(&numbers[0].to_string());
                        result.parse::<usize>().unwrap()
                    }
                };
                do_math_inner(new_numbers, new_ops, new_acc)
            }
        }

        do_math_inner(numbers, ops, 0)
    }

    fn is_solved(&self) -> bool {
        let ops = [Plus, Mult];
        let group_size = self.numbers.len() - 1;
        let combinations = repeat_n(ops.iter(), group_size)
            .multi_cartesian_product()
            .collect_vec();

        debug!(
            "Solving for {:?}. {} {:?} combinations",
            self, group_size, combinations
        );
        for ops in &combinations {
            debug!("Operations: {:?}", ops);
            debug!("   numbers: {:?}", self.numbers);
            let nums = self.numbers.clone();
            let result = Self::do_math(&nums, ops);
            debug!("Test Value: {} Result: {}", self.test_value, result);
            if result == self.test_value {
                return true;
            }
        }
        false
    }

    fn is_solved_part2(&self) -> bool {
        let ops = [Plus, Mult, Concat];
        let group_size = self.numbers.len() - 1;
        let combinations = repeat_n(ops.iter(), group_size)
            .multi_cartesian_product()
            .collect_vec();

        debug!(
            "Solving for {:?}. {} {:?} combinations",
            self, group_size, combinations
        );
        for ops in &combinations {
            debug!("Operations: {:?}", ops);
            debug!("   numbers: {:?}", self.numbers);
            let nums = self.numbers.clone();
            let result = Self::do_math(&nums, ops);
            debug!("Test Value: {} Result: {}", self.test_value, result);
            if result == self.test_value {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::part1;
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;

    #[test]
    fn day7_part_one() -> Result<(), DayError> {
        initialize_logger(None);
        trace!("trace");
        debug!("debug!");
        info!("info!");
        warn!("warn!");
        error!("error!");

        let input = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
"#
        .trim();

        let rows = input
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let result1 = part1(&rows)?;
        assert_eq!(result1, 3749);

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }

    #[test]
    fn day1_part_two() -> Result<(), DayError> {
        initialize_logger(None);

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
