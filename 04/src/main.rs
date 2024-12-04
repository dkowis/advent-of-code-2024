use crate::Direction::{E, N, NE, NW, S, SE, SW, W};
pub use shared::prelude::*;

fn main() -> Result<(), DayError> {
    Logger::try_with_env_or_str("warn")?.start()?;

    trace!("trace");
    debug!("debug!");
    info!("info!");
    warn!("warn!");
    error!("error!");

    println!("Test some input parsing lol");

    let result = load_input(4, 1, parse_chars)?;
    let part1_result = part1(&result)?;
    println!("Part 1: {}", part1_result);
    
    let part2_result = part2(&result)?;
    println!("Part 2: {}", part2_result);

    Ok(())
}

fn part1(input: &Vec<Vec<char>>) -> Result<usize, DayError> {
    //find all coordinates of X, then do the search algorithm
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'X' {
                let result = find_xmas((x as i32, y as i32), input)?;
                count += result;
            }
        }
    }

    Ok(count)
}
#[derive(Debug)]
enum Direction {
    NW,
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
}

impl Direction {
    fn get_value(&self) -> (i32, i32) {
        match self {
            Direction::NW => (-1, -1),
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
        }
    }
}

fn find_xmas(start_coords: (i32, i32), input: &Vec<Vec<char>>) -> Result<usize, DayError> {
    //look in all the directions 4 characters out, if possible.
    // 0,0 should be top left
    let mut found = 0;

    let find_word = "XMAS";
    found += count_word_direction(input, find_word, start_coords, NW)?;
    found += count_word_direction(input, find_word, start_coords, N)?;
    found += count_word_direction(input, find_word, start_coords, NE)?;
    found += count_word_direction(input, find_word, start_coords, E)?;
    found += count_word_direction(input, find_word, start_coords, SE)?;
    found += count_word_direction(input, find_word, start_coords, S)?;
    found += count_word_direction(input, find_word, start_coords, SW)?;
    found += count_word_direction(input, find_word, start_coords, W)?;

    Ok(found)
}

fn count_word_direction(
    input: &Vec<Vec<char>>,
    word: &str,
    start: (i32, i32),
    direction: Direction,
) -> Result<usize, DayError> {
    //direction is like -1, -1 or 1,1 or 1,0, 0,1 it's one of the 8 directions around a point
    //do a range check around start in the given direction
    let (x, y) = start;
    let (dx, dy) = direction.get_value();

    let max_x = input.len() - 1;
    let max_y = input[0].len() - 1;

    let wordlen = word.len();
    let mut found_word: Vec<char> = Vec::with_capacity(wordlen);
    for i in 0..wordlen {
        let coord: (i32, i32) = (x + dx * i as i32, y + dy * i as i32);
        if coord.0 < 0 || coord.1 < 0 {
            //too small, no word in this direction
            return Ok(0);
        }
        if coord.0 > max_x as i32 || coord.1 > max_y as i32 {
            //too large, no word in this direction
            return Ok(0);
        }
        let fixed_coord = (coord.0 as u32, coord.1 as u32);
        //I should just build up the found word here...

        found_word.push(input[fixed_coord.1 as usize][fixed_coord.0 as usize]);
    }

    let found_word_str = String::from_iter(found_word.iter());
    if found_word_str == word {
        Ok(1)
    } else {
        Ok(0)
    }
}

fn part2(input: &Vec<Vec<char>>) -> Result<usize, DayError> {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if input[y][x] == 'A' {
                let result = find_x_mas((x as i32, y as i32), input)?;
                if result {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn find_x_mas(start_coords: (i32, i32), input: &Vec<Vec<char>>) -> Result<bool, DayError> {
    //look in all the directions 4 characters out, if possible.
    //So now I need to find all the A's with an M and S in diagonals
    
    // 0,0 should be top left
    let (x, y) = start_coords;
    if x == 0 || y == 0 || x == input.len() as i32 - 1 || y == input[0].len() as i32 - 1 {
        //there cannot be any X-MAS on any of the edges
        return Ok(false)
    }
    //Find our maxes
    //surrounding the current coordinates at diagonals should be MAS or SAM
    let top_left = (x - 1, y - 1);
    let top_right = (x + 1, y - 1);
    let bottom_left = (x - 1, y + 1);
    let bottom_right = (x + 1, y + 1);
    
    let top_left_char = input[top_left.1 as usize][top_left.0 as usize];
    let top_right_char = input[top_right.1 as usize][top_right.0 as usize];
    let bottom_left_char = input[bottom_left.1 as usize][bottom_left.0 as usize];
    let bottom_right_char = input[bottom_right.1 as usize][bottom_right.0 as usize];
    
    match (top_left_char, top_right_char, bottom_left_char, bottom_right_char) {
        ('M','M','S','S') => Ok(true),
        ('S','S','M','M') => Ok(true),
        ('S','M','S','M') => Ok(true),
        ('M','S','M','S') => Ok(true),
        _ => Ok(false)
    }
    
}


#[cfg(test)]
mod test {
    extern crate indoc;
    use crate::{part1, part2};
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
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#
        .trim();

        let parsed: Vec<Vec<char>> = input
            .split("\n")
            .map(|x| x.trim().chars().collect())
            .collect();

        let result = part1(&parsed)?;
        assert_eq!(result, 18);
        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }

    #[test]
    fn day1_part_two() -> Result<(), DayError> {
        initialize();
        
        let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#
            .trim();

        let parsed: Vec<Vec<char>> = input
            .split("\n")
            .map(|x| x.trim().chars().collect())
            .collect();

        let result = part2(&parsed)?;
        assert_eq!(result, 9);
        assert_eq!(1, 1);
        assert_ne!(1, 2);
        Ok(())
    }
}
