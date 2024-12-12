use code_timing_macros::time_snippet;
pub use shared::prelude::*;
use std::collections::HashSet;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(8, 1, parse_word)?;
    let part1_result = time_snippet!(part1(&result)?);

    println!("Part 1: {}", part1_result);

    Ok(())
}

fn part1(input: &[String]) -> Result<usize, DayError> {
    let map = MappedWorld::new(input);
    let mut antinodes = HashSet::new();
    let mut seen_coords = HashSet::new();
    for i in 0..map.towers.len() {
        let looking = &map.towers[i];
        for j in 0..map.towers.len() {
            let other = &map.towers[j];
            if let Some(antinode) = looking.antinode_of(other) {
                debug!("Found antinode at {:?}", antinode);
                if map.contains(&antinode) {
                    if seen_coords.insert((antinode.x, antinode.y)) {
                        //Insert only if we've not got an antinode at these coords already.
                        antinodes.insert(antinode);
                    }
                }
            }
        }
    }

    debug!("FOUND {} ANTINODES\n{}", antinodes.len(), {
        antinodes
            .iter()
            .map(|x| format!("{:?}", x))
            .collect::<Vec<String>>()
            .join("\n")
    });

    Ok(antinodes.len())
}

fn part2(_input: &[String]) -> Result<usize, DayError> {
    todo!();
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Tower {
    x: usize,
    y: usize,
    freq: char,
}

impl std::fmt::Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:({},{})", self.freq, self.x, self.y)
    }
}

impl Tower {
    fn distance_to(&self, other: &Tower) -> isize {
        let dx = other.x as f64 - self.x as f64;
        let dy = other.y as f64 - self.y as f64;
        //I think this is safe
        let distance = (dx * dx + dy * dy).sqrt().floor() as isize;
        debug!("distance between {} and {} is {}", self, other, distance);
        distance
    }

    fn antinode_of(&self, other: &Tower) -> Option<Tower> {
        if self == other {
            None
        } else if self.freq == other.freq {
            //Create a new tower that is the location of the antinode of self relative to the given tower
            //That's the location of this node in a line double the distance to the other node.
            let dist = self.distance_to(other);
            // lenAB = sqrt(pow(A.x - B.x, 2.0) + pow(A.y - B.y, 2.0));

            let dx = other.x as isize - self.x as isize;
            let dy = other.y as isize - self.y as isize;

            let anti_x = self.x as isize + 2 * dx;
            let anti_y = self.y as isize + 2 * dy;

            if anti_x >= 0 && anti_y >= 0 {
                Some(Tower {
                    x: anti_x as usize,
                    y: anti_y as usize,
                    freq: other.freq,
                })
            } else {
                //antinode is off the edge of the map
                None
            }
        } else {
            //no antinode for different frequencies
            None
        }
    }
}

struct MappedWorld {
    width: usize,
    height: usize,
    towers: Vec<Tower>,
}
impl MappedWorld {
    fn contains(&self, tower: &Tower) -> bool {
        tower.x < self.width && tower.y < self.height
    }
}

//0,0 is top left
impl MappedWorld {
    fn new(input: &[String]) -> Self {
        let height = input.len();
        let width = input[0].len();

        let mut towers = Vec::new();

        let mut y = 0;
        for line in input {
            let chars = line.chars().collect::<Vec<char>>();
            let mut x = 0;
            for c in chars {
                if c != '.' {
                    towers.push(Tower { x, y, freq: c });
                }
                x += 1;
            }
            y += 1;
        }

        Self {
            width,
            height,
            towers,
        }
    }
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::part1;
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

        let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

        let result = part1(&input)?;
        assert_eq!(result, 14);
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
