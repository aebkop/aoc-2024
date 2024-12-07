advent_of_code::solution!(2);
use itertools::Itertools;

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn safe_input(input: &Vec<i32>) -> bool {
    let mut diff_signum_opt: Option<i32> = None;
    for (a, b) in input.iter().tuple_windows() {
        let diff = a - b;
        let current_diff_signum = diff.signum();
        if let Some(diff_signum) = diff_signum_opt {
            if diff_signum != current_diff_signum {
                return false;
            }
        } else {
            if current_diff_signum == 0 {
                return false;
            }
            diff_signum_opt = Some(current_diff_signum);
        }
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }
    return true;
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input);
    Some(input.iter().filter(|f| safe_input(f)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = parse(input);
    Some(
        input
            .iter()
            .filter(|f| {
                let res = safe_input(f);
                if !res {
                    for index in 0..f.len() {
                        let mut new_vec = (*f).clone();
                        new_vec.remove(index);
                        if safe_input(&new_vec) {
                            return true;
                        }
                    }
                    return false;
                }
                return true;
            })
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
