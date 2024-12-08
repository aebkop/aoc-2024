advent_of_code::solution!(4);
// Array format: [direction][step][coordinate]
const DIRECTIONS: [[[i32; 2]; 3]; 8] = [
    // up left
    [[-1, -1], [-2, -2], [-3, -3]],
    // up
    [[-1, 0], [-2, 0], [-3, 0]],
    // up right
    [[-1, 1], [-2, 2], [-3, 3]],
    // left
    [[0, -1], [0, -2], [0, -3]],
    // right
    [[0, 1], [0, 2], [0, 3]],
    // down left
    [[1, -1], [2, -2], [3, -3]],
    // down
    [[1, 0], [2, 0], [3, 0]],
    // down right
    [[1, 1], [2, 2], [3, 3]],
];

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let vec_chars = parse(input);
    let mut count = 0;
    let chars_to_look_for: [char; 3] = ['M', 'A', 'S'];
    for (y, outer) in vec_chars.iter().enumerate() {
        for (x, inner) in outer.iter().enumerate() {
            if *inner == 'X' {
                //Now look in all directions
                'outer: for direction_arrs in DIRECTIONS.iter() {
                    for (idx, direction) in direction_arrs.iter().enumerate() {
                        let new_x: usize = match (x as i32).checked_add(direction[0]) {
                            Some(val) if val >= 0 => val as usize,
                            _ => continue 'outer,
                        };
                        let new_y = match (y as i32).checked_add(direction[1]) {
                            Some(val) if val >= 0 => val as usize,
                            _ => continue 'outer,
                        };
                        match vec_chars.get(new_y).and_then(|row| row.get(new_x)) {
                            Some(c) => {
                                if *c != chars_to_look_for[idx] {
                                    continue 'outer;
                                }
                            }
                            None => continue 'outer,
                        }
                    }
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

pub fn check_directions(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    //Check to see if there are diagonals to look for
    if x == 0 || y == 0 || x >= input[0].len() - 1 || y >= input.len() - 1 {
        return false;
    }

    let diags = (
        (input[y - 1][x - 1], input[y + 1][x + 1]), // top-left to bottom-right diagonal
        (input[y - 1][x + 1], input[y + 1][x - 1]), // top-right to bottom-left diagonal
    );

    if check_diagonal(diags.0) && check_diagonal(diags.1) {
        return true;
    }

    return false;
}

pub fn check_diagonal(diagonal: (char, char)) -> bool {
    diagonal == ('M', 'S') || diagonal == ('S', 'M')
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec_chars = parse(input);
    let mut count = 0;

    for (y, outer) in vec_chars.iter().enumerate() {
        for (x, inner) in outer.iter().enumerate() {
            if *inner == 'A' {
                if check_directions(&vec_chars, x, y) {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u32> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
