advent_of_code::solution!(2);

#[derive(Debug)]
struct Round {
    r: u32,
    g: u32,
    b: u32,
}

fn process_round(input: &str) -> Round {
    let mut round = Round { r: 0, g: 0, b: 0 };
    for cube in input.split(", ") {
        let (amount, color) = cube.trim().split_once(' ').unwrap();
        if let Ok(amount) = amount.parse::<u32>() {
            match color {
                "red" => round.r += amount,
                "green" => round.g += amount,
                "blue" => round.b += amount,
                _ => {}
            }
        }
    }
    round
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (id, game) = line.split_once(':').unwrap();

        let mut game = game.split(';').map(process_round);

        if game.all(|r| r.r <= 12 && r.g <= 13 && r.b <= 14) {
            println!("Game {:#?} is OK", game);
            let id: u32 = id.strip_prefix("Game ").unwrap().parse().unwrap();
            sum += id;
        } else {
            println!("Game {:#?} is INVALID", game);
        }
    }
    Some(sum)
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let (_, game) = line.split_once(':').unwrap();
        let game = game.split(';').map(process_round);

        let mut max_r = Round { r: 0, g: 0, b: 0 };
        for r in game {
            max_r.r = max(max_r.r, r.r);
            max_r.g = max(max_r.g, r.g);
            max_r.b = max(max_r.b, r.b);
        }
        sum += max_r.r * max_r.g * max_r.b;
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
