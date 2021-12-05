use advent2020::get_input;

fn main() {
    let input = get_input(1);
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let count = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();
    println!("day1\tpart1: {}", count);
}

fn part2(input: &str) {
    let count = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .filter(|w| w[3] > w[0])
        .count();
    println!("day1\tpart2: {}", count);
}
