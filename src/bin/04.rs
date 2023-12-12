use std::collections::HashSet;

advent_of_code::solution!(4);

fn collect_nums(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (_, nums) = line.split_once(':').unwrap();
        let (wins, nums) = nums.split_once('|').unwrap();
        let (wins, nums) = (collect_nums(wins), collect_nums(nums));
        let wins: HashSet<u32> = HashSet::from_iter(wins);
        let mut val = 0;
        for num in nums {
            if wins.contains(&num) {
                if val == 0 {
                    val = 1;
                } else {
                    val <<= 1;
                }
            }
        }
        sum += val;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards = vec![1];
    for (i, line) in input.lines().enumerate() {
        if cards.len() == i {
            cards.push(1);
        }
        let (_, nums) = line.split_once(':').unwrap();
        let (wins, nums) = nums.split_once('|').unwrap();
        let (wins, nums) = (collect_nums(wins), collect_nums(nums));
        let wins: HashSet<u32> = HashSet::from_iter(wins);
        let matches = nums.iter().filter(|num| wins.contains(&num)).count();
        for offset in 0..matches {
            let won_ticket = i + offset + 1;
            if won_ticket == cards.len() {
                cards.push(1);
            }
            cards[won_ticket] += cards[i];
        }
    }
    println!("{:?}", cards);
    Some(cards.iter().sum())
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
