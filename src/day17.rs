const ROCKS: usize = 2022;
const WIDTH: usize = 7;
const HEIGHT: usize = (ROCKS + 1) * 4;

pub fn part1(input: &str) -> usize {
    let mut chamber = [[false; WIDTH]; HEIGHT];
    let mut height: usize = 0;

    let mut blows = input.trim().chars().cycle();
    for rock_num in 0..ROCKS {
        // ROCKS
        let mut rock = get_rock(rock_num, height);
        loop {
            let new_rock = move_rock(
                &rock,
                (0, if blows.next().unwrap() == '<' { -1 } else { 1 }),
            );
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
            // eprintln!("{:?}", rock[0])
        }
        // print_chamber(&chamber, height);
    }
    height
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
    usize_rock(&rock).iter().all(|pos| match pos {
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
    usize_rock(&rock).iter().for_each(|pos| match pos {
        Some(pos) => chamber[pos.0][pos.1] = true,
        None => {}
    });
}

fn get_rock(rock_num: usize, height: usize) -> Rock {
    let height: isize = (height + 3).try_into().unwrap();
    match rock_num % 5 {
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
    eprintln!("");
}

pub fn part2(input: &str) -> usize {
    input.len()
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

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1234);
    }
}
