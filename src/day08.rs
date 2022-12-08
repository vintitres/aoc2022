use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    let mut forest = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();
    let f = |tree: &mut i32, max: &mut i32| {
        if tree.abs() > *max {
            *max = tree.abs();
            if *tree >= 0 {
                *tree *= -1;
            }
        }
    };
    for treerow in forest.iter_mut() {
        let mut max = -1;
        for tree in treerow.iter_mut() {
            f(tree, &mut max);
        }
        max = -1;
        for tree in treerow.iter_mut().rev() {
            f(tree, &mut max);
        }
    }
    for j in 0..forest[0].len() {
        let mut max = -1;
        for treerow in forest.iter_mut() {
            f(&mut treerow[j], &mut max);
        }
        max = -1;
        for treerow in forest.iter_mut().rev() {
            f(&mut treerow[j], &mut max);
        }
    }
    let mut visible = 0;
    for (i, treerow) in forest.iter().enumerate() {
        for (j, tree) in treerow.iter().enumerate() {
            if *tree < 0
                || (*tree == 0
                    && (i == 0 || i == forest.len() - 1 || j == 0 || j == forest[0].len() - 1))
            {
                visible += 1;
            }
        }
    }
    visible
}

pub fn part2(input: &str) -> u32 {
    let forest = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec();
    let mut max = 0;
    for i in 0..forest.len() {
        for j in 0..forest[0].len() {
            let mut seen = (0, 0, 0, 0);
            for ii in i + 1..forest.len() {
                if forest[ii][j] < forest[i][j] {
                    seen.0 += 1;
                } else {
                    seen.0 += 1;
                    break;
                }
            }
            for ii in (0..i.checked_div(1).unwrap_or(0)).rev() {
                if forest[ii][j] < forest[i][j] {
                    seen.1 += 1;
                } else {
                    seen.1 += 1;
                    break;
                }
            }
            for jj in j + 1..forest[0].len() {
                if forest[i][jj] < forest[i][j] {
                    seen.2 += 1;
                } else {
                    seen.2 += 1;
                    break;
                }
            }
            for jj in (0..j.checked_div(1).unwrap_or(0)).rev() {
                seen.3 += 1;
                if forest[i][jj] >= forest[i][j] {
                    break;
                }
            }
            println!("{} {}: {:?}", i, j, seen);
            max = core::cmp::max(max, seen.0 * seen.1 * seen.2 * seen.3);
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
