use std::io::{self, BufRead};

fn main() {
    println!(
        "{}",
        aoc2022::day03::b(io::BufReader::new(io::stdin()).lines().map(|l| l.unwrap()))
    );
}
