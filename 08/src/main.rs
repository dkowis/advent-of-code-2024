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

    let part2_result = time_snippet!(part2(&result)?);
    println!("Part 2: {}", part2_result);

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
                if map.contains(&antinode) && seen_coords.insert((antinode.x, antinode.y)) {
                    //Insert only if we've not got an antinode at these coords already.
                    antinodes.insert(antinode);
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

fn part2(input: &[String]) -> Result<usize, DayError> {
    let map = MappedWorld::new(input);
    let mut all_antinodes = HashSet::new();
    let mut seen_coords = HashSet::new();
    for i in 0..map.towers.len() {
        let one = &map.towers[i];
        for j in 0..map.towers.len() {
            let other = &map.towers[j];
            let antinodes = map.find_antinodes(one, other);
            if !antinodes.is_empty() {
                //add all these nodes to the antinodes set
                antinodes.iter().cloned().for_each(|antinode| {
                    if seen_coords.insert((antinode.x, antinode.y)) {
                        //Insert only if we've not got an antinode at these coords already.
                        all_antinodes.insert(antinode);
                    }
                })
            }
        }
    }
    Ok(all_antinodes.len())
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
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

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

impl Tower {
    fn antinode_of(&self, other: &Tower) -> Option<Tower> {
        if self == other {
            None
        } else if self.freq == other.freq {
            //Find the direction vector, and then use it to double the distance.
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

    fn find_antinodes(&self, tower1: &Tower, tower2: &Tower) -> HashSet<Tower> {
        if tower1 == tower2 {
            //No antinodes if I am comparing myself
            HashSet::new()
        } else if tower1.freq != tower2.freq {
            HashSet::new()
        } else {
            let mut antinodes = HashSet::new();
            //find the direction vector
            let dx = tower2.x as isize - tower1.x as isize;
            let dy = tower2.y as isize - tower1.y as isize;

            //Normalize the direction vector., getting us grid points
            let divisor = gcd(dx, dy);
            let step_dx = dx / divisor;
            let step_dy = dy / divisor;

            for dir in [-1, 1] {
                let mut t = 1;
                loop {
                    let x = tower1.x as isize + t * dir * step_dx;
                    let y = tower1.y as isize + t * dir * step_dy;
                    //stop when off the map
                    if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
                        break;
                    }
                    antinodes.insert(Tower {
                        x: x as usize,
                        y: y as usize,
                        freq: tower1.freq,
                    });
                    t += 1;
                }
            }

            antinodes
        }
    }
}

//0,0 is top left
impl MappedWorld {
    fn new(input: &[String]) -> Self {
        let height = input.len();
        let width = input[0].len();

        let mut towers = Vec::new();

        for (y, line) in input.iter().enumerate() {
            let chars = line.chars().collect::<Vec<char>>();
            for (x, c) in chars.iter().enumerate() {
                if *c != '.' {
                    towers.push(Tower { x, y, freq: *c });
                }
            }
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
    use crate::{part1, part2};
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;

    #[test]
    fn day8_part_one() -> Result<(), DayError> {
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
    fn day8_part_two() -> Result<(), DayError> {
        initialize_logger(None);
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

        let result = part2(&input)?;
        assert_eq!(result, 34);
        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
