pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    Logger::try_with_env_or_str("warn")?.start()?;

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(0, 1, parse_word)?;
    for x in result {
        println!("A LINE: {}", x);
        let _ = part1();
    }

    let result2 = load_input(0, 2, parse_i32)?;
    for x in result2 {
        println!("ITS A NUMBER: {}", x);
        let _ = part2();
    }

    Ok(())
}

fn part1() -> Result<(), DayError> {
    todo!();
}

fn part2() -> Result<(), DayError> {
    todo!();
}

#[cfg(test)]
mod test {
    extern crate indoc;
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
    fn day1_part_one() -> Result<(), DayError> {
        initialize();
        assert_eq!(1, 1);
        assert_ne!(1, 2);
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
