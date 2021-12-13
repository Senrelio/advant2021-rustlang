use std::collections::HashSet;

use regex::Regex;
fn main() {
    let input = include_str!("../../inputs/day13_input");
    println!("day13 part1: {}", part1(input));
    part2(input);
}

fn part1(input: &str) -> usize {
    let (mut points, folds) = pretreat(input);
    let fold = &folds[0];
    points = points
        .into_iter()
        .filter_map(|p| p.new_position(&fold))
        .collect();
    points.len()
}

fn part2(input: &str) {
    let (mut points, folds) = pretreat(input);
    for fold in &folds {
        points = points
            .into_iter()
            .filter_map(|p| p.new_position(&fold))
            .collect();
    }
    let max_x = points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = points.iter().max_by_key(|p| p.y).unwrap().y;
    let mut matrix = {
        let mut m = Vec::with_capacity(max_y);
        let mut row = Vec::with_capacity(max_x);
        for _ in 0..=max_x {
            row.push('.');
        }
        for _ in 0..=max_y {
            m.push(row.clone());
        }
        m
    };
    for Point { x, y } in points {
        matrix[y][x] = '#';
    }
    for row in matrix {
        println!("{}", String::from_iter(row.iter()));
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new_position(self, fold: &Fold) -> Option<Point> {
        match fold {
            Fold::Y(y) => {
                if &self.y > y {
                    Some(Point {
                        x: self.x,
                        y: y * 2 - self.y,
                    })
                } else if &self.y == y {
                    None
                } else {
                    Some(self)
                }
            }
            Fold::X(x) => {
                if &self.x > x {
                    Some(Point {
                        x: 2 * x - self.x,
                        y: self.y,
                    })
                } else if &self.x == x {
                    None
                } else {
                    Some(self)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Fold {
    Y(usize),
    X(usize),
}

lazy_static::lazy_static! {
    static ref POINT: Regex = Regex::new(r"(?P<x>\d*),(?P<y>\d*)").unwrap();
    static ref FOLD: Regex = Regex::new(r"fold along (?P<direction>.)=(?P<num>\d*)").unwrap();
}

fn pretreat(input: &str) -> (HashSet<Point>, Vec<Fold>) {
    let mut points = HashSet::new();
    let mut folds = vec![];
    let (section_1, section_2) = input.split_once("\n\n").unwrap();
    for line in section_1.lines().filter(|l| !l.is_empty()) {
        let line = line.trim();
        let caps = POINT.captures(line).unwrap();
        let x = caps.name("x").unwrap().as_str().parse().unwrap();
        let y = caps.name("y").unwrap().as_str().parse().unwrap();
        points.insert(Point { x, y });
    }
    for line in section_2.lines() {
        let line = line.trim();
        let caps = FOLD.captures(line).unwrap();
        let n = caps.name("num").unwrap().as_str().parse().unwrap();
        let fold = match caps.name("direction").unwrap().as_str() {
            "x" => Fold::X(n),
            "y" => Fold::Y(n),
            _ => panic!(),
        };
        folds.push(fold);
    }
    (points, folds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regex_test() {
        let input = "10,12";
        let caps = POINT.captures(input).unwrap();
        assert_eq!("10", caps.name("x").unwrap().as_str());
        assert_eq!("12", caps.name("y").unwrap().as_str());
        let input = "fold along y=7
fold along x=5";
        let mut lines = input.lines();
        let first_line = lines.next().unwrap().trim();
        let caps = FOLD.captures(first_line).unwrap();
        assert_eq!("y", caps.name("direction").unwrap().as_str());
        assert_eq!("7", caps.name("num").unwrap().as_str());
        let second_line = lines.next().unwrap().trim();
        let caps = FOLD.captures(second_line).unwrap();
        assert_eq!("x", caps.name("direction").unwrap().as_str());
        assert_eq!("5", caps.name("num").unwrap().as_str());
    }

    #[test]
    fn test_pretreat() {
        let input = "
8,10
9,0

fold along y=7
fold along x=5";
        let (points, folds) = pretreat(input);
        let expect = [Point { x: 8, y: 10 }, Point { x: 9, y: 0 }];
        assert_eq!(&expect[0], points.get(&expect[0]).unwrap());
        assert_eq!(&expect[1], points.get(&expect[1]).unwrap());
        assert_eq!(vec![Fold::Y(7), Fold::X(5)], folds);
    }
    #[test]
    fn part1_test() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7";
        assert_eq!(17, part1(input));
    }
}
