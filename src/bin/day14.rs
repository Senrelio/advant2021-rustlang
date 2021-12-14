use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day14_input");
    println!("day14 part1: {}", solution(input, 10));
    println!("day14 part1: {}", solution(input, 40));
}

fn solution(input: &str, steps: usize) -> usize {
    let (chars, map) = pretreat(input);
    let mut char_counts = HashMap::new();
    for &c in &chars {
        let count = char_counts.entry(c).or_insert(0);
        *count += 1;
    }
    let mut seqs: HashMap<(char, char), usize> = HashMap::new();
    for seq in chars.windows(2) {
        let count = seqs.entry((seq[0], seq[1])).or_insert(0);
        *count += 1;
    }
    for _ in 0..steps {
        let mut tmp = HashMap::new();
        for (seq, &count) in &seqs {
            let mid = map.get(seq).unwrap().clone();
            let left = (seq.0, mid);
            let right = (mid, seq.1);
            let c_count = char_counts.entry(mid).or_insert(0);
            *c_count += count;
            let c = tmp.entry(left).or_insert(0);
            *c += count;
            let c = tmp.entry(right).or_insert(0);
            *c += count;
        }
        seqs = tmp;
    }
    let mut counts: Vec<usize> = char_counts.into_values().collect();
    counts.sort();
    counts.last().unwrap() - counts[0]
}

lazy_static::lazy_static! {
    static ref INSERTION: Regex = Regex::new(r"(?P<l>\w)(?P<r>\w) -> (?P<mid>\w)").unwrap();
}

fn pretreat(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (section1, section2) = input.split_once("\n\n").unwrap();
    let chars = section1.chars().collect();
    let mut map = HashMap::new();
    for line in section2.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        let caps = INSERTION.captures(line).unwrap();
        let l = caps.name("l").unwrap().as_str().chars().next().unwrap();
        let r = caps.name("r").unwrap().as_str().chars().next().unwrap();
        let mid = caps.name("mid").unwrap().as_str().chars().next().unwrap();
        map.insert((l, r), mid);
    }
    (chars, map)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> &'static str {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        input
    }
    #[test]
    fn part1_test() {
        assert_eq!(1588, solution(input(), 10));
    }
    #[test]
    fn part2_test() {
        assert_eq!(2188189693529, solution(input(), 40))
    }
}
