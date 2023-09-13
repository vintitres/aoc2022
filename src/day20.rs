use itertools::Itertools;

#[derive(Debug)]
struct Elem {
    val: i32,
    moved: bool,
}

impl Elem {
    fn read(sval: &str) -> Elem {
        Elem {
            val: sval.parse().unwrap(),
            moved: false,
        }
    }
}

fn negmod(x: i32, m: usize) -> usize {
    if x >= 0 {
        ((x as u32) % (m as u32)).try_into().unwrap()
    } else {
        (x + ((-x) / (m as i32) + 1) * (m as i32))
            .try_into()
            .unwrap()
    }
}
pub fn part1(input: &str) -> i32 {
    let mut file = input.lines().map(Elem::read).collect_vec();
    let ll = file.len();
    for _ in 0..ll {
        let mut i = 0;
        let mut j = ll;
        while i < ll {
            if !file[i].moved {
                j = negmod(i as i32 + file[i].val, ll - 1);
                break;
            }
            i += 1;
        }
        let v = file[i].val;
        file.remove(i);
        file.insert(
            j,
            Elem {
                val: v,
                moved: true,
            },
        );
        // eprintln!("{:?}", file);
    }
    let zeropos = file.iter().position(|f| f.val == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| file[(i + zeropos) % ll].val)
        .sum()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day20.txt")
    }
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 2215);
    }
    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 1234);
    }
}
