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

pub fn part2(input: &str) -> u32 {
    let forest = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect_vec()).collect_vec();
    let mut max = 0;
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            let mut seen = (0,0,0,0);
            for ii in i+1..forest.len() {
               if forest[ii][j] < forest[i][j] {
                seen.0 +=1;
               } else {
                seen.0 +=1;
                break;
               }
            }
            for ii in (0..i.checked_div(1).unwrap_or(0)).rev() {
               if forest[ii][j] < forest[i][j] {
                seen.1 +=1;
               } else {
                seen.1 +=1;
                break;
               }
            }
            for jj in j+1..forest[0].len() {
               if forest[i][jj] < forest[i][j] {
                seen.2 +=1;
               } else {
                seen.2 +=1;
                break;
               }
            }
            for jj in (0..j.checked_div(1).unwrap_or(0)).rev() {
                seen.3 +=1;
               if forest[i][jj] >= forest[i][j] {
                break;
               }
            }
            println!("{} {}: {:?}", i, j, seen);
            max = core::cmp::max(max, seen.0*seen.1*seen.2*seen.3);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        include_str!("../input/2022/day8.txt")
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(input()), 1647);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(input()), 392080);
    }
}
