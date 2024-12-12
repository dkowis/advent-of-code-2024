use code_timing_macros::time_snippet;
pub use shared::prelude::*;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Formatter;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    let result = load_input(9, 1, parse_word)?;
    let part1_result = time_snippet!(part1(&result[0])?);
    println!("Part 1: {}", part1_result);

    // let result2 = load_input(0, 2, parse_word)?;
    // let _ = time_snippet!(part2(&result2)?);

    Ok(())
}
#[derive(Eq, PartialEq)]
struct Block {
    id: Option<usize>,
}
impl Block {
    fn new(id: usize) -> Self {
        Self { id: Some(id) }
    }
    fn empty() -> Self {
        Self { id: None }
    }
}

struct DiskMap {
    disk: Vec<Block>,
}

impl std::fmt::Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //write out the disk block by block where empty is . and if it has some ID it's the number
        let disk_string = self
            .disk
            .iter()
            .map(|b| match b.id {
                Some(id) => id.to_string(),
                None => ".".to_string(),
            })
            .collect::<Vec<String>>()
            .join("");
        f.write_str(&disk_string)
    }
}

impl DiskMap {
    fn new(disk_string: &str) -> Self {
        let things: Vec<u32> = disk_string
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect();
        let mut file_toggle = true;
        let mut disk = Vec::new();
        let mut id = 0;
        for thing in things {
            if file_toggle {
                debug!("FILE ID: {} length of {}", id, thing);
                (0..thing).for_each(|_| disk.push(Block::new(id as usize)));
                id += 1;
                file_toggle = false;
            } else {
                debug!("EMPTY length of {}", thing);
                (0..thing).for_each(|_| disk.push(Block::empty()));
                file_toggle = true;
            }
        }

        Self { disk }
    }

    fn defrag(&mut self) {
        //find all empty spots, and then peel off the end of the thing into those empty spots
        let mut empty_block_indices = self
            .disk
            .iter()
            .enumerate()
            .filter(|(_, b)| b.id.is_none())
            .map(|(i, _)| i)
            .rev()
            .collect::<BTreeSet<usize>>();

        //now we crawl backwards through the disk vector, and move things from the end into the first empty slots
        for index in (0..self.disk.len()).rev() {
            if *empty_block_indices.first().unwrap() > index {
                //No more empty ahead of my index
                break;
            }
            if let Some(id) = &self.disk[index].id {
                let first_empty = empty_block_indices.pop_first().unwrap();
                // debug!("Moving {} into {}", id, first_empty);
                self.disk[first_empty].id = Some(*id);
                self.disk[index].id = None;
                //Also inject the now empty space.
                empty_block_indices.insert(index);
                debug!("DISK: {}", self);
            }
        }
    }

    fn chonk_defrag(&mut self) {
        //Now we need to move contiguous files. Attempt to move it once, if it doesn't fit, don't move it
    }

    fn checksum(&self) -> usize {
        self.disk
            .iter()
            .enumerate()
            .map(|(i, b)| if let Some(id) = b.id { id * i } else { 0 })
            .sum()
    }
}

fn part1(input: &str) -> Result<usize, DayError> {
    let mut disk = DiskMap::new(input);

    debug!("SAMPLE: 00...111...2...333.44.5555.6666.777.888899");
    debug!("  DISK: {}", disk);
    disk.defrag();
    debug!("SAMPLE: 0099811188827773336446555566..............");
    debug!("  DISK: {}", disk);
    Ok(disk.checksum())
}

fn part2(_input: &[String]) -> Result<usize, DayError> {
    todo!();
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::part1;
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;

    #[test]
    fn day9_part_one() -> Result<(), DayError> {
        initialize_logger(None);
        trace!("trace");
        debug!("debug!");
        info!("info!");
        warn!("warn!");
        error!("error!");

        let input = "2333133121414131402";

        let result = part1(input)?;
        assert_eq!(result, 1928);

        println!("Test some input parsing lol");

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }

    #[test]
    fn day9_part_two() -> Result<(), DayError> {
        initialize_logger(None);

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
