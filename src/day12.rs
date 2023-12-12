use itertools::{repeat_n, Itertools};

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
}
