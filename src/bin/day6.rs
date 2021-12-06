fn main() {
    let input = include_str!("../../inputs/day6_input");
    println!("day 6 part 1: {}", part1(input, 80));
    println!("day 6 part 2: {}", part2(input, 256));
}

fn part1(input: &str, days: usize) -> u128 {
    _solution(input, days)
}
fn part2(input: &str, days: usize) -> u128 {
    _solution(input, days)
}

fn _solution(input: &str, day: usize) -> u128 {
    let input: Vec<usize> = input
        .split(',')
        .map(|i| i.trim().parse().unwrap())
        .collect();
    let mut map = [0u128; 9];
    for n in input {
        map[n] += 1u128;
    }
    for _ in 0..day {
        let givebirth_today = map[0];
        for i in 1..9 {
            map[i - 1] = map[i];
        }
        map[8] = givebirth_today;
        map[6] += givebirth_today;
    }
    map.into_iter().sum()
}

#[cfg(test)]
mod tests_day6 {
    use super::*;
    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day6_test");
        assert_eq!(26, part1(input, 18));
        assert_eq!(5934, part1(input, 80));
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day6_test");
        assert_eq!(26984457539u128, part2(input, 256));
    }
}
