use std::collections::HashSet;

fn main() {
    let input = include_str!("../../inputs/day9_input");
    println!("day9 part1: {}", part1::<100, 100>(input));
    println!("day9 part2: {}", part2::<100, 100>(input));
}

fn part1<const R: usize, const C: usize>(input: &str) -> u32 {
    let matrix = pretreat::<R, C>(input);
    let mut sum = 0u32;
    for r in 0..R {
        for c in 0..C {
            let mid = matrix[r][c];
            let left = if c == 0 { 10 } else { matrix[r][c - 1] };
            let up = if r == 0 { 10 } else { matrix[r - 1][c] };
            let right = if c == C - 1 { 10 } else { matrix[r][c + 1] };
            let down = if r == R - 1 { 10 } else { matrix[r + 1][c] };
            if [left, up, right, down].iter().min().unwrap() > &mid {
                // println!("find low points: ({},{})", r, c);
                sum += mid as u32 + 1;
            }
        }
    }
    sum
}
// graph?

fn part2<const R: usize, const C: usize>(input: &str) -> u128 {
    let matrix = pretreat::<R, C>(input);
    let mut mids = [[Mid::default(); C]; R];
    for r in 0..R {
        for c in 0..C {
            let mid = matrix[r][c];
            let left = if c == 0 {
                None
            } else {
                let v_left = matrix[r][c - 1];
                if v_left > mid {
                    Some((r, c - 1))
                } else {
                    None
                }
            };
            let up = if r == 0 {
                None
            } else {
                let v_up = matrix[r - 1][c];
                if v_up > mid {
                    Some((r - 1, c))
                } else {
                    None
                }
            };
            let right = if c == C - 1 {
                None
            } else {
                let v_right = matrix[r][c + 1];
                if v_right > mid {
                    Some((r, c + 1))
                } else {
                    None
                }
            };
            let down = if r == R - 1 {
                None
            } else {
                let v_down = matrix[r + 1][c];
                if v_down > mid {
                    Some((r + 1, c))
                } else {
                    None
                }
            };
            mids[r][c] = Mid {
                idx: (r, c),
                v: matrix[r][c],
                left,
                up,
                right,
                down,
            }
        }
    }
    let mut basins = vec![];
    for r in 0..R {
        for c in 0..C {
            let mid = matrix[r][c];
            let left = if c == 0 { 10 } else { matrix[r][c - 1] };
            let up = if r == 0 { 10 } else { matrix[r - 1][c] };
            let right = if c == C - 1 { 10 } else { matrix[r][c + 1] };
            let down = if r == R - 1 { 10 } else { matrix[r + 1][c] };
            if [left, up, right, down].iter().min().unwrap() > &mid {
                let basin = mids[r][c].basin(&matrix, &mids);
                basins.push(basin.len() as u128);
            }
        }
    }
    basins.sort();
    basins.iter().rev().take(3).fold(1, |acc, n| acc * n)
}

#[derive(Clone, Copy)]
struct Mid {
    idx: (usize, usize),
    v: u8,
    left: Option<(usize, usize)>,
    up: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
    down: Option<(usize, usize)>,
}

impl Default for Mid {
    fn default() -> Self {
        Self {
            idx: Default::default(),
            v: Default::default(),
            left: Default::default(),
            up: Default::default(),
            right: Default::default(),
            down: Default::default(),
        }
    }
}

impl Mid {
    fn basin<const R: usize, const C: usize>(
        &self,
        matrix: &[[u8; R]; C],
        mids: &[[Mid; R]; C],
    ) -> HashSet<(usize, usize)> {
        let mut set = HashSet::new();
        if self.v == 9 {
            return set;
        }
        set.insert(self.idx);
        if let Some((r, c)) = self.left {
            let left = &mids[r][c];
            let left_basin = left.basin(matrix, mids);
            set.extend(left_basin);
        }
        if let Some((r, c)) = self.up {
            let up = &mids[r][c];
            let up_basin = up.basin(matrix, mids);
            set.extend(up_basin);
        }
        if let Some((r, c)) = self.right {
            let right = &mids[r][c];
            let right_basin = right.basin(matrix, mids);
            set.extend(right_basin);
        }
        if let Some((r, c)) = self.down {
            let down = &mids[r][c];
            let down_basin = down.basin(matrix, mids);
            set.extend(down_basin);
        }
        set
    }
}

fn pretreat<const R: usize, const C: usize>(input: &str) -> [[u8; C]; R] {
    let mut matrix = [[0; C]; R];
    for (r, line) in input.lines().enumerate() {
        for (c, n) in line.chars().enumerate() {
            matrix[r][c] = n as u8 - 48;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretreat_test() {
        let input = include_str!("../../inputs/day9_test");
        let matrix = pretreat::<5, 10>(input);
        let expect = vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ];
        assert_eq!(expect, matrix);
        let input = include_str!("../../inputs/day9_input");
        let r = input.lines().count();
        let c = input.lines().next().unwrap().chars().count();
        println!("r: {}, c: {}", r, c);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day9_test");
        assert_eq!(15, part1::<5, 10>(input));
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day9_test");
        assert_eq!(1134, part2::<5, 10>(input));
    }
}
