use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

#[derive(Debug, Clone)]
pub enum Operator {
    Mulitply,
    Add,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Mulitply => a * b,
            Operator::Add => a + b,
            Operator::Concatenate => (a.to_string() + &b.to_string()).parse().unwrap(),
        }
    }
}

pub fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let line_split = line.split_once(':').unwrap();
            let res_number: u64 = line_split.0.parse().unwrap();
            let other_numbers: Vec<u64> = line_split
                .1
                .trim()
                .split_whitespace()
                .map(|digit| digit.parse().unwrap())
                .collect();
            (res_number, other_numbers)
        })
        .collect()
}

const OPERATORS: [Operator; 2] = [Operator::Mulitply, Operator::Add];
const OPERATORS_WITH_CONCAT: [Operator; 3] =
    [Operator::Mulitply, Operator::Add, Operator::Concatenate];

pub fn solve<const N: usize>(input: Vec<(u64, Vec<u64>)>, operators: &[Operator; N]) -> u64 {
    input
        .par_iter()
        .filter_map(|(total, numbers)| {
            (0..numbers.len() - 1)
                .map(|_| operators)
                .multi_cartesian_product()
                .any(|f| {
                    *total
                        == f.iter()
                            .zip(numbers.iter().skip(1))
                            .fold(numbers[0], |acc, (op, num)| op.apply(acc, *num))
                })
                .then(|| total)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Vec<(u64, Vec<u64>)> = parse(input);
    Some(solve(input, &OPERATORS))
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse(input);
    Some(solve(input, &OPERATORS_WITH_CONCAT))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
