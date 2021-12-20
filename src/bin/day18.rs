use std::str::FromStr;

use binary_tree_utils::*;

fn main() {
    let input = include_str!("../../inputs/day18_input");
    println!("day18 part1:{}", part1(input));
    println!("day18 part2:{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut trees = input
        .lines()
        .map(|s| s.trim().parse::<BinaryTree>().unwrap());
    let first_tree = trees.next().unwrap();
    let final_tree = trees.fold(first_tree, |acc, t| {
        let mut tree = acc.merge(t);
        loop {
            if let Some(_) = tree.explode() {
                continue;
            }
            if let Some(_) = tree.split() {
                continue;
            }
            break;
        }
        tree
    });
    final_tree.magnitude(1)
}

fn part2(input: &str) -> u32 {
    let trees = input
        .lines()
        .map(|s| s.trim().parse::<BinaryTree>().unwrap())
        .collect::<Vec<BinaryTree>>();
    let mut magnitudes = vec![];
    for l in 0..trees.len() {
        'inner: for r in 0..trees.len() {
            if l == r {
                continue 'inner;
            }
            let mut sum = trees[l].clone().merge(trees[r].clone());
            sum.process();
            magnitudes.push(sum.magnitude(1));
        }
    }
    magnitudes.iter().max().unwrap().clone()
}

#[derive(Debug, Clone)]
pub struct BinaryTree {
    arr: Vec<Option<Node>>,
}

