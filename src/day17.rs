use std::collections::HashMap;

const WIDTH: usize = 7;
const HEIGHT: usize = 1000;
const ROCKS: u64 = 2022;
const ROCKS2: u64 = 1000000000000;

pub fn part1(input: &str) -> u64 {
    tetris(input, ROCKS)
}

fn tetris(input: &str, rocks: u64) -> u64 {
    let input = input.trim();
    let blows_len = input.len();

    let mut chamber = [[false; WIDTH]; HEIGHT];
    let mut base_height: u64 = 0;
    let mut height: usize = 0;

    let mut blows = input.chars().cycle();
    let mut seen: HashMap<(usize, usize), (usize, u64, u64)> = HashMap::new();
    let mut blow_num: usize = 0;
    let mut rock_num: u64 = 0;
    let mut skipped = false;
    while rock_num < rocks {
        let rock_type: usize = usize::try_from(rock_num % 5).unwrap();
        if !skipped {
            // cycle detection
            let wrs = (rock_type, blow_num);
            let real_height = base_height + u64::try_from(height).unwrap();
            if let Some((2, old_height, old_rock_num)) = seen.get(&wrs) {
                let cycle_rock_length = rock_num - old_rock_num;
                let cycle_height = real_height - old_height;
                let skip_cycles = (rocks - rock_num) / cycle_rock_length;
                rock_num += skip_cycles * cycle_rock_length;
                base_height += skip_cycles * cycle_height;
                skipped = true;
            } else {
                seen.entry(wrs)
                    .and_modify(|(old_count, old_height, old_rock_num)| {
                        *old_count += 1;
                        *old_height = real_height;
                        *old_rock_num = rock_num;
                    })
                    .or_insert((1, real_height, rock_num));
            }
        }
        let mut rock = get_rock(rock_type, height);
        rock_num += 1;
        loop {
            let new_rock = move_rock(
                &rock,
                (0, if blows.next().unwrap() == '<' { -1 } else { 1 }),
            );
            blow_num += 1;
            if blow_num == blows_len {
                blow_num = 0;
            }
            if check_rock(chamber, &new_rock) {
                rock = new_rock;
            }
            let new_rock = move_rock(&rock, (-1, 0));
            if check_rock(chamber, &new_rock) {
                rock = new_rock;
            } else {
                fill_rock(&mut chamber, &rock);
                height = std::cmp::max(
                    height,
                    rock.iter()
                        .map(|pos| pos.0 + 1)
                        .max()
                        .unwrap()
                        .try_into()
                        .unwrap(),
                );
                break;
            }
            if height > HEIGHT - 10 {
                let shift = HEIGHT / 2;
                height -= shift;
                base_height += u64::try_from(shift).unwrap();
                for x in shift..HEIGHT {
                    for y in 0..WIDTH {
                        chamber[x - shift][y] = chamber[x][y];
                        chamber[x][y] = false;
                    }
                }
                rock = move_rock(&rock, (-isize::try_from(shift).unwrap(), 0));
            }
            // eprintln!("{:?}", rock[0])
        }
        // print_chamber(&chamber, height);
    }
    print_chamber(&chamber, height);
    base_height + u64::try_from(height).unwrap()
}

type Pos = (isize, isize);
type Rock = Vec<Pos>;
type Chamber = [[bool; WIDTH]; HEIGHT];

fn move_rock(rock: &Rock, by: Pos) -> Rock {
    rock.iter()
        .map(|pos| (pos.0 + by.0, pos.1 + by.1))
        .collect()
}

fn check_rock(chamber: Chamber, rock: &Rock) -> bool {
    usize_rock(rock).iter().all(|pos| match pos {
        None => false,
        Some(pos) => !chamber[pos.0][pos.1],
    })
}

fn usize_rock(rock: &Rock) -> Vec<Option<(usize, usize)>> {
    rock.iter()
        .map(|pos| {
            if pos.0 >= 0 && pos.1 >= 0 && pos.1 < WIDTH.try_into().unwrap() {
                Some((
                    usize::try_from(pos.0).unwrap(),
                    usize::try_from(pos.1).unwrap(),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn fill_rock(chamber: &mut Chamber, rock: &Rock) {
    usize_rock(rock).iter().for_each(|pos| match pos {
        Some(pos) => chamber[pos.0][pos.1] = true,
        None => {}
    });
}

fn get_rock(rock_type: usize, height: usize) -> Rock {
    let height: isize = (height + 3).try_into().unwrap();
    match rock_type {
        0 => vec![(height, 2), (height, 3), (height, 4), (height, 5)], // --
        1 => vec![
            (height, 3),
            (height + 1, 2),
            (height + 1, 3),
            (height + 1, 4),
            (height + 2, 3),
        ], // +
        2 => vec![
            (height, 2),
            (height, 3),
            (height, 4),
            (height + 1, 4),
            (height + 2, 4),
        ], // _|
        3 => vec![
            (height, 2),
            (height + 1, 2),
            (height + 2, 2),
            (height + 3, 2),
        ], // |
        4 => vec![(height, 2), (height, 3), (height + 1, 2), (height + 1, 3)], // []
        _ => unimplemented!(),
    }
}

fn print_chamber(chamber: &Chamber, height: usize) {
    for x in (0..=height).rev() {
        eprintln!(
            "{:}",
            chamber[x]
                .iter()
                .map(|pos| if *pos { '#' } else { '.' })
                .collect::<String>()
        );
    }
    eprintln!("--------");
    eprintln!();
}

pub fn part2(input: &str) -> u64 {
    tetris(input, ROCKS2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day17.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 3151);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1560919540245);
    }
}
