use code_timing_macros::time_snippet;
pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    initialize_logger(None);

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(0, 1, parse_word)?;
    let _ = time_snippet!(part1(&result)?);

    let result2 = load_input(0, 2, parse_word)?;
    let _ = time_snippet!(part2(&result2)?);

    Ok(())
}

fn part1(_input: &[String]) -> Result<usize, DayError> {
    todo!();
}

fn part2(_input: &[String]) -> Result<usize, DayError> {
    todo!();
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;

    #[test]
    fn day1_part_one() -> Result<(), DayError> {
        initialize_logger(None);
        trace!("trace");
        debug!("debug!");
        info!("info!");
        warn!("warn!");
        error!("error!");

        println!("Test some input parsing lol");

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
