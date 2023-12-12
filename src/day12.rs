use itertools::{repeat_n, Itertools};
use memoize::memoize;

fn check_fit(line: &str, groups: &[usize]) -> bool {
    let g: Vec<_> = line
        .split(|c: char| c == '.')
        .map(|s| s.len())
        .filter(|s| *s > 0)
        .collect();
    g.len() == groups.len() && g.iter().zip(groups).all(|(l, r)| l == r)
}

fn solve_one(line: &str, groups: &[usize]) -> usize {
    line.chars()
        .map(|c| match c {
            '?' => vec!['.', '#'],
            x => vec![x],
        })
        .multi_cartesian_product()
        .map(|x| x.iter().collect::<String>())
        .map(|x| check_fit(&x, groups))
        .filter(|x| *x)
        .count()
}

pub fn parse_input(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let groups = right.split(',').map(|x| x.parse().unwrap()).collect();
            (left.to_string(), groups)
        })
        .collect()
}

pub fn part1(input: Vec<(String, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(line, groups)| solve_one(&line, &groups))
        .sum()
}

pub fn part1_recursive(input: Vec<(String, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(line, groups)| solve_recursive(line.to_string(), groups))
        .sum()
}

#[memoize]
fn solve_recursive(string: String, target: Vec<usize>) -> usize {
    let string = string.trim_start_matches('.');
    if target.is_empty() {
        return match string.contains('#') {
            true => 0,
            false => 1,
        };
    }

    // println!("{sep}string: {}, target {:?}", string, target);
    if !string.contains('?') {
        // nothing to replace
        let splits = string
            .split('.')
            .filter(|x| !x.is_empty())
            .collect::<Vec<_>>();
        match target.len() == splits.len()
            && target.iter().zip(splits.iter()).all(|(t, s)| s.len() == *t)
        {
            true => 1,
            false => 0,
        }
    } else {
        if string.len() < target[0] {
            return 0;
        }
        let mut max_start = string.find('#').unwrap_or(string.len() - target[0]) + 1;
        max_start = max_start.min(string.len() - target[0] + 1);

        let mut res = 0;
        for start in 0..max_start {
            res += if string[start..start + target[0]].chars().all(|c| c != '.') {
                match string.chars().nth(start + target[0]) {
                    Some('#') => 0,
                    Some(_) => solve_recursive(
                        string[start + target[0] + 1..].to_string(),
                        target.clone().into_iter().skip(1).collect(),
                    ),
                    None => match target.len() {
                        1 => 1,
                        _ => 0,
                    },
                }
            } else {
                0
            };
        }
        res
    }
}

pub fn part2(input: Vec<(String, Vec<usize>)>) -> usize {
    input
        .into_iter()
        .map(|(line, groups)| {
            (
                vec![line.clone(), line.clone(), line.clone() ,line.clone(), line].join("?"),
                repeat_n(groups, 5).flatten().collect::<Vec<_>>(),
            )
        })
        .map(|(line, groups)| solve_recursive(line.to_string(), groups))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_check() {
        assert_eq!(check_fit("..##...###.", &[2, 3]), true);
        assert_eq!(check_fit("..##...###.", &[2, 4]), false);
    }

    #[test]
    fn test_solve_recursive() {
        let groups = vec![1usize, 1, 3];
        let line = "???.###";
        assert_eq!(solve_recursive(line.to_string(), groups.clone()), 1);
        let line = ".??..??...?##.".to_string();
        assert_eq!(solve_recursive(line, groups), 4);
        let groups = vec![2, 1];
        let line = "?????".to_string();
        assert_eq!(solve_recursive(line, groups), 3);
        let groups = vec![3, 2, 1];
        let line = "?###????????".to_string();
        assert_eq!(solve_recursive(line, groups), 10);
    }
    #[test]
    fn test_solve_one() {
        let groups = [1usize, 1, 3];
        let line = "???.###";
        assert_eq!(solve_one(line, &groups), 1);
        let line = ".??..??...?##.";
        assert_eq!(solve_one(line, &groups), 4);
        let groups = [3, 2, 1];
        let line = "?###????????";
        assert_eq!(solve_one(line, &groups), 10);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1"
        };
        assert_eq!(part1(parse_input(input)), 21);
    }
    #[test]
    fn test_part1_recursive() {
        let input = indoc! {
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1"
        };
        assert_eq!(part1_recursive(parse_input(input)), 21);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            "???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1"
        };
        assert_eq!(part2(parse_input(input)), 525152);
    }
    #[test]
    fn test_part2_simple() {
        //???.### 1,1,3 - 1 arrangement
        //.??..??...?##. 1,1,3 - 16384 arrangements
        //?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
        //????.#...#... 4,1,1 - 16 arrangements
        //????.######..#####. 1,6,5 - 2500 arrangements
        //?###???????? 3,2,1 - 506250 arrangements

        assert_eq!(part2(parse_input("???.### 1,1,3")), 1);
        assert_eq!(part2(parse_input("?#?#?#?#?#?#?#? 1,3,1,6")), 16384);
    }
}
