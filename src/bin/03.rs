advent_of_code::solution!(3);

enum Cell {
    Empty,
    Digit(u8),
    Symbol,
}

fn collect_num(grid: &[Vec<Cell>], row: usize, col: usize) -> Option<u32> {
    let cell = grid.get(row)?.get(col)?;
    match cell {
        Cell::Digit(d) => match grid[row].get(col.wrapping_sub(1)) {
            Some(Cell::Digit(_)) => None,
            _ => {
                let mut num: u32 = *d as u32;
                for i in 1.. {
                    if let Some(Cell::Digit(d)) = grid[row].get(col + i) {
                        num = num * 10 + *d as u32;
                    } else {
                        break;
                    }
                }
                Some(num)
            }
        },

        _ => None,
    }
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn has_nearby_symbol(grid: &[Vec<Cell>], row: usize, col: usize) -> bool {
    for (dr, dc) in OFFSETS {
        let (r, c) = (row.wrapping_add_signed(dr), col.wrapping_add_signed(dc));
        if let Some(row_data) = grid.get(r as usize) {
            if let Some(Cell::Symbol) = row_data.get(c as usize) {
                return true;
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    d if c.is_digit(10) => Cell::Digit(d.to_digit(10).unwrap() as u8),
                    _ => Cell::Symbol,
                })
                .collect()
        })
        .collect();

    for r in 0..grid.len() {
        'cell: for c in 0..grid[r].len() {
            if let Some(num) = collect_num(&grid[..], r, c) {
                for i in 0..num.ilog10() + 1 {
                    if has_nearby_symbol(&grid[..], r, c + i as usize) {
                        sum += num;
                        continue 'cell;
                    }
                }
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
