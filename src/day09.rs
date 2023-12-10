use crate::parsers::neg_num;
use itertools::Itertools;
use winnow::ascii::line_ending;
use winnow::combinator::{repeat, separated, terminated};
use winnow::{PResult, Parser};

pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut input = input;
    repeat(1.., terminated(line, line_ending))
        .parse_next(&mut input)
        .unwrap()
}

fn line(input: &mut &str) -> PResult<Vec<isize>> {
    separated(1.., neg_num::<isize>, ' ').parse_next(input)
}

fn compute_line(nums: &[isize]) -> isize {
    let mut nums: Vec<isize> = Vec::from(nums);
    let mut last = *nums.last().unwrap();
    while !nums.iter().all(|&x| x == 0) {
        nums = nums
            .into_iter()
            .tuple_windows()
            .map(|(x, y)| y - x)
            .collect();
        last += *nums.last().unwrap();
    }
    last
}

pub fn part1(input: Vec<Vec<isize>>) -> isize {
    input.iter().map(|x| compute_line(x)).sum()
}

pub fn part2(input: Vec<Vec<isize>>) -> isize {
    part1(
        input
            .into_iter()
            .map(|x| x.into_iter().rev().collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_compute_line() {
        assert_eq!(compute_line(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(compute_line(&[1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(compute_line(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "-1 -2 3 4 -5 6 7 8 9 10
        1 2 -3
        "};
        let expected = vec![vec![-1, -2, 3, 4, -5, 6, 7, 8, 9, 10], vec![1, 2, -3]];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_line() {
        let mut input = "1 2 -3";
        let expected = vec![1, 2, -3];
        assert_eq!(line(&mut input).unwrap(), expected);
    }
}
