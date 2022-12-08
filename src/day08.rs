use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut forest = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect_vec()).collect_vec();
    let mut visible = forest.len() * 2 + (forest[0].len() - 2) * 2;
    for i in 1..forest.len()-1 {
        let mut max = forest[i][0];
        for j in 1..forest[0].len()-1 {
            if forest[i][j].abs() > max {
                max = forest[i][j].abs();
                if forest[i][j] >= 0 {
                    forest[i][j] *= -1;
                    visible += 1;
                }
            }
        }
        max = forest[i][forest[0].len()-1];
        for j in (1..forest[0].len()-1).rev() {
            println!("{}", j);
            if forest[i][j].abs() > max {
                max = forest[i][j].abs();
                if forest[i][j] >= 0 {
                    forest[i][j] *= -1;
                    visible += 1;
                }
            } 
        }
    }
    for j in 1..forest[0].len()-1 {
        let mut max = forest[0][j];
        for i in 1..forest.len()-1 {
            if forest[i][j].abs() > max {
                max = forest[i][j].abs();
                if forest[i][j] >= 0 {
                    forest[i][j] *= -1;
                    visible += 1;
                }
            } else {
            }
        }
        max = forest[forest.len()-1][j];
        for i in (1..forest.len()-1).rev() {
            if forest[i][j].abs() > max {
                max = forest[i][j].abs();
                if forest[i][j] >= 0 {
                    forest[i][j] *= -1;
                    visible += 1;
                }
            } else {
            }
        }
    }
    println!("{:?}", forest);
    visible

}

pub fn part2(_input: &str) -> u32 {
    1 + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 3);
    }
}
