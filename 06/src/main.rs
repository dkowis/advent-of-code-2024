use code_timing_macros::time_snippet;
pub use shared::prelude::*;
use std::collections::HashSet;
use std::fmt::Formatter;

fn main() -> Result<(), DayError> {
    initialize_logger(Some(Level::WARN));

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(6, 1, parse_word)?;
    let part1_result = time_snippet!(part1(&result)?);
    println!("Part 1: {}", part1_result);

    // let result2 = load_input(0, 2, parse_word)?;
    // let _ = time_snippet!(part2(&result2)?);

    Ok(())
}

#[derive(PartialEq, Debug, Eq, Hash, Copy, Clone)]
struct Coords {
    x: isize,
    y: isize,
}

impl Coords {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}
struct Guard {
    location: Coords,
    facing: Direction,
}

impl Guard {
    fn turn(&mut self) {
        //this is called when an obstical is hit, the guard always turns 90 To the right
        match self.facing {
            Direction::N => self.facing = Direction::E,
            Direction::E => self.facing = Direction::S,
            Direction::S => self.facing = Direction::W,
            Direction::W => self.facing = Direction::N,
        }
    }

    fn peek_forward(&self) -> Coords {
        match self.facing {
            Direction::N => Coords::new(self.location.x, self.location.y - 1),
            Direction::E => Coords::new(self.location.x + 1, self.location.y),
            Direction::S => Coords::new(self.location.x, self.location.y + 1),
            Direction::W => Coords::new(self.location.x - 1, self.location.y),
        }
    }

    fn move_forward(&mut self) {
        self.location = self.peek_forward();
    }
}

struct World {
    visited: HashSet<Coords>,
    crossed_count: usize,
    map: Vec<char>,
    height: usize,
    width: usize,
    guard: Guard,
}

//0,0 is top left
impl World {
    fn new(input: &[String]) -> Self {
        let visited = HashSet::new();
        //load in the entire map, and note where the dude is, and where he's going
        let max_x = input[0].len();
        let max_y = input.len();

        let mut map: Vec<char> = input
            .iter()
            .flat_map(|l| {
                debug!("LINE: {:?}", l);
                l.chars().collect::<Vec<char>>()
            })
            .collect();

        let guard_index = map.iter().position(|x| *x == '^').unwrap();
        let guard = Guard {
            location: Self::static_index_to_coords(max_x, guard_index),
            facing: Direction::N,
        };
        //Replace the index of the guard with a .
        map[guard_index] = '.';

        Self {
            visited,
            map,
            height: max_y,
            width: max_x,
            guard,
            crossed_count: 0,
        }
    }
    fn static_index_of(width: usize, coords: &Coords) -> usize {
        let result = coords.y * width as isize + coords.x;
        if result.is_negative() {
            panic!("Cannot index in the negatives!")
        }
        result as usize
    }
    fn static_index_to_coords(width: usize, index: usize) -> Coords {
        let y = index / width;
        let x = index % width;
        Coords {
            x: x as isize,
            y: y as isize,
        }
    }

    fn index_of(&self, coords: &Coords) -> usize {
        Self::static_index_of(self.width, coords)
    }

    fn index_to_coords(&self, index: usize) -> Coords {
        Self::static_index_to_coords(self.width, index)
    }

    fn char_at(&self, coords: Coords) -> char {
        self.map[self.index_of(&coords)]
    }

    fn patrol(&mut self) {
        //Have the guard walk straight, and record all the coords they travel
        //When the guard would run into something, they turn and walk instead.
        //When the guard leaves the grid, the patrol is over.
        let mut patrolling = true;
        while patrolling {
            debug!("THE WORLD:\n{}", self);
            //we do include the starting position of the guard, so this is correct.
            //And the HashSet will take care of duplicates.
            //Only insert the visited location if the guard is still on the map
            if (self.guard.location.x >= 0
                && self.guard.location.x < self.width as isize
                && self.guard.location.y >= 0
                && self.guard.location.y < self.height as isize)
            {
                //Any time I go to insert this, where I've already crossed should be a spot a loop can happen
                if self.visited.contains(&self.guard.location) {
                    self.crossed_count += 1;
                }
                self.visited.insert(self.guard.location.clone());
            }

            // Peek forward first to see if the guard is going to need to turn instead
            let peek_index = self.index_of(&self.guard.peek_forward());
            if peek_index >= self.map.len() {
                patrolling = false; //we'd walk off the map!
            } else {
                if self.map[peek_index] == '#' {
                    //gotta turn bro! and looping again is smort
                    self.guard.turn();
                } else {
                    self.guard.move_forward();
                }
            }
            //If the guard has moved off the map, we're not patrolling any more
            if self.guard.location.x < 0 || self.guard.location.x > self.width as isize {
                patrolling = false;
            }
            if self.guard.location.y < 0 || self.guard.location.y > self.height as isize {
                patrolling = false;
            }
        }
        debug!("FINAL STATE OF WORLD:\n{}", self);
    }
}

