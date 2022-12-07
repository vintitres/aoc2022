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
    pub fn at(&mut self, path: &[&str]) -> &mut Node {
        let (next, path) = match path.split_first() {
            None => return self,
            Some(np) => np,
        };
        match self {
            Node::Dir(nodes) => nodes.get_mut(&next.to_string()).unwrap().at(path),
            Node::File(_) => panic!("only enter dirs"),
        }
    }
    pub fn size(&self) -> u32 {
        match self {
            Node::File(size) => *size,
            Node::Dir(nodes) => nodes.iter().map(|(_, n)| n.size()).sum(),
        }
    }

    // TODO learn how to create iterator for this
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
}

fn read(input: &str) -> Node {
    let mut root = Node::Dir(HashMap::new());
    let mut pwd: Vec<&str> = Vec::new();
    for line in input.lines() {
        match line {
            "$ ls" => (),
            "$ cd /" => pwd.clear(),
            "$ cd .." => {
                pwd.pop();
            }
            cmd if cmd.starts_with("$ cd ") => {
                let name = &cmd[5..];
                pwd.push(name);
            }
            node if node.starts_with("dir ") => {
                let name = &node[4..];
                root.at(&pwd).makedir(name)
            }
            node if node.as_bytes()[0].is_ascii_digit() => {
                let (size, name) = node.split(' ').collect_tuple().unwrap();
                root.at(&pwd).touch(name, size.parse().unwrap());
            }
            _ => unimplemented!(),
        }
    }
    root
}

pub fn part1(input: &str) -> u32 {
    read(input).sum_sizes_under(100000)
}

pub fn part2(input: &str) -> u32 {
    let root = read(input);
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
