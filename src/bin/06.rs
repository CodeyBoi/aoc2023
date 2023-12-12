advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().unwrap())
        })
        .collect::<Vec<_>>();
    let (times, dists) = (input[0].clone(), input[1].clone());
    let sum = times.zip(dists).fold(1, |acc, (time, dist)| {
        acc * (1..time).filter(|hold| hold * (time - hold) > dist).count()
    });
    Some(sum as u32)
}

fn collect_num(input: &str) -> u128 {
    input
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap())
        .fold(0, |acc, n| acc * 10_u128.pow(n.ilog10() + 1) + n)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input.split_once('\n').unwrap();
    let (time, dist) = (collect_num(input.0), collect_num(input.1));
    let sum = (1..time).filter(|hold| hold * (time - hold) > dist).count();
    Some(sum as u32)
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
