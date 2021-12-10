use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day4_input");
    println!("day4 part1: {}", part1(input));
    println!("day4 part2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let (nums, mut boards) = pretreat(input);
    let mut score = 0;
    'outer: for n in nums {
        for b in &mut boards {
            let bingoed = b.tick(n);
            if bingoed.is_some() {
                score = b.final_score(n);
                break 'outer;
            }
        }
    }
    score
}

fn part2(input: &str) -> u32 {
    let (nums, mut boards) = pretreat(input);
    let size = boards.len();
    let mut score = 0;
    let mut won_count = 0;
    'outer: for n in nums {
        let remain = boards.iter_mut().filter(|b| !b.won);
        for b in remain {
            let r = b.tick(n);
            if r.is_some() {
                won_count += 1;
                if won_count == size {
                    score = b.final_score(n);
                    break 'outer;
                }
            }
        }
    }
    score
}
fn pretreat(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();
    let nums = lines
        .next()
        .expect("input has no content")
        .split(',')
        .map(|s| s.parse::<u32>().expect("number str is not a number"))
        .collect();
    let str_blocks = lines.fold(vec![], |mut arr, line| {
        if line.is_empty() {
            arr.push(Vec::with_capacity(5));
        } else {
            arr.last_mut()
                .expect("there is no string vec at the end of arr while pretreat input")
                .push(line);
        }
        arr
    });
    let num_blocks = str_blocks.into_iter().map(|strs| {
        let mut nums = [[0; 5]; 5];
        for (r, line) in strs.into_iter().enumerate() {
            for (c, n) in line.split_ascii_whitespace().enumerate() {
                let n = n.parse().unwrap();
                nums[r][c] = n;
            }
        }
        nums
    });
    let mut boards = vec![];
    for (idx, block) in num_blocks.enumerate() {
        let mut board = Board {
            _idx: idx,
            nums: HashMap::with_capacity(25),
            ticked: [[false; 5]; 5],
            won: false,
        };
        for (r, row) in block.into_iter().enumerate() {
            for (c, n) in row.into_iter().enumerate() {
                board.nums.insert(n, (r, c));
            }
        }
        boards.push(board);
    }
    (nums, boards)
}

#[derive(Debug)]
struct Board {
    _idx: usize,
    nums: HashMap<u32, (usize, usize)>,
    ticked: [[bool; 5]; 5],
    won: bool,
}

impl Board {
    fn tick(&mut self, n: u32) -> Option<()> {
        if let Some(&(r, c)) = self.nums.get(&n) {
            self.ticked[r][c] = true;
            if bingo(self.ticked).is_some() {
                self.won = true;
                Some(())
            } else {
                None
            }
        } else {
            None
        }
    }
    fn final_score(&self, last_n: u32) -> u32 {
        let mut map = [[0; 5]; 5];
        for (&n, &(r, c)) in &self.nums {
            map[r][c] = n;
        }
        map.iter()
            .zip(self.ticked)
            .map(|(row, flags)| {
                let mut sum = 0;
                for (i, &f) in flags.iter().enumerate() {
                    if !f {
                        sum += row[i];
                    }
                }
                sum
            })
            .sum::<u32>()
            * last_n
    }
}

fn horizon<const N: usize>(arr: [[bool; N]; N]) -> bool {
    for row in arr {
        if row.into_iter().all(|b| b) {
            return true;
        }
    }
    false
}

fn vertical<const N: usize>(arr: [[bool; N]; N]) -> bool {
    for c in 0..N {
        if arr.into_iter().all(|row| row[c] == true) {
            return true;
        }
    }
    false
}

#[allow(unused)]
fn diagonal<const N: usize>(arr: [[bool; N]; N]) -> bool {
    let down = (0..N).map(|i| arr[i][i]).all(|n| n);
    let up = (0..N).map(|i| arr[N - 1 - i][i]).all(|n| n);
    down | up
}

fn bingo<const N: usize>(ticked: [[bool; N]; N]) -> Option<()> {
    // if horizon(ticked) | vertical(ticked) | diagonal(ticked) {
    if horizon(ticked) | vertical(ticked) {
        Some(())
    } else {
        None
    }
}

#[test]
fn part1_test() {
    let input = include_str!("../../inputs/day4_test");
    let score = part1(input);
    assert_eq!(4512, score);
}
#[test]
fn part2_test() {
    let input = include_str!("../../inputs/day4_test");
    let score = part2(input);
    assert_eq!(1924, score);
}
#[test]
fn score_test() {
    let mut nums = HashMap::with_capacity(25);
    nums.insert(0, (4, 1));
    nums.insert(3, (4, 3));
    nums.insert(7, (4, 4));
    nums.insert(22, (3, 0));
    nums.insert(6, (3, 3));
    nums.insert(9, (1, 3));
    nums.insert(12, (4, 2));
    nums.insert(19, (1, 4));
    nums.insert(5, (3, 4));
    nums.insert(23, (2, 2));
    nums.insert(24, (0, 3));
    nums.insert(16, (1, 1));
    nums.insert(26, (2, 3));
    nums.insert(11, (3, 1));
    nums.insert(10, (1, 0));
    nums.insert(21, (0, 1));
    nums.insert(17, (0, 2));
    nums.insert(18, (2, 0));
    nums.insert(2, (4, 0));
    nums.insert(8, (2, 1));
    nums.insert(4, (0, 4));
    nums.insert(20, (2, 4));
    nums.insert(14, (0, 0));
    nums.insert(15, (1, 2));
    nums.insert(13, (3, 2));
    let b = Board {
        _idx: 2,
        nums,
        ticked: [
            [true, true, true, true, true],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, true],
            [true, true, false, false, true],
        ],
        won: false,
    };
    assert_eq!(4512, b.final_score(24));
}

#[test]
fn bingo_test_1() {
    let b = [
        [false, false, true, false, true],
        [false, false, false, true, false],
        [false, false, true, false, false],
        [false, true, false, false, true],
        [true, false, false, false, true],
    ];
    dbg!(horizon(b));
    dbg!(vertical(b));
    dbg!(diagonal(b));
    assert!(bingo(b).is_none());
}
