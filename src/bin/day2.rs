use advant2020::get_input;
use lazy_static::lazy_static;

lazy_static! {
    static ref RE_COMMAND: regex::Regex = regex::Regex::new(r"(?P<command>\w*) (?P<n>\d)").unwrap();
}

fn main() {
    let input = get_input(2);
    println!("day2 part1: {}", part1(&input));
    println!("day2 part2: {}", part2(&input));
}

enum Command {
    Forward(i32),
    Depth(i32),
}

fn part1(input: &str) -> i32 {
    let mut hor = 0;
    let mut depth = 0;
    for s in input.lines() {
        let cap = RE_COMMAND.captures(s).unwrap();
        let n = cap.name("n").unwrap().as_str().parse::<i32>().unwrap();
        let command = cap.name("command").unwrap().as_str();
        let command = match command {
            "forward" => Command::Forward(n),
            "down" => Command::Depth(n),
            "up" => Command::Depth(-n),
            _ => panic!("unknown command"),
        };
        match command {
            Command::Forward(n) => hor += n,
            Command::Depth(n) => depth += n,
        }
    }
    hor * depth
}

fn part2(input: &str) -> i32 {
    let mut hor = 0;
    let mut depth = 0;
    let mut aim = 0;
    for s in input.lines() {
        let cap = RE_COMMAND.captures(s).unwrap();
        let n = cap.name("n").unwrap().as_str().parse::<i32>().unwrap();
        let command = cap.name("command").unwrap().as_str();
        let command = match command {
            "forward" => Command::Forward(n),
            "down" => Command::Depth(n),
            "up" => Command::Depth(-n),
            _ => panic!("unknown command"),
        };
        match command {
            Command::Forward(n) => {
                hor += n;
                depth += n * aim;
            }
            Command::Depth(n) => aim += n,
        }
    }
    hor * depth
}

#[test]
fn test_part2() {
    let input = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    assert_eq!(900, part2(input));
}
