use std::fs::File;
use std::io::{self, BufRead};

fn input01() -> Box<dyn io::BufRead> {
    Box::new(io::BufReader::new(File::open("input/01").unwrap()))
}

#[test]
fn test_01a() {
    assert_eq!(aoc2022::day01::a(input01()), 67658);
}

#[test]
fn test_01b() {
    assert_eq!(aoc2022::day01::b(input01()), 200158);
}

fn input(t: &str) -> Box<dyn Iterator<Item = String>> {
    Box::new(
        io::BufReader::new(File::open(format!("input/{}", t)).unwrap())
            .lines()
            .map(|l| l.unwrap()),
    )
}

#[test]
fn test_02a() {
    assert_eq!(aoc2022::day02::a(input("02")), 14827);
}

#[test]
fn test_02b() {
    assert_eq!(aoc2022::day02::b(input("02")), 13889);
}

#[test]
fn test_03a() {
    assert_eq!(aoc2022::day03::a(input("03")), 8394);
}

#[test]
fn test_03b() {
    assert_eq!(aoc2022::day03::b(input("03")), 2413);
}
