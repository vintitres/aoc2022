use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn d1p1(input: &str) -> i32 {
    super::day01::a(input)
}

#[aoc(day1, part2)]
pub fn d1p2(input: &str) -> i32 {
    super::day01::b(input)
}

#[aoc(day2, part1)]
pub fn d2p1(input: &str) -> i32 {
    super::day02::a(input)
}

#[aoc(day2, part2)]
pub fn d2p2(input: &str) -> i32 {
    super::day02::b(input)
}

#[aoc(day3, part1)]
pub fn d3p1(input: &str) -> i32 {
    super::day03::part1(input)
}

#[aoc(day3, part2)]
pub fn d3p2(input: &str) -> i32 {
    super::day03::part2(input)
}

#[aoc(day4, part1)]
pub fn d4p1(input: &str) -> usize {
    super::day04::part1(input)
}

#[aoc(day4, part2)]
pub fn d4p2(input: &str) -> usize {
    super::day04::part2(input)
}

#[aoc(day5, part1)]
pub fn d5p1(input: &str) -> String {
    super::day05::part1(input)
}

#[aoc(day5, part2)]
pub fn d5p2(input: &str) -> String {
    super::day05::part2(input)
}

#[aoc(day6, part1)]
pub fn d6p1(input: &str) -> usize {
    super::day06::part1(input)
}

#[aoc(day6, part2)]
pub fn d6p2(input: &str) -> usize {
    super::day06::part2(input)
}
