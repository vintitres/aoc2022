use itertools::Itertools;

fn read(line: String) -> (String, String) {
    let len = line.chars().count();
    (
        String::from(&line[0..(len / 2)]),
        String::from(&line[(len / 2)..]),
    )
}

fn item((l, r): (String, String)) -> char {
    for c in l.chars() {
        if r.contains(c) {
            return c;
        }
    }
    unimplemented!();
}

fn score(item: char) -> u32 {
    match item {
        i if i.is_ascii_lowercase() => (i as u32) - ('a' as u32) + 1,
        i if i.is_ascii_uppercase() => (i as u32) - ('A' as u32) + 27,
        _ => unimplemented!(),
    }
}

fn readb(mut group: impl Iterator<Item = String>) -> char {
    let e1 = group.next().unwrap();
    let e2 = group.next().unwrap();
    let e3 = group.next().unwrap();
    for c in e1.chars() {
        if e2.contains(c) && e3.contains(c) {
            return c;
        }
    }
    unimplemented!();
}

pub fn a(input: impl Iterator<Item = String>) -> u32 {
    input.map(read).map(item).map(score).sum()
}

pub fn b(input: impl Iterator<Item = String>) -> u32 {
    input.chunks(3).into_iter().map(readb).map(score).sum()
}
