use std::ops::Range;

fn main() {
    let input = include_str!("../../inputs/day17_input");
    let (x_range, y_range) = pretreat(input);
    println!("day17 part1: {}", part1(y_range.clone()));
    println!("day17 part2: {}", part2(x_range, y_range));
}

fn pretreat(input: &str) -> (Range<u128>, Range<i128>) {
    let re = regex::Regex::new(
        r"target area: x=(?P<x_l>.*)\.\.(?P<x_r>.*), y=(?P<y_l>.*)\.\.(?P<y_r>.*)",
    )
    .unwrap();
    let caps = re.captures(input).unwrap();
    let x_l = caps.name("x_l").unwrap().as_str().parse().unwrap();
    let x_r: u128 = caps.name("x_r").unwrap().as_str().parse().unwrap();
    let y_l = caps.name("y_l").unwrap().as_str().parse().unwrap();
    let y_r: i128 = caps.name("y_r").unwrap().as_str().parse().unwrap();
    (x_l..x_r + 1, y_l..y_r + 1)
}

fn part1(y_range: Range<i128>) -> i128 {
    let vy_init = y_range.start.abs() - 1;
    vy_init * (vy_init + 1) / 2
}

fn part2(x_range: Range<u128>, y_range: Range<i128>) -> usize {
    let vx_max = x_range.end - 1;
    let vx_min = {
        #[allow(unused)]
        let mut prev2 = None;
        let mut prev = None;
        let mut curr = None;
        let mut break_point = 0;
        for i in 1..vx_max {
            prev2 = prev;
            prev = curr;
            curr = Some(i * (i + 1) / 2);
            if let (Some(prev2), Some(prev), Some(curr)) = (prev2, prev, curr) {
                if (prev2 < prev) & (prev == curr) {
                    break_point = prev;
                    break;
                }
            }
        }
        break_point
    };
    let vy_min = y_range.start;
    let vy_max = y_range.start.abs() - 1;
    let mut combs: Vec<(SpeedX, SpeedY)> = vec![];
    for vx in vx_min..=vx_max {
        'y: for vy in vy_min..=vy_max {
            for t in 1.. {
                let x_dist = x_dists(vx, t);
                let y_dist = y_dists(vy, t);
                if x_range.contains(&x_dist) & y_range.contains(&y_dist) {
                    combs.push((vx, vy));
                    continue 'y;
                }
                if (x_dist >= x_range.end) | (y_dist <= y_range.start) {
                    continue 'y;
                }
            }
        }
    }
    combs.len()
}

type SpeedX = u128;
type SpeedY = i128;
type Step = u32;

fn x_dists(v_init: SpeedX, t: Step) -> u128 {
    match t {
        0 => 0,
        i if (i as u128) < v_init => ((v_init + v_init - i as u128 + 1) * i as u128) / 2,
        _ => (v_init * (v_init + 1)) / 2,
    }
}

fn y_dists(v_init: SpeedY, t: Step) -> i128 {
    (v_init + v_init - t as i128 + 1) * t as i128 / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = "target area: x=20..30, y=-10..-5";
        let (_, y_range) = pretreat(input);
        let max_y = part1(y_range);
        assert_eq!(45, max_y);
    }

    #[test]
    fn part2_test() {
        let input = "target area: x=20..30, y=-10..-5";
        let (x_range, y_range) = pretreat(input);
        assert_eq!(112, part2(x_range, y_range));
    }
}