impl BinaryTree {
    fn process(&mut self) -> Option<()> {
        let mut flag = false;
        loop {
            if let Some(_) = self.explode() {
                flag = true;
                continue;
            }
            if let Some(_) = self.split() {
                flag = true;
                continue;
            }
            break;
        }
        if flag {
            Some(())
        } else {
            None
        }
    }
    fn with_depth(depth: usize) -> Self {
        let arr = vec![None; 2usize.pow(depth as u32)];
        Self { arr }
    }
    fn set(&mut self, idx: usize, node: Node) {
        self.arr[idx] = Some(node);
    }
    fn get(&self, idx: usize) -> Option<&Node> {
        self.arr.get(idx).unwrap().as_ref()
    }
    fn depth(&self) -> usize {
        format!("{:#b}", self.arr.len()).len() - 2 - 1
    }
    fn children(&self, idx: usize) -> Option<(usize, usize)> {
        if node_depth(idx) + 1 > self.depth() {
            // self.extend(node_depth(idx) + 1 - self.depth());
            // Some((idx*2, idx*2+1))
            None
        } else {
            let l = 2 * idx;
            let r = 2 * idx + 1;
            Some((l, r))
        }
    }
    fn post_order_travelsal_from(&self, start: usize) -> impl Iterator<Item = usize> {
        PostOrderIterator {
            idx: start,
            depth: self.depth(),
        }
    }
    fn post_order_traversal(&self) -> impl Iterator<Item = usize> {
        let start = 2usize.pow((self.depth() - 1) as u32);
        self.post_order_travelsal_from(start)
    }
    fn cleanup(mut self) -> BinaryTree {
        for d in 1..=self.depth() {
            if self.arr[2usize.pow(d as u32 - 1)..=2usize.pow(d as u32) - 1]
                .iter()
                .all(|n| n.is_none())
            {
                self.arr = self.arr[..2usize.pow(d as u32 - 1)].to_vec();
            }
        }
        self
    }
    fn merge(self, r: BinaryTree) -> BinaryTree {
        let mut l_layers = {
            let depth = self.depth();
            let mut l_layers = vec![];
            for d in 1..=depth {
                let line_range = 2u32.pow((d as u32) - 1)..2u32.pow(d as u32);
                let line = line_range
                    .map(|idx| self.arr[idx as usize])
                    .collect::<Vec<Option<Node>>>();
                if line.iter().any(|n| n.is_some()) {
                    l_layers.push(line);
                }
            }
            l_layers
        };
        let mut r_layers = {
            let depth = r.depth();
            let mut r_layers = vec![];
            for d in 1..=depth {
                let line_range = 2u32.pow((d as u32) - 1)..2u32.pow(d as u32);
                let line = line_range
                    .map(|idx| r.arr[idx as usize])
                    .collect::<Vec<Option<Node>>>();
                if line.iter().any(|n| n.is_some()) {
                    r_layers.push(line);
                }
            }
            r_layers
        };
        let l_len = l_layers.len();
        let r_len = r_layers.len();
        if l_len != r_len {
            let less_len = l_len.min(r_len);
            let more_len = l_len.max(r_len);
            let diff = (less_len + 1..=more_len)
                .map(|depth| vec![None; 2usize.pow(depth as u32 - 1)])
                .collect::<Vec<Vec<Option<Node>>>>();
            if l_len < r_len {
                l_layers.extend(diff);
            } else {
                r_layers.extend(diff);
            }
        }
        let mut arr = vec![None, Some(Node::Branch)];
        arr.extend(
            l_layers
                .into_iter()
                .zip(r_layers)
                .map(|(l, r)| {
                    l.into_iter()
                        .chain(r.into_iter())
                        .collect::<Vec<Option<Node>>>()
                })
                .flatten(),
        );
        BinaryTree { arr }.cleanup()
    }
    fn children_recursive(&self, idx: usize) -> Vec<usize> {
        let mut current_parent = vec![idx];
        let mut children = vec![];
        loop {
            let this_gen = current_parent
                .iter()
                .map(|i| [i * 2, i * 2 + 1])
                .flatten()
                .collect::<Vec<usize>>();
            if this_gen[0] >= self.arr.len() {
                break;
            }
            children.extend_from_slice(&this_gen);
            current_parent = this_gen;
        }
        children
    }
    fn node_value(&self, idx: usize) -> Option<u32> {
        if let Some(Node::Leaf(v)) = self.arr[idx] {
            Some(v)
        } else {
            None
        }
    }
    fn explode(&mut self) -> Option<()> {
        let traversal = self.post_order_traversal();
        let mut leaves = vec![];
        for idx in traversal {
            if let Some(n) = self.get(idx) {
                if matches!(n, &Node::Leaf(_value)) {
                    leaves.push(idx);
                    if idx % 2 == 0 {}
                }
            } else {
                continue;
            }
        }
        let (l, pair, r) = check_pair(&leaves);
        if let Some((l_half, r_half)) = pair {
            let (vlh, vrh) = (
                self.node_value(l_half).unwrap(),
                self.node_value(r_half).unwrap(),
            );
            let parent = parent(l_half).unwrap();
            // parent set to Leaf(0)
            self.set(parent, Node::Leaf(0));
            // parent's children set to none
            for i in self.children_recursive(parent) {
                self.arr[i] = None;
            }
            // l set to v + l_half if exist
            if let Some(idx_l) = l {
                let vl = self.node_value(idx_l).unwrap();
                self.set(idx_l, Node::Leaf(vlh + vl));
            }
            // r set to v + r_half if exist
            if let Some(idx_r) = r {
                let vr = self.node_value(idx_r).unwrap();
                self.set(idx_r, Node::Leaf(vrh + vr));
            }
            Some(())
        } else {
            None
        }
    }
    fn split(&mut self) -> Option<()> {
        let leaves = self
            .post_order_traversal()
            .filter(|&i| matches!(self.arr[i], Some(Node::Leaf(_))))
            .collect::<Vec<usize>>();
        if let Some(i) = leaves
            .into_iter()
            .find(|&i| self.node_value(i).unwrap() >= 10)
        {
            let value = self.node_value(i).unwrap();
            self.set(i, Node::Branch);
            let (vl, vr) = (value / 2, value - (value / 2));
            let (idx_l, idx_r) = self.children(i).unwrap();
            self.set(idx_l, Node::Leaf(vl));
            self.set(idx_r, Node::Leaf(vr));
            Some(())
        } else {
            None
        }
    }
    fn magnitude(&self, idx: usize) -> u32 {
        match self.arr[idx] {
            Some(n) => match n {
                Node::Branch => {
                    let (l, r) = (idx * 2, idx * 2 + 1); // do not use fn children(&mut self) because here assume the tree is well extended to contain all children
                    self.magnitude(l) * 3 + self.magnitude(r) * 2
                }
                Node::Leaf(v) => v,
            },
            None => 0,
        }
    }
}

