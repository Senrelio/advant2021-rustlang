use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../inputs/day15_input");
    println!("day15 part1: {}", part1(input));
    println!("day15 part2: {}", part2(input));
}

fn pretreat(input: &str) -> Graph {
    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as u8 - 48) as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let max_r = (matrix.len() - 1) as i64;
    let max_c = (matrix[0].len() - 1) as i64;
    let mut edges = HashMap::new();
    for r in 0..=max_r {
        for c in 0..=max_c {
            let id = NodeId::of((r as usize, c as usize));
            let adjancants = [(r, c - 1), (r - 1, c), (r, c + 1), (r + 1, c)];
            let adjancants: HashSet<Edge> = adjancants
                .into_iter()
                .filter(|&(r, c)| (r >= 0) & (r <= max_r) & (c >= 0) & (c <= max_c))
                .map(|(r, c)| (r as usize, c as usize))
                .map(|p| Edge {
                    to: NodeId::of(p),
                    weight: matrix[p.0][p.1],
                })
                .collect();
            edges.insert(id, adjancants);
        }
    }
    let nodes: Vec<Vec<Node>> = matrix
        .into_iter()
        .map(|line| line.into_iter().map(|weight| Node { weight }).collect())
        .collect();
    Graph { nodes, edges }
}

fn flip(mut origin: u32, n: usize) -> u32 {
    for _ in 1..=n {
        origin = match origin {
            i if i < 9 => i + 1,
            9 => 1,
            _ => panic!(),
        }
    }
    origin
}

fn pretreat2(input: &str) -> Graph {
    let mut matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| (c as u8 - 48) as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    for line in matrix.iter_mut() {
        let seq = line.clone();
        for i in 1..=4 {
            for &n in &seq {
                line.push(flip(n, i));
            }
        }
    }
    let lines = matrix.clone();
    for i in 1..=4 {
        for r in 0..lines.len() {
            matrix.push(lines[r].iter().map(|n| flip(*n, i)).collect());
        }
    }
    let max_r = (matrix.len() - 1) as i64;
    let max_c = (matrix[0].len() - 1) as i64;
    let mut edges = HashMap::new();
    for r in 0..=max_r {
        for c in 0..=max_c {
            let id = NodeId::of((r as usize, c as usize));
            let adjancants = [(r, c - 1), (r - 1, c), (r, c + 1), (r + 1, c)];
            let adjancants: HashSet<Edge> = adjancants
                .into_iter()
                .filter(|&(r, c)| (r >= 0) & (r <= max_r) & (c >= 0) & (c <= max_c))
                .map(|(r, c)| (r as usize, c as usize))
                .map(|p| Edge {
                    to: NodeId::of(p),
                    weight: matrix[p.0][p.1],
                })
                .collect();
            edges.insert(id, adjancants);
        }
    }
    let nodes: Vec<Vec<Node>> = matrix
        .into_iter()
        .map(|line| line.into_iter().map(|weight| Node { weight }).collect())
        .collect();
    // for line in &nodes {
    //     for w in line {
    //         print!("{}", w.weight);
    //     }
    //     println!("");
    // }
    Graph { nodes, edges }
}

fn part1(input: &str) -> u32 {
    let graph = pretreat(input);
    let start = NodeId::of((0, 0));
    let end = NodeId::of((graph.nodes.len() - 1, graph.nodes[0].len() - 1));
    graph.dijkstra(start, end)
}

fn part2(input: &str) -> u32 {
    let graph = pretreat2(input);
    let start = NodeId::of((0, 0));
    let end = NodeId::of((graph.nodes.len() - 1, graph.nodes[0].len() - 1));
    graph.dijkstra(start, end)
}

struct Graph {
    nodes: Vec<Vec<Node>>,
    edges: HashMap<NodeId, HashSet<Edge>>,
}

impl Graph {
    fn dijkstra(&self, start: NodeId, end: NodeId) -> u32 {
        let row = self.nodes.len();
        let column = self.nodes[0].len();
        let mut distances = vec![vec![(u32::MAX, false); column]; row];
        distances[start.r][start.c].0 = 0;
        loop {
            if distances.iter().flatten().all(|&(_, visited)| visited) {
                break;
            }
            let current = {
                let current = distances
                    .iter()
                    .enumerate()
                    .map(|(r, line)| line.iter().enumerate().map(move |(c, n)| (r, c, n)))
                    .flatten()
                    .filter(|(_r, _c, (_dist, visited))| !visited)
                    .min_by_key(|&(_r, _c, (dist, _visited))| dist);
                if let Some(current) = current {
                    NodeId::of((current.0, current.1))
                } else {
                    break;
                }
            };
            distances[current.r][current.c].1 = true;
            for edge in self.edges.get(&current).unwrap() {
                let alt = edge.weight + distances[current.r][current.c].0;
                if alt < distances[edge.to.r][edge.to.c].0 {
                    distances[edge.to.r][edge.to.c].0 = alt
                }
            }
        }
        distances[end.r][end.c].0
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct PetEdge {
    from: NodeId,
    to: NodeId,
    weight: u32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct NodeId {
    r: usize,
    c: usize,
}

impl NodeId {
    fn of(point: (usize, usize)) -> Self {
        NodeId {
            r: point.0,
            c: point.1,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Node {
    weight: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Edge {
    to: NodeId,
    weight: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day15_test");
        assert_eq!(40, part1(input));
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day15_test");
        assert_eq!(315, part2(input));
    }
    #[test]
    fn dijkstra2_test() {
        let input = include_str!("../../inputs/day15_test");
        let graph = pretreat(input);
        graph.dijkstra(NodeId::of((0, 0)), NodeId::of((9, 9)));
    }
}
