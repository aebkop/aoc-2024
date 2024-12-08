use std::collections::HashSet;

use ahash::AHashSet;

advent_of_code::solution!(6);
#[derive(Debug, Clone)]
pub enum Tile {
    //Bool represents if it has been visited
    Empty(bool),
    Guard,
    Obstruction,
}
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

type Position = (usize, usize);

#[derive(Clone)]
pub struct Guard {
    pos: Position,
    direction: Direction,
}

pub fn parse(input: &str) -> ((usize, usize), Vec<Vec<Tile>>) {
    let mut guard_pos: (usize, usize) = (0, 0);
    let tile_vec = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    '.' => Tile::Empty(false),
                    '#' => Tile::Obstruction,
                    '^' => {
                        guard_pos = (x, y);
                        Tile::Empty(true)
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (guard_pos, tile_vec)
}

pub fn new_guard_pos(guard: &Guard) -> Option<Position> {
    match guard.direction {
        Direction::North => guard.pos.1.checked_sub(1).map(|new_y| (guard.pos.0, new_y)),
        Direction::South => Some((guard.pos.0, guard.pos.1 + 1)),
        Direction::East => Some((guard.pos.0 + 1, guard.pos.1)),
        Direction::West => guard.pos.0.checked_sub(1).map(|new_x| (new_x, guard.pos.1)),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (guard_pos, mut grid) = parse(input);
    let mut guard = Guard {
        pos: guard_pos,
        direction: Direction::North,
    };
    let x_bound = grid[0].len();
    let y_bound = grid.len();
    loop {
        let pos_to_test = new_guard_pos(&guard);
        //check for bounds
        if let Some(pos_to_test) = pos_to_test {
            if pos_to_test.0 < x_bound && pos_to_test.1 < y_bound {
                //get grid and check for obstacle
                let char = &grid[pos_to_test.1][pos_to_test.0];
                match char {
                    Tile::Empty(_) => {
                        guard.pos = pos_to_test;
                        grid[pos_to_test.1][pos_to_test.0] = Tile::Empty(true); // This actually updates the grid
                    }
                    Tile::Obstruction => {
                        //Update guard direction
                        guard.direction = match guard.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        };
                    }
                    _ => {
                        println!("We shouldn't be here :(, tile: {:?}", char);
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    //Now count all Tile::Empty(true)
    Some(
        grid.iter()
            .flatten()
            .filter(|tile| matches!(tile, Tile::Empty(true)))
            .count() as u32,
    )
}

pub fn simulate(grid: &mut Vec<Vec<Tile>>, guard: &mut Guard) -> bool {
    let x_bound = grid[0].len();
    let y_bound = grid.len();
    let mut visited = AHashSet::new();
    loop {
        let pos_to_test = new_guard_pos(&guard);
        //check for bounds
        if let Some(pos_to_test) = pos_to_test {
            if pos_to_test.0 < x_bound && pos_to_test.1 < y_bound {
                if visited.contains(&(pos_to_test, guard.direction)) {
                    return true;
                } else {
                    visited.insert((pos_to_test, guard.direction));
                }
                //get grid and check for obstacle
                let char = &grid[pos_to_test.1][pos_to_test.0];
                match char {
                    Tile::Empty(_) => {
                        guard.pos = pos_to_test;
                        grid[pos_to_test.1][pos_to_test.0] = Tile::Empty(true); // This actually updates the grid
                    }
                    Tile::Obstruction => {
                        //Update guard direction
                        guard.direction = match guard.direction {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North,
                        };
                    }
                    _ => {
                        println!("We shouldn't be here :(, tile: {:?}", char);
                    }
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    //Now count all Tile::Empty(true)
    return false;
}

//Super ugly brute force approach :(
pub fn part_two(input: &str) -> Option<u32> {
    let (guard_pos, grid) = parse(input);
    let guard = Guard {
        pos: guard_pos,
        direction: Direction::North,
    };
    let indicies: Vec<(usize, usize)> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, tile)| match tile {
                    Tile::Empty(false) => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    //Go through indicies and simulate if they were a obstruction
    let mut counter = 0;
    for (x, y) in indicies {
        let mut new_grid = grid.clone();
        let mut new_guard = guard.clone();
        new_grid[y][x] = Tile::Obstruction;
        if simulate(&mut new_grid, &mut new_guard) {
            counter += 1;
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
