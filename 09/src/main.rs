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

    let part2_result = time_snippet!(part2(&result[0])?);
    println!("Part 2: {}", part2_result);
    //6374711498735 is too high
    //6304576012713
    //15616542775140 //well that's WAY too high
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

#[derive(Copy, Clone, Debug)]
struct Chunk {
    start_index: usize,
    id: Option<usize>,
    size: usize,
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

    fn chonk_defrag2(&mut self) {
        fn chunk_disk(disk: &[Block]) -> Vec<Chunk> {
            let mut disk_chunks = Vec::new();
            let mut current_chunk = Chunk {
                start_index: 0,
                id: disk[0].id,
                size: 1,
            };
            for index in 0..disk.len() {
                if index == disk.len() - 1 {
                    current_chunk.size += 1;
                    disk_chunks.push(current_chunk);
                    break;
                }
                //if the current index contains a different set of data than what our current chunk is building up
                //then we need to start a new chunk
                if current_chunk.id != disk[index].id {
                    //new CHUNK
                    disk_chunks.push(current_chunk);
                    current_chunk = Chunk {
                        start_index: index,
                        id: disk[index].id,
                        size: 1,
                    };
                } else {
                    //still in the current chunk
                    current_chunk.size += 1;
                }
            }

            disk_chunks
        }
        //Now I need to collect the empty chunks in order, to use them
        //Do it once for decreasing file id number, there can only be 9 file IDs
        //Way more than 9, I need to do this smartly
        //Find highest Block ID, and use that
        let largest_id_block = self.disk.iter().max_by(|a, b| a.id.cmp(&b.id)).unwrap();
        let largest_id = largest_id_block.id.unwrap();
        warn!("LARGEST ID: {}", largest_id);
        for id in (0..=largest_id).rev() {
            //Regen the disk chunk map every time...
            let disk_chunks = chunk_disk(&self.disk);
            //Make sure our free space indicies are updated every time
            let empty_chunk_indices = disk_chunks
                .iter()
                .enumerate()
                .filter(|(_, x)| x.id.is_none())
                .map(|(i, _)| i)
                .rev()
                .collect::<BTreeSet<usize>>();

            //find the chunk that is the ID
            let file_chunk = disk_chunks
                .iter()
                .enumerate()
                .find(|(_idx, chunk)| chunk.id == Some(id))
                .unwrap();

            let file_chunk_size = file_chunk.1.size;
            let file_chunk_start_index = file_chunk.1.start_index;
            let file_chunk_id = file_chunk.1.id;
            let file_chunk_index = file_chunk.0;

            //Find the left most free space
            let left_most_empty_index = empty_chunk_indices.iter().find(|empty_chunk_index| {
                let chunk = &disk_chunks[**empty_chunk_index];
                chunk.size >= file_chunk_size && chunk.start_index < file_chunk_start_index
            });

            if let Some(empty_index) = left_most_empty_index {
                //now I need to move all bits of that file to this left most empty index
                let empty_chunk_start_index = disk_chunks[*empty_index].start_index;

                //This does the actual disk crunching
                debug!(
                    "MOVING {:?} into {:?}",
                    disk_chunks[file_chunk_index], disk_chunks[*empty_index]
                );

                //Update the actual disk, because we'll rescan it into chunks every time
                for x in 0..file_chunk_size {
                    self.disk[x + empty_chunk_start_index].id = file_chunk_id;
                    self.disk[x + file_chunk_start_index].id = None;
                }

                debug!("DISK: {}", self);
            }
        }
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

fn part2(input: &str) -> Result<usize, DayError> {
    let mut disk = DiskMap::new(input);

    debug!("SAMPLE: 00...111...2...333.44.5555.6666.777.888899");
    debug!("  DISK: {}", disk);
    disk.chonk_defrag2();
    debug!("SAMPLE: 00992111777.44.333....5555.6666.....8888..");
    debug!("  DISK: {}", disk);
    Ok(disk.checksum())
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
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
        let input = "2333133121414131402";

        let result = part2(input)?;
        assert_eq!(result, 2858);

        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
