use winnow::{
    ascii::{line_ending, space1},
    combinator::{delimited, preceded, separated},
    Parser,
};

use crate::parsers::num;

fn compute_range(t: usize, d: usize) -> f64 {
    let delta = t * t - 4 * d;
    let h1 = (t as f64 - (delta as f64).sqrt()) / 2.0;
    let h2 = (t as f64 + (delta as f64).sqrt()) / 2.0;

    (h2 - 1.0).ceil() - (h1 + 1.0).floor() + 1.0
}

pub fn part1(input: Vec<(usize, usize)>) -> f64 {
    input
        .into_iter()
        .map(|(t, d)| compute_range(t, d))
        .product()
}

pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut input = input;
    let time: Vec<usize> = delimited(
        ("Time:", space1),
        separated(1.., num::<usize>, space1),
        line_ending,
    )
    .parse_next(&mut input)
    .unwrap();
    let distances: Vec<usize> = delimited(
        ("Distance: ", space1),
        separated(1.., num::<usize>, space1),
        line_ending,
    )
    .parse_next(&mut input)
    .unwrap();
    // vec![(7, 9), (15, 40), (30, 200)]
    time.into_iter().zip(distances).collect()
}

pub fn parse_input_p2(input: &str) -> Vec<(usize, usize)> {
    let input: String = input
        .lines()
        .map(|l| l.to_string().replace(' ', ""))
        .collect::<String>();
    let mut input = input.as_str();
    let time = preceded("Time:", num::<usize>)
        .parse_next(&mut input)
        .unwrap();
    let distance = preceded("Distance:", num::<usize>)
        .parse_next(&mut input)
        .unwrap();
    vec![(time, distance)]
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_parse() {
        let input = indoc! {
            "Time:      7  15   30
            Distance:  9  40  200
            "
        };
        let res = parse_input(input);
        assert_eq!(res, vec![(7, 9), (15, 40), (30, 200)]);
    }
    #[test]
    fn test_parse_p2() {
        let input = indoc! {
            "Time:      7  15   30
            Distance:  9  40  200
            "
        };
        let res = parse_input_p2(input);
        assert_eq!(res, vec![(71530, 940200)]);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
            "Time:      7  15   30
            Distance:  9  40  200
            "
        };
        let res = part1(parse_input(input));
        assert_eq!(res, 288.0);
    }
}
