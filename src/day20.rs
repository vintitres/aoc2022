use itertools::Itertools;

fn negmod(x: i64, m: usize) -> usize {
    if x >= 0 {
        ((x as u64) % (m as u64)).try_into().unwrap()
    } else {
        (x + ((-x) / (m as i64) + 1) * (m as i64))
            .try_into()
            .unwrap()
    }
}

pub fn part1(input: &str) -> i64 {
    decrypt(input, 1, 1)
}

fn decrypt(input: &str, key: i64, times: usize) -> i64 {
    let mut file: Vec<(usize, i64)> = input
        .lines()
        .map(|e| e.parse::<i64>().unwrap() * key)
        .enumerate()
        .collect_vec();
    let ll = file.len();
    for index_to_move in (0..ll).cycle().take(ll * times) {
        let i = file
            .iter()
            .position(|&(index, _)| index == index_to_move)
            .unwrap();
        let val = file[i].1;
        let j = negmod(i as i64 + val, ll - 1);
        file.remove(i);
        file.insert(j, (index_to_move, val));
    }
    let zeropos = file.iter().position(|&(_, val)| val == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|i| file[(i + zeropos) % ll].1)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    decrypt(input, 811589153, 10)
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
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 8927480683);
    }
}
