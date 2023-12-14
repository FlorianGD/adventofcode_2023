use crate::helpers::transpose;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
type Matrix<T> = Vec<Vec<T>>;
use memoize::memoize;

pub fn parse_input(input: &str) -> Matrix<char> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn move_left(s: &str) -> String {
    let mut new_string = s.to_string();
    static BLOCKS: Lazy<Regex> = Lazy::new(|| Regex::new(r"((?:[.]*O+[.]*)+)").unwrap());
    let groups = BLOCKS.captures_iter(s);
    for group in groups {
        let block = group.get(0).unwrap();
        let range = block.range();
        let num_o = s[range.clone()].chars().filter(|&c| c == 'O').count();
        let new = format!(
            "{}{}",
            "O".repeat(num_o),
            ".".repeat(range.clone().len() - num_o),
        );
        new_string.replace_range(range, &new);
    }
    new_string
}

fn move_right(s: &str) -> String {
    let mut new_string = s.to_string();
    static BLOCKS: Lazy<Regex> = Lazy::new(|| Regex::new(r"((?:[.]*O+[.]*)+)").unwrap());
    let groups = BLOCKS.captures_iter(s);
    for group in groups {
        let block = group.get(0).unwrap();
        let range = block.range();
        let num_o = s[range.clone()].chars().filter(|&c| c == 'O').count();
        let new = format!(
            "{}{}",
            ".".repeat(range.clone().len() - num_o),
            "O".repeat(num_o),
        );
        new_string.replace_range(range, &new);
    }
    new_string
}

#[memoize]
fn north(m: Matrix<char>) -> Matrix<char> {
    let m = transpose(m);
    let mut new = Vec::with_capacity(m.len());
    for line in m {
        new.push(
            move_left(&line.iter().collect::<String>())
                .chars()
                .collect_vec(),
        );
    }
    transpose(new)
}
#[memoize]
fn south(m: Matrix<char>) -> Matrix<char> {
    let m = transpose(m);
    let mut new = Vec::with_capacity(m.len());
    for line in m {
        new.push(
            move_right(&line.iter().collect::<String>())
                .chars()
                .collect_vec(),
        );
    }
    transpose(new)
}

#[memoize]
fn east(m: Matrix<char>) -> Matrix<char> {
    let mut new = Vec::with_capacity(m.len());
    for line in m {
        new.push(
            move_right(&line.iter().collect::<String>())
                .chars()
                .collect_vec(),
        );
    }
    new
}

#[memoize]
fn west(m: Matrix<char>) -> Matrix<char> {
    let mut new = Vec::with_capacity(m.len());
    for line in m {
        new.push(
            move_left(&line.iter().collect::<String>())
                .chars()
                .collect_vec(),
        );
    }
    new
}

fn cycle(m: Matrix<char>) -> Matrix<char> {
    east(south(west(north(m))))
}

fn score(m: Matrix<char>) -> usize {
    m.into_iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.into_iter().filter(|x| x == &'O').count() * (i + 1))
        .sum()
}

pub fn part1(input: Matrix<char>) -> usize {
    score(north(input))
}

fn to_string(m: &Matrix<char>) -> String {
    m.iter().map(|l| l.iter().collect::<String>()).collect()
}

pub fn part2(input: Matrix<char>) -> usize {
    let mut m = input;
    let mut seen = Vec::new();
    seen.push(to_string(&m));
    let tot = 1000000000;
    for i in 0..tot {
        m = cycle(m);
        let s = to_string(&m);
        if let Some((p, _)) = seen.iter().find_position(|&st| st == &s) {
            let remaining = tot - i;
            let len_cycle = i - p + 1;
            let num_to_do = remaining % len_cycle;
            for _ in 1..num_to_do {
                m = cycle(m);
            }
            return score(m);
        }
        seen.push(s);
    }
    score(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_move_left() {
        let input = "O.O#..OO.O#";
        assert_eq!(move_left(input), "OO.#OOO...#".to_owned());
    }
    #[test]
    fn test_move_right() {
        let input = "O.O#..OO.O#";
        assert_eq!(move_right(input), ".OO#...OOO#".to_owned());
    }
    #[test]
    fn test_north() {
        let input = indoc! {
            "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        let expected = indoc! {
            "
            OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#...."
        };
        let parsed = parse_input(input);
        let expected_parsed = parse_input(expected);
        assert_eq!(north(parsed), expected_parsed);
    }
    #[test]
    fn test_south() {
        let input = indoc! {
            "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        let expected = indoc! {
            "
            .....#....
            ....#....#
            ...O.##...
            ...#......
            O.O....O#O
            O.#..O.#.#
            O....#....
            OO....OO..
            #OO..###..
            #OO.O#...O"
        };
        let parsed = parse_input(input);
        let expected_parsed = parse_input(expected);
        assert_eq!(south(parsed), expected_parsed);
    }

    #[test]
    fn test_east() {
        let input = indoc! {
            "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        let expected = indoc! {
            "
            ....O#....
            .OOO#....#
            .....##...
            .OO#....OO
            ......OO#.
            .O#...O#.#
            ....O#..OO
            .........O
            #....###..
            #..OO#...."
        };
        let parsed = parse_input(input);
        let expected_parsed = parse_input(expected);
        assert_eq!(east(parsed), expected_parsed);
    }
    #[test]
    fn test_west() {
        let input = indoc! {
            "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        let expected = indoc! {
            "
            O....#....
            OOO.#....#
            .....##...
            OO.#OO....
            OO......#.
            O.#O...#.#
            O....#OO..
            O.........
            #....###..
            #OO..#...."
        };
        let parsed = parse_input(input);
        let expected_parsed = parse_input(expected);
        assert_eq!(west(parsed), expected_parsed);
    }

    #[test]
    fn test_score() {
        let expected = indoc! {
            "OOOO.#.O..
            OO..#....#
            OO..O##..O
            O..#.OO...
            ........#.
            ..#....#.#
            ..O..#.O.O
            ..O.......
            #....###..
            #....#...."
        };
        let expected_parsed = parse_input(expected);
        assert_eq!(score(expected_parsed), 136);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        assert_eq!(part1(parse_input(input)), 136);
    }

    #[test]
    fn test_cycle() {
        let input = indoc! {
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        let expected = indoc! {
            ".....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#...."
        };
        let parsed = parse_input(input);
        let expected_parsed = parse_input(expected);
        assert_eq!(cycle(parsed), expected_parsed);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
            "O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
        };
        assert_eq!(part2(parse_input(input)), 64);
    }
}
