use advant2020::get_input;

fn main() {
    let input = get_input(1);
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let mut count = 0;
    let (mut prev, mut now) = (None, None);
    for n in input.lines().map(|l| l.parse::<i32>().unwrap()) {
        core::mem::swap(&mut prev, &mut now);
        now = Some(n);
        if let (Some(p), Some(n)) = (prev, now) {
            if n > p {
                count += 1;
            }
        }
    }
    println!("day1\tpart1: {}", count);
}

fn part2(input: &str) {
    let mut count = 0;
    let (mut one, mut two, mut three, mut now) = (None, None, None, None);
    for n in input.lines().map(|l| l.parse::<i32>().unwrap()) {
        one = two;
        two = three;
        three = now;
        now = Some(n);
        if let (Some(one), Some(two), Some(three), Some(now)) = (one, two, three, now) {
            if (now + three + two) > (one + two + three) {
                count += 1;
            }
        }
    }
    println!("day1\tpart2: {}", count);
}
