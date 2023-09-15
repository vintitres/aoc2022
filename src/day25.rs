use itertools::Itertools;

const SNAFU_BASE: i8 = 5;
struct SNAFU {
    digits: Vec<i8>,
}

impl SNAFU {
    fn parse_digit(c: char) -> i8 {
        match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unexpected char {}", c),
        }
    }
    fn parse(input: &str) -> Self {
        Self {
            digits: input
                .chars()
                .rev()
                .map(|c| Self::parse_digit(c))
                .collect_vec(),
        }
    }
    fn to_dec(&self) -> i64 {
        let mut p: i64 = 1;
        let mut val = 0;
        for d in self.digits.iter() {
            val += p * (*d as i64);
            p *= SNAFU_BASE as i64;
        }
        val
    }
    fn inc(&mut self) {
        let mut added = false;
        for d in &mut self.digits {
            if !added && *d == 2 {
                *d = -2;
            } else if !added {
                *d += 1;
                added = true;
                break;
            }
        }
        if !added {
            self.digits.push(2);
        }
    }
    fn zero() -> Self {
        Self { digits: vec![0] }
    }
    fn neg(&mut self) {
        for d in &mut self.digits {
            *d *= -1;
        }
    }
    fn push(&mut self, v: i8) {
        match v {
            1 | 2 => self.digits.push(v),
            _ => panic!("bad push digit value: {}", v),
        }
    }
    fn from_dec(n: i64) -> Self {
        if n == 0 {
            return Self::zero();
        }
        if n < 0 {
            let mut s = Self::from_dec(-n);
            s.neg();
            return s;
        }
        // TODO analyze this
        let mut p: i64 = 1;
        let mut s = 0;
        while s < n {
            s += 2 * p;
            p *= SNAFU_BASE as i64;
        }
        p /= SNAFU_BASE as i64;
        s -= p;
        if s < n {
            let mut ss = Self::from_dec(n - s);
            ss.push(2);
            return ss;
        }
        s -= p;
        let mut ss = Self::from_dec(n - s);
        ss.push(1);
        return ss;

        // 1bits: 2 * 5^0 =   2    2
        // 2bits: 2 * 5^1 =  10   12
        // 3bits: 2 * 5^2 =  50   62
        // 4bits: 2 * 5^3 = 250  312
        // 5bits: 2 * 5^4 =1250 1562

        // 10    20
        // 11    21
        // 12    22
        // 13   1==
        // 14   1=-
        // 15   1=0
        // 16   1=1
        // 17   1=2
        // 18   1-=
        // 19   1--
        // 20   1-0
        // 21   1-1
        // 22   1-2
        // 23   10=
        // 24   10-
        // 25   100
        // 26   101
        // 27   102
        // 28   11=
        // 29   11-
        // 30   110
    }
    fn to_str(&self) -> String {
        self.digits
            .iter()
            .rev()
            .map(|d| match d {
                -2 => '=',
                -1 => '-',
                0 => '0',
                1 => '1',
                2 => '2',
                _ => panic!("unexpected SNAFTU digit val: {}", d),
            })
            .collect()
    }
}

pub fn part1(input: &str) -> String {
    let fuelsum: i64 = input.lines().map(|l| SNAFU::parse(l).to_dec()).sum();
    // fuelsum.to_string()
    SNAFU::from_dec(fuelsum).to_str()
}

pub fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day25e.txt")
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), "");
    }

    #[ignore = "not implemented"]
    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 4);
    }
}
