fn read(line: String) -> (String, String) {
    let len = line.chars().count();
    (String::from(&line[0..(len/2)]), String::from(&line[(len/2)..]))
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
        i if i >= 'a' && i <= 'z' => (i as u32) - ('a' as u32) + 1,
        i if i >= 'A' && i <= 'Z' => (i as u32) - ('A' as u32) + 27,
        _ => unimplemented!(),
    }
}


pub fn a(input: impl Iterator<Item = String>) -> u32 {
    input.map(read).map(item).map(score).sum()
}

pub fn b(input: impl Iterator<Item = String>) -> i32 {
    // input.map(read).map(gameb).map(score).sum()
    0
}

