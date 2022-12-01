pub mod day01;

use std::io;

fn main() {
    let input = Box::new(io::BufReader::new(io::stdin()));
    println!("{:?}", day01::a(input));
}
