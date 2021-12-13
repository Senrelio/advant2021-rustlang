use std::collections::HashMap;

fn main() {
    let input = include_str!("../../inputs/day12_input");
    println!("day12 part1: {}", part1(input));
    println!("day12 part2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let graph = pretreat(input);
    let pathes = graph.path_bfs();
    pathes.len()
}
fn part2(input: &str) -> usize {
    let graph = pretreat(input);
    let pathes = graph.path_bfs2();
    pathes.len()
}

struct Part1Context<'g> {
    journeies: Vec<Vec<&'g str>>, // = vec![vec!["start"]];
    paths: Vec<Vec<&'g str>>,     // = vec![];
}

lazy_static::lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?P<l>.*)-(?P<r>.*)").unwrap();
}

fn pretreat(input: &str) -> Graph {
    let mut nodes: HashMap<NodeId, Node> = HashMap::new();
    let mut edges: HashMap<NodeId, Vec<Edge>> = HashMap::new();
    for line in input.lines() {
        let caps = RE.captures(line.trim()).unwrap();
        let l = caps.name("l").unwrap().as_str();
        let r = caps.name("r").unwrap().as_str();
        println!("{} - {}", l, r);
        let l_edge = Edge { to_id: r };
        let r_edge = Edge { to_id: l };
        nodes.entry(l).or_insert(match l {
            "start" => Node {
                node_type: NodeType::Start,
            },
            "end" => Node {
                node_type: NodeType::End,
            },
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Node {
                node_type: NodeType::Small,
            },
            s if s.chars().all(|c| c.is_ascii_uppercase()) => Node {
                node_type: NodeType::Big,
            },
            _ => panic!(),
        });
        nodes.entry(r).or_insert(match r {
            "start" => Node {
                node_type: NodeType::Start,
            },
            "end" => Node {
                node_type: NodeType::End,
            },
            s if s.chars().all(|c| c.is_ascii_lowercase()) => Node {
                node_type: NodeType::Small,
            },
            s if s.chars().all(|c| c.is_ascii_uppercase()) => Node {
                node_type: NodeType::Big,
            },
            _ => panic!(),
        });
        let l_edges = edges.entry(l).or_default();
        l_edges.push(l_edge);
        let r_edges = edges.entry(r).or_default();
        r_edges.push(r_edge);
    }
    Graph { nodes, edges }
}

#[derive(Debug)]
struct Graph<'a> {
    // dict: HashMap<NodeId, String>,
    nodes: HashMap<NodeId<'a>, Node>,
    edges: HashMap<NodeId<'a>, Vec<Edge<'a>>>,
}

