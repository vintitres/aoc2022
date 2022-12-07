use core::panic;
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
enum Node {
    Dir(HashMap<String, Node>),
    File(u32),
}

impl Node {
    pub fn makedir(&mut self, name: &str) {
        match self {
            Node::Dir(nodes) => {
                nodes
                    .entry(name.to_string())
                    .or_insert_with(|| Node::Dir(HashMap::new()));
            }
            _ => panic!("only create in directory"),
        }
    }
    pub fn touch(&mut self, name: &str, size: u32) {
        match self {
            Node::Dir(nodes) => {
                nodes
                    .entry(name.to_string())
                    .or_insert_with(|| Node::File(size));
            }
            _ => panic!("only create in directory"),
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            Node::File(size) => *size,
            Node::Dir(nodes) => nodes.iter().map(|(_, n)| n.size()).sum(),
        }
    }

    // TODO learn how to create iterator over the file tree
    /*
    pub fn empty() -> impl Iterator<Item = Node> {
        std::iter::empty()
    }
    pub fn chain(l: impl Iterator<Item = Node>, r: impl Iterator<Item = Node>) -> impl Iterator<Item = Node> {
        l.chain(r)
    }
    pub fn iter_dirs(&self) -> impl Iterator<Item = Node> {
        match self {
            Node::File(_) => Node::empty(),
            Node::Dir(nodes) => Node::chain(
                std::iter::once(self),
                nodes.iter().fold(
                    Node::empty(),
                    |i, (_, n)| Node::chain(i, n.iter_dirs())
                )
            )

        }
    }
    */

    fn sum_sizes_under(&self, size_limit: u32) -> u32 {
        match self {
            Node::File(_) => 0,
            Node::Dir(nodes) => {
                let size = self.size();
                nodes
                    .iter()
                    .map(|(_, n)| n.sum_sizes_under(size_limit))
                    .sum::<u32>()
                    + if size <= size_limit { size } else { 0 }
            }
        }
    }

    fn find_smallest_over(&self, size_limit: u32) -> u32 {
        match self {
            Node::File(_) => u32::MAX,
            Node::Dir(nodes) => core::cmp::min(
                match self.size() {
                    size if size < size_limit => u32::MAX,
                    size => size,
                },
                nodes
                    .iter()
                    .map(|(_, n)| n.find_smallest_over(size_limit))
                    .min()
                    .unwrap(),
            ),
        }
    }

    fn read(&mut self, lines: &mut std::str::Lines) {
        loop {
            match lines.next() {
                Some("$ ls") => (),
                Some("$ cd ..") => return,
                Some("$ cd /") => (), // assuming not called after first line
                Some(cmd) if cmd.starts_with("$ cd ") => {
                    let name = &cmd[5..];
                    self.makedir(name);
                    match self {
                        Node::Dir(nodes) => nodes.get_mut(name).unwrap(),
                        Node::File(_) => panic!(),
                    }
                    .read(lines)
                }
                Some(node) if node.starts_with("dir ") => (), // dirs only matter if we enter (and ls inside)
                Some(node) if node.as_bytes()[0].is_ascii_digit() => {
                    let (size, name) = node.split(' ').collect_tuple().unwrap();
                    self.touch(name, size.parse().unwrap());
                }
                Some(_) => unimplemented!(),
                None => return,
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let mut root = Node::Dir(HashMap::new());
    root.read(&mut input.lines());
    root.sum_sizes_under(100000)
}

pub fn part2(input: &str) -> u32 {
    let mut root = Node::Dir(HashMap::new());
    root.read(&mut input.lines());
    root.find_smallest_over(30000000 - (70000000 - root.size()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day7.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1648397);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1815525);
    }
}
