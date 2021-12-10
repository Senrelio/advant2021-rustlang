use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day10_input");
    println!("day10 part1: {}", part1(input));
    println!("day10 part2: {}", part2(input));
}

lazy_static::lazy_static! {
    static ref SCORE_MAP: HashMap<char, u128> = {
        let mut map = HashMap::with_capacity(4);
        map.insert(')', 3);
        map.insert(']', 57);
        map.insert('}', 1197);
        map.insert('>', 25137);
        map
    };
    static ref BRACKET_MAP: HashMap<char, char> = {
        let mut map = HashMap::with_capacity(4);
        map.insert('(', ')');
        map.insert('[', ']');
        map.insert('{', '}');
        map.insert('<', '>');
        map
    };
    static ref SCORE_MAP2: HashMap<char, u128> = {
        let mut map = HashMap::with_capacity(4);
        map.insert('(', 1);
        map.insert('[', 2);
        map.insert('{', 3);
        map.insert('<', 4);
        map
    };
}

fn part1(input: &str) -> u128 {
    let mut illegals = vec![];
    let mut stacks = vec![];
    for line in input.lines() {
        let line = line.trim();
        let mut stack = Vec::new();
        'line: for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    let last = stack.pop().unwrap();
                    let expect = BRACKET_MAP.get(&last).unwrap();
                    if &c != expect {
                        illegals.push(c);
                        break 'line;
                    }
                }
            }
        }
        stacks.push(stack);
    }
    illegals
        .into_iter()
        .map(|c| SCORE_MAP.get(&c).unwrap())
        .sum()
}

fn part2(input: &str) -> u128 {
    let mut stacks = vec![];
    'outer: for line in input.lines() {
        let line = line.trim();
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                _ => {
                    let last = stack.pop().unwrap();
                    let expect = BRACKET_MAP.get(&last).unwrap();
                    if &c != expect {
                        continue 'outer;
                    }
                }
            }
        }
        if !stack.is_empty() {
            stacks.push(stack);
        }
    }
    let mut scores: Vec<u128> = stacks
        .into_iter()
        .map(|stack| {
            stack
                .into_iter()
                .map(|c| SCORE_MAP2.get(&c).unwrap())
                .rev()
                .fold(0, |acc, current| acc * 5 + current)
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        include_str!("../../inputs/day10_test")
    }

    #[test]
    fn part1_test() {
        assert_eq!(26397, part1(input()));
    }
    #[test]
    fn part2_test() {
        assert_eq!(288957, part2(input()));
    }
}
