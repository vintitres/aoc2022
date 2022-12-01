use std::fs::File;
use std::io;

fn input() -> Box<dyn io::BufRead> {
    Box::new(io::BufReader::new(File::open("input/01").unwrap()))
}

#[test]
fn test_a() {
    assert_eq!(aoc2022::day01::a(input()), 67658);
}

#[test]
fn test_b() {
    assert_eq!(aoc2022::day01::b(input()), 200158);
}
