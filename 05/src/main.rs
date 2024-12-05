use code_timing_macros::time_snippet;
pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    Logger::try_with_env_or_str("warn")?.start()?;

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(5, 1, parse_word)?;

    let instructions = Instructions::new(&result);
    let part1_result = time_snippet!(part1(&instructions)?);

    println!("PART 1: {}", part1_result);

    let part2_result = time_snippet!(part2(&instructions)?);
    println!("PART 2: {}", part2_result);

    Ok(())
}

struct Rule {
    before: usize,
    after: usize,
}

struct Update {
    order: Vec<usize>,
}

struct Instructions {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

impl Instructions {
    fn new(input: &[String]) -> Instructions {
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        let mut rules_processing = true;
        for line in input {
            if line == "" {
                rules_processing = false;
            } else {
                if rules_processing {
                    let parts = line.split("|").collect::<Vec<_>>();
                    let before = parts[0].split(",").collect::<Vec<_>>();
                    let after = parts[1].split(",").collect::<Vec<_>>();
                    rules.push(Rule {
                        before: before[0].parse().unwrap(),
                        after: after[0].parse().unwrap(),
                    });
                } else {
                    let order = line
                        .split(",")
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<usize>>();
                    updates.push(Update { order });
                }
            }
        }

        Instructions { rules, updates }
    }

    fn update_passes(&self, update: &Update) -> bool {
        //maybe instead of looking at each rule, I should look at each page and see if it passes rules that apply to it
        let mut passing = true;
        'outer: for i in 0..update.order.len() {
            let item = update.order[i];
            for rule in self.rules.iter() {
                //if it doesn't apply, it passes
                if rule.before == item || rule.after == item {
                    //test that the rule works.
                    let before = update.order.iter().position(|x| *x == rule.before);
                    let after = update.order.iter().position(|x| *x == rule.after);
                    match (before, after) {
                        (Some(before_index), Some(after_index)) => {
                            if before_index > i || after_index < i {
                                passing = false;
                                break 'outer; //Short circuit the whole thing!
                            }
                        }
                        _ => {
                            //Rule doesn't apply, if it can't find both numbers.
                        }
                    }
                }
            }
        }

        passing
    }

    fn passing_updates(&self) -> Vec<Vec<usize>> {
        //for each update, check it against every rule, if the numbers don't exist in the rule, the rule doesn't apply
        let mut passing: Vec<Vec<usize>> = Vec::new();
        for update in self.updates.iter() {
            if self.update_passes(update) {
                passing.push(update.order.clone())
            }
        }

        passing
    }

    fn fix_update(&self, update: &Update) -> Update {
        //maybe instead of looking at each rule, I should look at each page and see if it passes rules that apply to it
        let mut passing = false;
        let mut data = update.order.clone();
        while !passing {
            //I think we can do this lamely, reorder it and loop until it's fixed.
            'outer: for i in 0..data.len() {
                let item = data[i];
                for rule in self.rules.iter() {
                    //if it doesn't apply, it passes
                    //lets assume we're gonna pass, until we find a rule that doesn't work
                    passing = true;
                    if rule.before == item || rule.after == item {
                        //test that the rule works.
                        let before = data.iter().position(|x| *x == rule.before);
                        let after = data.iter().position(|x| *x == rule.after);
                        match (before, after) {
                            (Some(before_index), Some(after_index)) => {
                                if before_index > i || after_index < i {
                                    passing = false; //this will make sure we run through again
                                                     //fix it instead I think this will work
                                    data.swap(i, before_index);
                                    break 'outer;
                                }
                            }
                            _ => {
                                //Rule doesn't apply, if it can't find both numbers.
                            }
                        }
                    }
                }
            }
        }
        debug!("BEFORE: {:?}", update.order);
        debug!(" FIXED: {:?}", data);

        Update { order: data }
    }

    fn corrected_updates(&self) -> Vec<Update> {
        let mut failing: Vec<&Update> = Vec::new();
        for update in self.updates.iter() {
            if !self.update_passes(update) {
                failing.push(update)
            }
        }

        let fixed: Vec<Update> = failing.iter().map(|f| self.fix_update(f)).collect();
        fixed
    }
}

fn part1(instructions: &Instructions) -> Result<usize, DayError> {
    let passing = instructions.passing_updates();

    let result = passing
        .iter()
        .map(|x| {
            //get the middle element
            let middle = x.len() / 2;
            x[middle]
        })
        .sum();

    Ok(result)
}

fn part2(instructions: &Instructions) -> Result<usize, DayError> {
    let fixed = instructions.corrected_updates();

    let result = fixed
        .iter()
        .map(|x| {
            //get the middle element
            let middle = x.order.len() / 2;
            x.order[middle]
        })
        .sum();

    Ok(result)
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2, Instructions};
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
    fn day5_part_one() -> Result<(), DayError> {
        initialize();
        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        let instructions = Instructions::new(&input);
        let result = part1(&instructions)?;
        assert_eq!(result, 143);

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }

    #[test]
    fn day5_part_two() -> Result<(), DayError> {
        initialize();

        let input = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        let instructions = Instructions::new(&input);
        let result = part2(&instructions)?;
        assert_eq!(result, 123);

        Ok(())
    }
}
