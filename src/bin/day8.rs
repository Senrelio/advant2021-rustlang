use std::{collections::HashMap, panic};

use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/day8_input");
    println!("day8 part1: {}", part1(input));
    println!("day8 part2: {}", part2(input));
}

// n  seg_n digits
// 0    6   [a,b,c, ,e,f,g]
// 1    2   [ , ,c, , ,f, ]
// 2    5   [a, ,c,d,e, ,g]
// 3    5   [a, ,c,d, ,f,g]
// 4    4   [ ,b,c,d, ,f, ]
// 5    5   [a,b, ,d, ,f,g]
// 6    6   [a,b, ,d,e,f,g]
// 7    3   [a, ,c, , ,f, ]
// 8    7   [a,b,c,d,e,f,g]
// 9    6   [a,b,c,d, ,f,g]

// n_seg n
// 2     1
// 3     7
// 4     4
// 5     2,3,5
// 6     0,6,9
// 7     8

fn part1(input: &str) -> usize {
    let input = pretreat(input);
    input
        .into_iter()
        .map(|e| e.tails)
        .flatten()
        .filter(|s| {
            let c_count = s.chars().count();
            match c_count {
                2 | 3 | 4 | 7 => true,
                _ => false,
            }
        })
        .count()
}

fn part2(input: &str) -> u128 {
    let mut sum = 0u128;
    for entry in pretreat(input) {
        let dict = entry.dict();
        let tails = entry.tails;
        let n_str: String = tails.into_iter().map(|s| comb_to_char(s, &dict)).collect();
        let n = u128::from_str_radix(&n_str, 10).unwrap();
        sum += n;
    }
    sum
}

lazy_static::lazy_static! {
    static ref RE: Regex = Regex::new(r"(?P<heads>([[:lower:]]+ ){10})\|(?P<tails>( [[:lower:]]+){4})").unwrap();
}

fn pretreat<'a>(input: &'a str) -> Vec<Entry<'a>> {
    let mut v = vec![];
    for line in input.lines() {
        let cap = RE.captures(line.trim()).unwrap();
        let heads = cap
            .name("heads")
            .unwrap()
            .as_str()
            .trim()
            .split(' ')
            .take(10)
            .enumerate()
            .fold([""; 10], |mut arr, (i, s)| {
                arr[i] = s.trim();
                arr
            });
        let tails = cap
            .name("tails")
            .unwrap()
            .as_str()
            .trim()
            .split(' ')
            .take(4)
            .enumerate()
            .fold([""; 4], |mut arr, (i, s)| {
                arr[i] = s.trim();
                arr
            });
        let entry = Entry { heads, tails };
        v.push(entry);
    }
    v
}

#[derive(Debug)]
struct Entry<'a> {
    heads: [&'a str; 10],
    tails: [&'a str; 4],
}

impl<'a> Entry<'a> {
    fn dict(&self) -> HashMap<char, char> {
        let mut char_dict = HashMap::with_capacity(7); // map[str:Option<number>]
                                                       // char_dict.insert('a', None);
                                                       // char_dict.insert('b', None);
                                                       // char_dict.insert('c', None);
                                                       // char_dict.insert('d', None);
                                                       // char_dict.insert('e', None);
                                                       // char_dict.insert('f', None);
                                                       // char_dict.insert('g', None);
        let heads = self.heads;
        // c, f
        let comb_1 = heads.iter().find(|s| s.chars().count() == 2).unwrap();
        // b, c, d, f
        let comb_4 = heads.iter().find(|s| s.chars().count() == 4).unwrap();
        // a, c, f
        let comb_7 = heads.iter().find(|s| s.chars().count() == 3).unwrap();
        // all
        let _comb_8 = heads.iter().find(|s| s.chars().count() == 7).unwrap();
        let mut encoded_sum = heads.iter().map(|s| s.chars()).flatten().fold(
            HashMap::<char, (usize, bool)>::with_capacity(7),
            |mut d, c| {
                // let (mut count, _decoded) = d.entry(c).or_insert((0, false));
                let v = d.entry(c).or_insert((0, false));
                *v = (v.0 + 1, v.1);
                d
            },
        );
        char_dict.insert(tick(&mut encoded_sum, 6), 'b');
        char_dict.insert(tick(&mut encoded_sum, 4), 'e');
        char_dict.insert(tick(&mut encoded_sum, 9), 'f');
        subtract_comb_from_sum(&mut encoded_sum, comb_1);
        subtract_comb_from_sum(&mut encoded_sum, comb_4);
        char_dict.insert(tick(&mut encoded_sum, 8), 'a');
        char_dict.insert(tick(&mut encoded_sum, 7), 'g');
        subtract_comb_from_sum(&mut encoded_sum, comb_7);
        char_dict.insert(tick(&mut encoded_sum, 5), 'c');
        char_dict.insert(tick(&mut encoded_sum, 6), 'd');
        char_dict
    }
}

fn tick(sum: &mut HashMap<char, (usize, bool)>, n: usize) -> char {
    let encoded_c = *sum
        .iter_mut()
        .find(|(_k, (count, decoded))| !decoded && *count == n)
        .unwrap()
        .0;
    sum.get_mut(&encoded_c).unwrap().1 = true;
    encoded_c
}

fn subtract_comb_from_sum(sum: &mut HashMap<char, (usize, bool)>, comb: &str) {
    for c in comb.chars() {
        let v = sum.get_mut(&c).unwrap();
        *v = (v.0 - 1, v.1);
    }
}

lazy_static::lazy_static! {
    static ref COMB_DICT: HashMap<&'static str, char> = {
        let mut map = HashMap::new();
        map.insert("abcefg", '0');
        map.insert("cf", '1');
        map.insert("acdeg", '2');
        map.insert("acdfg", '3');
        map.insert("bcdf", '4');
        map.insert("abdfg", '5');
        map.insert("abdefg", '6');
        map.insert("acf", '7');
        map.insert("abcdefg", '8');
        map.insert("abcdfg", '9');
        map
    };
}

fn comb_to_char(comb: &str, dict: &HashMap<char, char>) -> char {
    let mut comb: Vec<char> = comb.chars().map(|c| *dict.get(&c).unwrap()).collect();
    comb.sort();
    let comb: String = comb.into_iter().collect();
    if let Some(c) = COMB_DICT.get(comb.as_str()) {
        *c
    } else {
        panic!("comb_dict has no {}", &comb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn regex_test() {
        let s =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";
        let caps = RE.captures(s).unwrap();
        assert_eq!(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb ",
            caps.name("heads").unwrap().as_str()
        );
        assert_eq!(
            " fdgacbe cefdb cefbgd gcbe",
            caps.name("tails").unwrap().as_str()
        );
    }
    #[test]
    fn dict_test() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let input = pretreat(input);
        let entry = &input[0];
        let dict = entry.dict();
        let mut nums = vec![];
        for head in &entry.heads {
            let mut chars: Vec<char> = head.chars().map(|c| *dict.get(&c).unwrap()).collect();
            chars.sort();
            let s = COMB_DICT
                .get(chars.into_iter().collect::<String>().as_str())
                .unwrap();
            let n = u8::from_str_radix([s].into_iter().collect::<String>().as_str(), 10).unwrap();
            nums.push(n);
        }
        assert_eq!(vec![8, 5, 2, 3, 7, 9, 6, 4, 0, 1], nums);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day8_test");
        assert_eq!(26, part1(input));
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day8_test");
        assert_eq!(61229, part2(input));
    }
}
