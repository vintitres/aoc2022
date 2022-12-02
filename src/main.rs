use std::io::{self, BufRead};

fn main() {
    let input = Box::new(io::BufReader::new(io::stdin()).lines().map(|l| l.unwrap()));
    println!("{:?}", aoc2022::day02::b(input));
}