impl FromStr for BinaryTree {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let depth = {
            let mut stacks = vec![];
            let mut max_layer = 0;
            for c in s.chars() {
                match c {
                    '[' => {
                        stacks.push('[');
                        let mut count = 0;
                        'tail: for &c in stacks.iter().rev() {
                            if c == '[' {
                                count += 1;
                            } else {
                                break 'tail;
                            }
                        }
                        max_layer = max_layer.max(count);
                    }
                    ']' => {
                        let pop = stacks.pop().expect("pop ] when stack is empty");
                        assert_eq!('[', pop);
                    }
                    _ => continue,
                }
            }
            max_layer + 1
        };
        let mut tree = BinaryTree::with_depth(depth);
        let mut cursor = 1;
        for c in s.chars() {
            match c {
                '[' => {
                    tree.set(cursor, Node::Branch);
                    cursor = cursor * 2;
                    // add empty new node to current place
                    // move cursor to current place's left
                }
                ']' => {
                    if let Some(p) = parent(cursor) {
                        cursor = p;
                    } else {
                        break;
                    }
                    //move cursor
                }
                ',' => {
                    // move cursor to current place's right
                    assert!(left(cursor));
                    cursor = cursor + 1;
                }
                i => {
                    // current place is a leaf
                    let n = (i as u8 - 48) as u32;
                    tree.set(cursor, Node::Leaf(n));
                }
            }
        }
        Ok(tree)
    }
}

mod binary_tree_utils {
    use super::*;
    pub fn node_depth(idx: usize) -> usize {
        format!("{:#b}", (idx + 1).next_power_of_two()).len() - 2 - 1
    }

    pub fn left(idx: usize) -> bool {
        idx % 2 == 0
    }
    pub fn parent(idx: usize) -> Option<usize> {
        if idx == 0 {
            unreachable!()
        }
        if idx == 1 {
            return None;
        }
        if idx % 2 == 0 {
            Some(idx / 2)
        } else {
            Some((idx - 1) / 2)
        }
    }
    pub fn check_pair(leaves: &[usize]) -> (Option<usize>, Option<(usize, usize)>, Option<usize>) {
        if leaves.len() < 2 {
            return (None, None, None);
        }
        for i in 1..leaves.len() {
            let r = leaves[i];
            let l = leaves[i - 1];
            let pair = if (l + 1 == r)
                & (node_depth(l) == node_depth(r))
                & ((node_depth(l) > 5) & left(l))
            {
                Some((l, r))
            } else {
                None
            };
            if pair.is_some() {
                let left = if i - 1 > 0 { Some(leaves[i - 2]) } else { None };
                let right = if i + 1 < leaves.len() {
                    Some(leaves[i + 1])
                } else {
                    None
                };
                return (left, pair, right);
            }
        }
        (None, None, None)
    }
    #[allow(dead_code)]
    pub fn print_tree(tree: &BinaryTree) {
        for i in 1..=tree.depth() {
            let line_range = 2u32.pow((i as u32) - 1)..2u32.pow(i as u32);
            for node in line_range.map(|idx| tree.get(idx as usize)) {
                match node {
                    Some(n) => match n {
                        Node::Branch => print!(" | "),
                        Node::Leaf(num) => print!(" {} ", num),
                    },
                    None => print!(" x "),
                }
            }
            println!("");
        }
    }
}
#[cfg(test)]
mod utils_tests {
    use super::*;
    #[test]
    fn check_pair_test() {
        let input = "[[[[[9,8],1],2],3],4]";
        let tree = BinaryTree::from_str(input).unwrap();
        let mut leaves = vec![];
        for n in tree.post_order_traversal() {
            if matches!(tree.get(n), Some(&Node::Leaf(_))) {
                leaves.push(n);
            }
        }
        let (l, pair, r) = check_pair(&leaves);
        assert_eq!(None, l);
        assert_eq!(Some((32, 33)), pair);
        assert_eq!(Some(17), r);
    }
}
struct PostOrderIterator {
    idx: usize,
    depth: usize,
}

