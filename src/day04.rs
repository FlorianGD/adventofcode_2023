use std::collections::HashSet;

use winnow::{
    ascii::{digit1, space0, space1},
    combinator::{delimited, separated, separated_pair},
    PResult, Parser,
};

type Cards = (u16, (Vec<u16>, Vec<u16>));

fn card(input: &mut &str) -> PResult<u16> {
    delimited(("Card", space1), digit1, ": ")
        .parse_to()
        .parse_next(input)
}

fn num(input: &mut &str) -> PResult<u16> {
    digit1.parse_to().parse_next(input)
}

fn numbers(input: &mut &str) -> PResult<Vec<u16>> {
    delimited(space0, separated(1.., num, space1), space0).parse_next(input)
}

fn sets(input: &mut &str) -> PResult<(Vec<u16>, Vec<u16>)> {
    separated_pair(numbers, (space0, "|", space0), numbers).parse_next(input)
}

fn line(input: &mut &str) -> PResult<Cards> {
    (card, sets).parse_next(input)
}

pub fn parse_input(input: &str) -> Vec<Cards> {
    input
        .lines()
        .map(|mut l| line.parse_next(&mut l).unwrap())
        .collect()
}

fn score(len: usize) -> usize {
    match len {
        0 => 0,
        n => 2_usize.pow(n as u32 - 1),
    }
}

fn num_winning(winning_numbers: &[u16], got_numbers: &[u16]) -> usize {
    let g: HashSet<_> = HashSet::from_iter(got_numbers);
    let w: HashSet<_> = HashSet::from_iter(winning_numbers);
    Iterator::count(g.intersection(&w))
}

pub fn part1(input: Vec<Cards>) -> usize {
    input
        .into_iter()
        .map(|(_, (winning_numbers, got_numbers))| {
            score(num_winning(&winning_numbers, &got_numbers))
        })
        .sum()
}

pub fn part2(input: Vec<Cards>) -> usize {
    let mut buffer = vec![1; input.len() + 1];
    for (id, (winning_numbers, got_numbers)) in input.iter() {
        let current_num = buffer[*id as usize];
        let num_winning = num_winning(winning_numbers, got_numbers);
        match num_winning {
            0 => (),
            n => {
                let id = *id as usize;
                for i in id + 1..=id + n {
                    buffer[i] += current_num;
                }
            }
        }
    }
    buffer.iter().skip(1).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_card() {
        let mut input = "Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let res = card(&mut input);
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn test_numbers() {
        let mut input = " 41 48 83 86 17 ";
        let res = numbers(&mut input);
        assert_eq!(res, Ok(vec![41, 48, 83, 86, 17]))
    }
    #[test]
    fn test_sets() {
        let mut input = " 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let res = sets(&mut input);
        assert_eq!(
            res,
            Ok((vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]))
        )
    }
    #[test]
    fn test_line() {
        let mut input = "Card    1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let res = line(&mut input);
        assert_eq!(
            res,
            Ok((
                1,
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53])
            ))
        )
    }
    #[test]
    fn test_parse() {
        let input = indoc! {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        "};
        let res = parse_input(input);
        let expected = vec![
            (
                1,
                (vec![41, 48, 83, 86, 17], vec![83, 86, 6, 31, 17, 9, 48, 53]),
            ),
            (
                2,
                (
                    vec![13, 32, 20, 16, 61],
                    vec![61, 30, 68, 82, 17, 32, 24, 19],
                ),
            ),
        ];
        assert_eq!(res, expected)
    }

    #[test]
    fn test_score() {
        assert_eq!(score(0), 0);
        assert_eq!(score(1), 1);
        assert_eq!(score(4), 8);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let parsed = parse_input(input);
        assert_eq!(part1(parsed), 13);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "};
        let parsed = parse_input(input);
        assert_eq!(part2(parsed), 30);
    }
}
