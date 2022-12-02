use std::fs::File;
use std::io;

fn input() -> Box<dyn io::BufRead> {
    Box::new(io::BufReader::new(File::open("input/02").unwrap()))
}

#[test]
fn test_a() {
    assert_eq!(aoc2022::day02::a(input()), 14827);
}

#[test]
fn test_b() {
    assert_eq!(aoc2022::day02::b(input()), 13889);
}

