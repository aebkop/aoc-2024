advent_of_code::solution!(3);
use regex::Regex;

pub enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

pub fn parse_part_2(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    regex
        .captures_iter(input)
        .filter_map(|f| {
            if let Some(first) = f.get(1) {
                let first = first.as_str().parse::<u32>().ok()?;
                let second = f[2].parse::<u32>().ok()?;
                Some(Instruction::Mul(first, second))
            } else {
                match &f[0] {
                    "do()" => Some(Instruction::Do),
                    "don't()" => Some(Instruction::Dont),
                    _ => None,
                }
            }
        })
        .collect()
}

pub fn parse_part_1(input: &str) -> Vec<(u32, u32)> {
    let mul_regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul_regex
        .captures_iter(input)
        .filter_map(|f| {
            let first = f[1].parse::<u32>().ok()?;
            let second = f[2].parse::<u32>().ok()?;
            Some((first, second))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse_part_1(input);
    Some(input.iter().map(|f| f.0 * f.1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input: Vec<Instruction> = parse_part_2(input);
    let mut is_active = true;
    let mut total: u32 = 0;
    for instruction in input {
        match instruction {
            Instruction::Mul(a, b) => {
                if is_active {
                    total += a * b
                }
            }
            Instruction::Do => {
                is_active = true;
            }
            Instruction::Dont => {
                is_active = false;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u32> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
