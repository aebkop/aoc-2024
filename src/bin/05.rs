use std::{cmp::Ordering, collections::HashMap, ops::Not};

advent_of_code::solution!(5);

pub fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (section_1, section_2) = input.split_once("\n\n").unwrap();
    let map: HashMap<u32, Vec<u32>> = section_1.lines().fold(HashMap::new(), |mut acc, line| {
        let (page_1, page_2) = line
            .split_once('|')
            .map(|a| (a.0.parse::<u32>().unwrap(), a.1.parse::<u32>().unwrap()))
            .unwrap();
        acc.entry(page_1)
            .and_modify(|f| f.push(page_2))
            .or_insert(vec![page_2]);
        acc
    });
    let section_2_parsed: Vec<Vec<u32>> = section_2
        .lines()
        .map(|x| {
            x.split(',')
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    (map, section_2_parsed)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, input) = parse(input);
    Some(
        input
            .iter()
            .filter_map(|pages| {
                pages
                    .is_sorted_by(|a, b| rules.get(a).is_some_and(|rules| rules.contains(b)))
                    .then(|| pages[pages.len() / 2])
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, mut input) = parse(input);
    Some(
        input
            .iter_mut()
            .filter(|pages| {
                pages
                    .is_sorted_by(|a, b| rules.get(a).is_some_and(|rules| rules.contains(b)))
                    .not()
            })
            .map(|pages| {
                pages.sort_by(|a, b| {
                    if rules.get(a).is_some_and(|pages| pages.contains(b)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });
                pages[pages.len() / 2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