impl<'a> Graph<'a> {
    /// do not consider cycle consists of big cave
    /// check if start has nodes to go: must be true
    /// acc:
    ///     1. visited nodes
    ///     2. fallback from
    /// choices filter:
    ///     1. not visited small cave
    ///     2. not start
    ///     3. not what you fallback from
    /// move on:
    ///     1. fallback if end:
    ///     2. fallback if no choice:
    /// when fallback:
    ///     1. pop visited nodes
    ///     2. remember where you fallback from
    ///     3. break if there is nowhere to fallback to: fallback on start
    // fn path(&self) -> Vec<Vec<NodeId<'a>>> {
    //     let mut paths = vec![];
    //     let mut journey = vec![];
    //     journey.push("start");
    //     let mut fallbacks = HashMap::new();
    //     loop {
    //         let current_nid = journey
    //             .last()
    //             .expect("predicate: the journey cannot be empty at the start of a loop")
    //             .clone();
    //         // println!("move to {}, journey so far: {:?}", &current_nid, &journey);
    //         // if current node is end, then push path and fallback
    //         if matches!(
    //             self.nodes.get(&current_nid).unwrap().node_type,
    //             NodeType::End
    //         ) {
    //             let path = journey.clone();
    //             println!("path found: {:?}", &path);
    //             paths.push(path);
    //             journey.pop().unwrap();
    //             let last = journey
    //                 .last()
    //                 .expect("pop end node, there has to be node left")
    //                 .clone();
    //             fallbacks.entry(last).or_insert(vec![]).push(current_nid);
    //             println!(
    //                 "fallback to {}, journey so far: {:?}, fallbacks: {:?}",
    //                 last, &journey, &fallbacks
    //             );
    //             continue;
    //         }
    //         println!("examine node {}:", current_nid);
    //         let choices: Vec<&Edge> = self
    //             .edges
    //             .get(&current_nid)
    //             .expect("expected no isolated node")
    //             .iter()
    //             .filter(|edge| {
    //                 let node_to = self.nodes.get(&edge.to_id).unwrap();
    //                 let start = matches!(node_to.node_type, NodeType::Start);
    //                 let fallback_to_current = fallbacks.entry(current_nid).or_insert(vec![]);
    //                 let fallbacked_from = fallback_to_current.contains(&edge.to_id);
    //                 let visited_small_cave = match node_to.node_type {
    //                     NodeType::Small => journey.contains(&edge.to_id),
    //                     _ => false,
    //                 };
    //                 println!(
    //                     "\t\tchoice {}: {:?}: fallbacked_from: {}, visited small cave: {}",
    //                     edge.to_id, node_to, fallbacked_from, visited_small_cave
    //                 );
    //                 !start & !fallbacked_from & !visited_small_cave
    //             })
    //             .collect();
    //         println!("node {}'s choices are {:?}", current_nid, choices);
    //         if choices.is_empty() {
    //             // fallback
    //             let fallback_from = journey.pop().unwrap();
    //             if journey.is_empty() {
    //                 // fallback from start
    //                 break;
    //             } else {
    //                 let last = journey.last().unwrap().clone();
    //                 fallbacks.entry(last).or_insert(vec![]).push(fallback_from);
    //             }
    //             continue;
    //         }
    //         let choice = choices[0].to_id;
    //         journey.push(choice);
    //     }
    //     paths
    // }
    // fn path_bfs<C>(&self, predicate: impl FnMut(C) -> bool) -> Vec<Vec<NodeId>> {
    fn path_bfs(&self) -> Vec<Vec<NodeId>> {
        let mut journeies = vec![vec!["start"]];
        let mut paths = vec![];
        for _i in 0.. {
            journeies = journeies
                .into_iter()
                .filter_map(|j| {
                    let last = j.last().unwrap();
                    if *last == "end" {
                        paths.push(j);
                        return None;
                    } else {
                        let choices = self
                            .edges
                            .get(last)
                            .unwrap()
                            .iter()
                            .filter(|e| {
                                let not_start = e.to_id != "start";
                                let not_visited_smallcave = !matches!(
                                    self.nodes.get(e.to_id).unwrap().node_type,
                                    NodeType::Small
                                ) | !j.contains(&e.to_id);
                                not_start & not_visited_smallcave
                            })
                            .map(|e| e.to_id);
                        let forks: Vec<Vec<NodeId>> = choices
                            .map(|new_node| {
                                let mut journey_so_far = j.clone();
                                journey_so_far.push(new_node);
                                journey_so_far
                            })
                            .collect();
                        if forks.is_empty() {
                            return None;
                        }
                        Some(forks)
                    }
                })
                .flatten()
                .collect();
            // println!(
            //     "round {}: journeies: {:?}, paths: {:?}",
            //     i, &journeies, &paths
            // );
            if journeies.is_empty() {
                break;
            }
        }
        paths
    }
    fn path_bfs2(&self) -> Vec<Vec<NodeId>> {
        let mut journeies = vec![vec!["start"]];
        let mut paths = vec![];
        for _i in 0.. {
            journeies = journeies
                .into_iter()
                .filter_map(|j| {
                    let last = j.last().unwrap();
                    if *last == "end" {
                        paths.push(j);
                        return None;
                    }
                    let choices = self
                        .edges
                        .get(last)
                        .unwrap()
                        .iter()
                        .filter(|e| {
                            let not_start = e.to_id != "start";
                            let small_cave_flag = if matches!(
                                self.nodes.get(&e.to_id).unwrap().node_type,
                                NodeType::Small
                            ) {
                                let mut small_so_far: Vec<NodeId> = j
                                    .clone()
                                    .into_iter()
                                    .filter(|nid| {
                                        matches!(
                                            self.nodes.get(nid).unwrap().node_type,
                                            NodeType::Small
                                        )
                                    })
                                    .collect();
                                small_so_far.push(e.to_id);
                                let len = small_so_far.len();
                                let set: std::collections::HashSet<&str> =
                                    small_so_far.into_iter().collect();
                                let len_after = set.len();
                                len - len_after <= 1
                            } else {
                                true
                            };
                            not_start & small_cave_flag
                        })
                        .map(|e| e.to_id);
                    let forks: Vec<Vec<NodeId>> = choices
                        .map(|new_node| {
                            let mut journey_so_far = j.clone();
                            journey_so_far.push(new_node);
                            journey_so_far
                        })
                        .collect();
                    if forks.is_empty() {
                        return None;
                    }
                    Some(forks)
                })
                .flatten()
                .collect();
            // println!(
            //     "round {}: journeies: {:?}, paths: {:?}",
            //     _i, &journeies, &paths
            // );
            if journeies.is_empty() {
                break;
            }
        }
        paths
    }
}

type NodeId<'a> = &'a str;

#[derive(Debug)]
struct Node {
    node_type: NodeType,
}

#[derive(Debug)]
enum NodeType {
    Start,
    End,
    Big,
    Small,
}

#[derive(Debug)]
struct Edge<'a> {
    to_id: NodeId<'a>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_test() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(10, part1(input));
    }
    #[test]
    fn part2_test() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        assert_eq!(36, part2(input));
    }
    #[test]
    fn dedup_test() {
        let mut a = vec![
            "start", "b", "A", "c", "A", "b", "d", "b", "A", "c", "A", "b", "A", "b", "A", "c",
            "A", "end",
        ];
    }
}
