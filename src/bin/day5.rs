use std::{collections::HashMap, fmt::Display};

lazy_static::lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?P<x1>\d*),(?P<y1>\d*) -> (?P<x2>\d*),(?P<y2>\d*)").unwrap();
}

fn main() {
    let input = include_str!("../../inputs/day5_input");
    println!("day5, part1: {}", part1(input));
    println!("day5, part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let entries = pretreat(input);
    let mut diagram = Diagram::default();
    for entry in entries.iter().filter(|e| e.horizontal() | e.vertical()) {
        let coverred = entry.coverred();
        for p in coverred {
            let count = diagram.entry(p).or_insert(0);
            *count += 1;
        }
    }
    diagram.map.values().filter(|v| **v > 1).count()
}

fn part2(input: &str) -> usize {
    let entries = pretreat(input);
    let mut diagram = Diagram::default();
    for entry in entries
        .iter()
        .filter(|e| e.horizontal() | e.vertical() | e.diagonal())
    {
        let coverred = entry.coverred();
        for p in coverred {
            let count = diagram.entry(p).or_insert(0);
            *count += 1;
        }
    }
    diagram.map.values().filter(|v| **v > 1).count()
}

fn pretreat(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            let r1 = cap.name("x1").unwrap().as_str().parse().unwrap();
            let c1 = cap.name("y1").unwrap().as_str().parse().unwrap();
            let r2 = cap.name("x2").unwrap().as_str().parse().unwrap();
            let c2 = cap.name("y2").unwrap().as_str().parse().unwrap();
            let from = Point { r: r1, c: c1 };
            let to = Point { r: r2, c: c2 };
            Entry { from, to }
        })
        .collect()
}

#[derive(Debug)]
struct Diagram {
    map: HashMap<Point, usize>,
}

impl Default for Diagram {
    fn default() -> Self {
        Self {
            map: Default::default(),
        }
    }
}

impl Diagram {
    fn entry(&mut self, p: Point) -> std::collections::hash_map::Entry<Point, usize> {
        self.map.entry(p)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    r: i32,
    c: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}

#[derive(Debug)]
struct Entry {
    from: Point,
    to: Point,
}

impl Entry {
    fn vertical(&self) -> bool {
        self.from.c == self.to.c
    }
    fn horizontal(&self) -> bool {
        self.from.r == self.to.r
    }
    fn diagonal(&self) -> bool {
        (self.from.r - self.to.r).abs() == (self.from.c - self.to.c).abs()
    }
    fn coverred(&self) -> Vec<Point> {
        if self.horizontal() {
            let from_c = self.from.c;
            let to_c = self.to.c;
            let range = if from_c < to_c {
                from_c..=to_c
            } else {
                to_c..=from_c
            };
            range.map(|c| Point { r: self.from.r, c }).collect()
        } else if self.vertical() {
            let from_r = self.from.r;
            let to_r = self.to.r;
            let range = if from_r < to_r {
                from_r..=to_r
            } else {
                to_r..=from_r
            };
            range.map(|r| Point { r, c: self.from.c }).collect()
        } else if self.diagonal() {
            let (l, r) = if self.from.c > self.to.c {
                (self.to, self.from)
            } else {
                (self.from, self.to)
            };
            if l.r < r.r {
                (0..=r.r - l.r)
                    .map(|step| Point {
                        r: l.r + step,
                        c: l.c + step,
                    })
                    .collect()
            } else {
                (0..=l.r - r.r)
                    .map(|step| Point {
                        r: l.r - step,
                        c: l.c + step,
                    })
                    .collect()
            }
        } else {
            unimplemented!("only need to consider vertical, horizontal and diagonal")
        }
    }
}

#[cfg(test)]
mod tests_day5 {
    use super::*;
    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day5_test");
        let answer = part1(input);
        assert_eq!(5, answer);
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day5_test");
        let answer = part2(input);
        assert_eq!(12, answer);
    }
}
