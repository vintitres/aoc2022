use std::io;

fn main() {
    let input = Box::new(io::BufReader::new(io::stdin()));
    println!("{:?}", aoc2022::day02::b(input));
}