// need to render the map to figure out wtf is going on
impl std::fmt::Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Construct the map with guard and obstacles
        let mut display_map = self.map.clone();

        // Guard's index in the map vector
        let guard_index = Coords::new(self.guard.location.x, self.guard.location.y);
        let guard_pos = World::static_index_of(self.width, &guard_index);

        // Represent the Guard's facing direction with a character
        let guard_char = match self.guard.facing {
            Direction::N => '^',
            Direction::E => '>',
            Direction::S => 'v',
            Direction::W => '<',
        };

        // Place the guard character on the map
        display_map[guard_pos] = guard_char;

        // Display the map
        for y in 0..self.height {
            for x in 0..self.width {
                let index =
                    World::static_index_of(self.width, &Coords::new(x as isize, y as isize));
                if self.visited.contains(&self.index_to_coords(index)) {
                    write!(f, "X")?;
                } else {
                    write!(f, "{}", display_map[index])?;
                }
            }
            writeln!(f)?; // New line after each row
        }

        // Display guard's location and facing
        writeln!(
            f,
            "Guard is at ({}, {}) facing {:?}. Unique Visits: {}, Crossed: {}",
            self.guard.location.x,
            self.guard.location.y,
            self.guard.facing,
            self.visited.len(),
            self.crossed_count
        )
    }
}

fn part1(input: &[String]) -> Result<usize, DayError> {
    let mut world = World::new(input);
    world.patrol();
    let result = world.visited.len();
    Ok(result)
}

fn part2(input: &[String]) -> Result<usize, DayError> {
    let mut world = World::new(input);
    world.patrol();
    let result = world.crossed_count;
    Ok(result)
}

#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2, Coords, World};
    use pretty_assertions::{assert_eq, assert_ne};
    use shared::prelude::*;

    #[test]
    fn day6_part_one_map_tests() -> Result<(), DayError> {
        initialize_logger(None);
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
        .trim();

        let parsed = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        let mut world = World::new(&parsed);
        let coords0 = world.index_to_coords(0);
        assert_eq!(coords0, Coords::new(0, 0));
        let coords1 = world.index_to_coords(1);
        assert_eq!(coords1, Coords::new(1, 0));

        let coords8 = world.index_to_coords(8);
        assert_eq!(coords8, Coords::new(8, 0));
        let coords9 = world.index_to_coords(10);
        assert_eq!(coords9, Coords::new(0, 1));

        let char0 = world.char_at(Coords::new(4, 0));
        assert_eq!(char0, '#');

        let char1 = world.char_at(Coords::new(9, 1));
        assert_eq!(char1, '#');
        Ok(())
    }

    #[test]
    fn day6_part1_corner() -> Result<(), DayError> {
        initialize_logger(None);
        let input = r#"
....#.....
.....#...#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
        .trim();
        let parsed = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        let result = part1(&parsed)?;

        assert_eq!(result, 9);

        Ok(())
    }

    #[test]
    fn day6_part_one() -> Result<(), DayError> {
        initialize_logger(None);
        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
        .trim();

        let parsed = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        let result = part1(&parsed)?;

        assert_eq!(result, 41);
        Ok(())
    }

    #[test]
    fn day6_part_two() -> Result<(), DayError> {
        initialize_logger(None);

        let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#
            .trim();

        let parsed = input.split("\n").map(|x| x.to_string()).collect::<Vec<_>>();
        let result = part2(&parsed)?;

        assert_eq!(result, 6);        Ok(())
    }
}