impl Iterator for PostOrderIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        let current = self.idx.clone();
        match idx {
            0 => return None,
            1 => {
                self.idx -= 1;
            }
            i if i % 2 != 0 => {
                self.idx = (i - 1) / 2;
            }
            i => {
                let r = i + 1;
                let r_depth = node_depth(i);
                self.idx = r * (2usize.pow((self.depth - r_depth) as u32));
            }
        }
        Some(current)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Node {
    Branch,
    Leaf(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_test() {
        let tree = BinaryTree {
            arr: vec![None; 2usize.pow(4)],
        };
        assert_eq!(4, tree.depth());
        let idx: usize = 5;
        let depth = node_depth(idx);
        assert_eq!(3, depth);
        let idx = 8;
        assert_eq!(4, node_depth(idx));
    }
    #[test]
    fn family_test() {
        let tree = BinaryTree {
            arr: vec![None; 2usize.pow(8) - 1],
        };
        let idx = 3;
        let children = tree.children(idx).unwrap();
        assert_eq!((6, 7), children);
        let p = parent(idx).unwrap();
        assert_eq!(1, p);
        let p = parent(1);
        assert_eq!(None, p);
        let p = parent(4).unwrap();
        assert_eq!(2, p);
    }
    #[test]
    fn post_order_travel_test() {
        let tree = BinaryTree {
            arr: vec![
                None,
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Branch),
                Some(Node::Leaf(4)),
                Some(Node::Leaf(12)),
                Some(Node::Leaf(18)),
                Some(Node::Leaf(24)),
                Some(Node::Leaf(31)),
                Some(Node::Leaf(44)),
                Some(Node::Leaf(66)),
                Some(Node::Leaf(90)),
            ],
        };
        let got = tree.post_order_traversal().collect::<Vec<usize>>();
        assert_eq!(vec![8, 9, 4, 10, 11, 5, 2, 12, 13, 6, 14, 15, 7, 3, 1], got);
    }
    #[test]
    fn explode_test() {
        let input = "[[[[[9,8],1],2],3],4]";
        let mut tree = BinaryTree::from_str(input).unwrap();
        tree.explode().unwrap();
        tree = tree.cleanup();
        let expect = BinaryTree::from_str("[[[[0,9],2],3],4]").unwrap();
        assert_eq!(tree.arr, expect.arr);
    }
    #[test]
    fn magnitude_test() {
        let tree = BinaryTree::from_str("[9,1]").unwrap();
        assert_eq!(tree.magnitude(1), 29);
        let tree = BinaryTree::from_str("[[9,1],[1,9]]").unwrap();
        assert_eq!(tree.magnitude(1), 129);
        let tree =
            BinaryTree::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
                .unwrap();
        assert_eq!(4140, tree.magnitude(1));
    }
    #[test]
    fn part1_test() {
        let input = include_str!("../../inputs/day18_test");
        assert_eq!(4140, part1(input));
    }
    #[test]
    fn part2_test() {
        let input = include_str!("../../inputs/day18_test");
        assert_eq!(3993, part2(input));
    }
}
