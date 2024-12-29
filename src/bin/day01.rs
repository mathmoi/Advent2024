use std::{collections::HashMap, fs};

fn part1(input: &str) -> u32 {
    let (mut first_list, mut second_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            match (nums.next(), nums.next()) {
                (Some(a), Some(b)) => match (a.parse::<u32>(), b.parse::<u32>()) {
                    (Ok(num_a), Ok(num_b)) => Some((num_a, num_b)),
                    _ => None,
                },
                _ => None,
            }
        })
        .unzip();

    first_list.sort();
    second_list.sort();

    first_list
        .iter()
        .zip(second_list.iter())
        .map(|(a, b)| (*a as i32 - *b as i32).abs() as u32)
        .sum()
}

fn part2(input: &str) -> u32 {
    let (first_list, second_list): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| {
            let mut nums = line.split_whitespace();
            match (nums.next(), nums.next()) {
                (Some(a), Some(b)) => match (a.parse::<u32>(), b.parse::<u32>()) {
                    (Ok(num_a), Ok(num_b)) => Some((num_a, num_b)),
                    _ => None,
                },
                _ => None,
            }
        })
        .unzip();

    let mut counter = HashMap::new();
    for num in second_list {
        *counter.entry(num).or_insert(0) += 1;
    }

    first_list
        .iter()
        .map(|num| num * counter.get(&num).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input/day01.txt")
        .expect("Should have been able to read the input file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 31);
    }
}
