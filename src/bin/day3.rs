fn main() {
    let input: &'static str = include_str!("../../inputs/day3_input");
    // anyway to get single line length at compile time? it should be possible.
    let input = pretreat::<12>(&input);
    println!("day3 part1: {}", part1(&input));
    println!("day3 part2: {}", part2(&input));
}

fn pretreat<const N: usize>(input: &str) -> Vec<[bool; N]> {
    let mut v = vec![];
    for s in input.lines() {
        let s = s.trim();
        let mut bools = [false; N];
        let mut chars = s.chars();
        for i in 0..N {
            bools[i] = chars.next().unwrap() == '1';
        }
        v.push(bools);
    }
    v
}

fn part1<const N: usize>(input: &[[bool; N]]) -> u32 {
    let length = input.len();
    let mut ones = [0; N];
    for arr in input {
        for i in 0..N {
            if arr[i] {
                ones[i] += 1;
            }
        }
    }
    let gamma: Vec<bool> = ones
        .into_iter()
        .map(|n_one| n_one >= length - n_one)
        .collect();
    let gamma = bitvec_to_u32(&gamma);
    let epsilon = 2u32.pow(N as u32) - 1 - gamma;
    epsilon * gamma
}

fn part2<const N: usize>(input: &[[bool; N]]) -> u32 {
    let mut cursor = 0;
    let mut input_1 = input.into_iter().collect();
    let oxygen = loop {
        let remain = filter(input_1, cursor, true);
        if remain.len() == 1 {
            break remain[0];
        } else {
            input_1 = remain;
            cursor += 1;
            if cursor >= N {
                cursor = 0;
            }
        }
    };
    let mut input_2 = input.into_iter().collect();
    let mut cursor = 0;
    let co2 = loop {
        let remain = filter(input_2, cursor, false);
        if remain.len() == 1 {
            break remain[0];
        } else {
            input_2 = remain;
            cursor += 1;
            if cursor >= N {
                cursor = 0;
            }
        }
    };
    let oxygen = bitvec_to_u32(oxygen);
    let co2 = bitvec_to_u32(co2);
    oxygen * co2
}

fn filter<const N: usize>(arr: Vec<&[bool; N]>, cursor: usize, flag: bool) -> Vec<&[bool; N]> {
    assert!(cursor < N);
    let length = arr.len();
    let one_count = arr.iter().filter(|v| v[cursor]).count();
    let zero_count = length - one_count;
    let criteria = match Ord::cmp(&one_count, &zero_count) {
        std::cmp::Ordering::Less => !flag,
        std::cmp::Ordering::Equal => flag,
        std::cmp::Ordering::Greater => flag,
    };
    let remain: Vec<&[bool; N]> = arr.into_iter().filter(|v| v[cursor] == criteria).collect();
    remain
}
fn bitvec_to_u32(bitvec: &[bool]) -> u32 {
    bitvec
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |prev, (i, b)| match b {
            true => prev + 2u32.pow(i as u32),
            false => prev,
        })
}


// const fn position(str: &'static str, p: char) -> usize {

// }
#[cfg(test)]
mod tests {
    use super::*;
    fn test_input() -> Vec<[bool; 5]> {
        let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
        pretreat::<5>(input)
    }
    #[test]
    fn part1_test() {
        let input = test_input();
        assert_eq!(198, part1(&input));
    }
    #[test]
    fn part2_test() {
        let input = test_input();
        assert_eq!(230, part2(&input));
    }
}
