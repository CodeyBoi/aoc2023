advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (mut left, mut right) = (None, None);
        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                (left, right) = match (left, right) {
                    (None, None) => (Some(digit), Some(digit)),
                    (Some(a), Some(_)) => (Some(a), Some(digit)),
                    _ => (left, right),
                }
            }
        }
        if let (Some(a), Some(b)) = (left, right) {
            sum += a * 10 + b;
        }
    }
    Some(sum)
}

const DIGITS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn match_digit(input: &str) -> Option<u32> {
    if let Some(digit) = input.chars().next().unwrap().to_digit(10) {
        return Some(digit);
    }
    for (i, digit) in DIGITS.iter().enumerate() {
        if input.starts_with(digit) {
            return Some(i as u32);
        }
    }
    None
}

fn next_digit(input: &str) -> (Option<u32>, usize) {
    for i in 0..input.len() {
        if let Some(val) = match_digit(&input[i..]) {
            return (Some(val), i + 1);
        }
    }
    (None, 0)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (mut left, mut right) = (None, None);
        let (mut cur, mut index) = next_digit(line);
        while let Some(digit) = cur {
            (left, right) = match (left, right) {
                (None, None) => (Some(digit), Some(digit)),
                (Some(a), Some(_)) => (Some(a), Some(digit)),
                _ => (left, right),
            };
            let new_index;
            (cur, new_index) = next_digit(&line[index..]);
            if cur.is_none() {
                break;
            }
            index += new_index;
        }
        if let (Some(a), Some(b)) = (left, right) {
            sum += a * 10 + b;
        }
    }
    Some(sum)
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
