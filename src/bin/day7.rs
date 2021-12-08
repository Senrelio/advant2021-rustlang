fn main() {
    let input = include_str!("../../inputs/day7_input");
    let input = pretreat(input);
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

fn pretreat(input: &str) -> Vec<i128> {
    input
        .split(',')
        .map(|s| i128::from_str_radix(s.trim(), 10).unwrap())
        .collect()
}

fn part1(input: &[i128]) -> i128 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    (*min..*max)
        .into_iter()
        .map(|n| input.iter().map(|&location| (n - location).abs()).sum())
        .min()
        .unwrap()
}

fn part2(input: &[i128]) -> i128 {
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    (*min..*max)
        .into_iter()
        .map(|n| {
            input
                .iter()
                .map(|&location| {
                    let distance = (n - location).abs();
                    (1 + distance) * distance / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

#[test]
fn part1_test() {
    let input = include_str!("../../inputs/day7_test");
    let input = pretreat(input);
    assert_eq!(37, part1(&input));
}

#[test]
fn part2_test() {
    let input = include_str!("../../inputs/day7_test");
    let input = pretreat(input);
    assert_eq!(168, part2(&input));
}
