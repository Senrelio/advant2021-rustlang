use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day11_input");
    println!("day11 part1: {}", part1(input));
    println!("day11 part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut matrix = Matrix::new(input);
    for _ in 0..100 {
        let _ = matrix.next_day();
    }
    matrix.flash_count
}
fn part2(input: &str) -> u32 {
    let mut matrix = Matrix::new(input);
    let mut day = 0;
    for i in 0.. {
        let flash_today = matrix.next_day();
        if flash_today == 100 {
            day = i + 1;
            break;
        }
    }
    day
}

struct Matrix {
    m: [[u8; 10]; 10],
    flash_count: u32,
}

impl Matrix {
    fn new(input: &str) -> Matrix {
        let mut m = [[0; 10]; 10];
        for (r, line) in input.lines().enumerate() {
            for (c, n) in line.trim().chars().map(|c| c as u8 - 48).enumerate() {
                m[r][c] = n;
            }
        }
        Matrix { m, flash_count: 0 }
    }
    fn next_day(&mut self) -> u32 {
        let mut to_flash = HashSet::with_capacity(100);
        let mut adjancant = vec![];
        for r in 0..10 {
            for c in 0..10 {
                self.m[r][c] += 1;
                if self.m[r][c] == 10 {
                    to_flash.insert((r, c));
                    adjancant.extend(surrounding(r, c));
                }
            }
        }
        // every loop, all adjancant plus one and recount adjancant
        // loop until there is no adjancant

        loop {
            if adjancant.is_empty() {
                break;
            }
            let mut temp = vec![];
            for &(r, c) in &adjancant {
                self.m[r][c] += 1;
                if self.m[r][c] == 10 {
                    to_flash.insert((r, c));
                    temp.extend(surrounding(r, c));
                }
            }
            adjancant = temp;
        }
        // before next day, set all point over 9 to 0;
        // count flashed
        let mut flash_this_day = 0;
        for row in self.m.iter_mut() {
            for n in row.iter_mut() {
                if *n >= 10 {
                    flash_this_day += 1;
                    *n = 0;
                }
            }
        }
        self.flash_count += flash_this_day;
        flash_this_day
    }
}

fn surrounding(r: usize, c: usize) -> Vec<(usize, usize)> {
    let r = r as i8;
    let c = c as i8;
    [
        (r - 1, c - 1),
        (r - 1, c),
        (r - 1, c + 1),
        (r, c - 1),
        (r, c + 1),
        (r + 1, c - 1),
        (r + 1, c),
        (r + 1, c + 1),
    ]
    .into_iter()
    .filter(|(r, c)| (*r >= 0) & (*c >= 0) & (*r < 10) & (*c < 10))
    .map(|(r, c)| (r as usize, c as usize))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut m = Matrix::new(input);
        for _ in 0..10 {
            m.next_day();
        }
        assert_eq!(204, m.flash_count);
    }
}
