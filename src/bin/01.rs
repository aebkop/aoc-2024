use std::collections::BTreeMap;

advent_of_code::solution!(1);

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|s| s.parse::<u32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip()
}

fn get_occurances(list: Vec<u32>) -> BTreeMap<u32, u32> {
    let mut occurances = BTreeMap::new();
    for num in list {
        *occurances.entry(num).or_insert(0) += 1;
    }
    occurances
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list_1, mut list_2) = parse(input);
    //now sort lists
    list_1.sort_unstable();
    list_2.sort_unstable();
    let total_diff = list_1
        .iter()
        .zip(list_2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(total_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list_1, list_2) = parse(input);
    let occurances_2 = get_occurances(list_2);
    let total_simularity = list_1
        .iter()
        .map(|num| {
            if let Some(occurance) = occurances_2.get(&num) {
                occurance * num
            } else {
                0
            }
        })
        .sum();
    Some(total_simularity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
