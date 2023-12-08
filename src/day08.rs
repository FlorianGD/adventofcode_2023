use num::integer::lcm;
use rustc_hash::FxHashMap as HashMap;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, delimited, preceded, repeat, terminated};
use winnow::token::take;
use winnow::{PResult, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Right,
}

impl std::str::FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

fn l(input: &mut &str) -> PResult<Dir> {
    "L".parse_to().parse_next(input)
}
fn r(input: &mut &str) -> PResult<Dir> {
    "R".parse_to().parse_next(input)
}

fn directions(input: &mut &str) -> PResult<Vec<Dir>> {
    repeat(1.., alt((l, r))).parse_next(input)
}

fn step(input: &mut &str) -> PResult<(String, (String, String))> {
    let start = take(3usize).parse_next(input)?;
    let left = preceded(" = (", take(3usize)).parse_next(input)?;
    let right = delimited(", ", take(3usize), (")", line_ending)).parse_next(input)?;
    Ok((start.to_string(), (left.to_string(), right.to_string())))
}

pub fn parse_input(input: &str) -> (Vec<Dir>, HashMap<String, (String, String)>) {
    let mut input = input;
    let dirs = terminated(directions, (line_ending, line_ending))
        .parse_next(&mut input)
        .unwrap();
    let steps: Vec<(String, (String, String))> = repeat(1.., step).parse_next(&mut input).unwrap();
    (dirs, HashMap::from_iter(steps))
}

pub fn part1((directions, steps): (Vec<Dir>, HashMap<String, (String, String)>)) -> usize {
    solve(&directions, &steps, "AAA")
}

fn solve(directions: &[Dir], steps: &HashMap<String, (String, String)>, start: &str) -> usize {
    let mut cycle = directions.iter().cycle();
    let mut pos = start.to_string();
    let mut num_steps = 0;
    while !pos.ends_with('Z') {
        let possible = &steps[&pos];
        pos = match cycle.next() {
            Some(Dir::Left) => possible.0.to_string(),
            Some(Dir::Right) => possible.1.to_string(),
            _ => panic!("weird direction"),
        };
        num_steps += 1
    }
    num_steps
}

fn all_exits(directions: &[Dir], steps: &HashMap<String, (String, String)>) -> Vec<usize> {
    steps
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|node| solve(directions, steps, node))
        .collect()
}

pub fn part2((directions, steps): (Vec<Dir>, HashMap<String, (String, String)>)) -> usize {
    let exits = all_exits(&directions, &steps);
    exits.into_iter().reduce(lcm).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_all_exits() {
        let input = indoc! {
        "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
        "
        };
        let (directions, steps) = parse_input(input);
        let mut all = all_exits(&directions, &steps);
        all.sort();
        assert_eq!(all, vec![2, 3]);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            "LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            "
        };
        let result = part2(parse_input(input));
        assert_eq!(result, 6)
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "
        };
        let result = part1(parse_input(input));
        assert_eq!(result, 2)
    }

    #[test]
    fn test_directions() {
        let mut input = "LRL";
        let dirs = directions.parse_next(&mut input).unwrap();
        assert_eq!(dirs, vec![Dir::Left, Dir::Right, Dir::Left]);
    }

    #[test]
    fn test_step() {
        let mut input = "AAA = (BBB, CCC)\n";
        let (start, (left, right)): (String, (String, String)) =
            step.parse_next(&mut input).unwrap();
        assert_eq!(&start, "AAA");
        assert_eq!(&left, "BBB");
        assert_eq!(&right, "CCC");
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "
        };
        let (dirs, steps) = parse_input(input);
        assert_eq!(dirs, vec![Dir::Right, Dir::Left]);
        assert_eq!(
            steps,
            HashMap::from_iter(vec![
                ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string())),
                ("BBB".to_string(), ("DDD".to_string(), "EEE".to_string())),
                ("CCC".to_string(), ("ZZZ".to_string(), "GGG".to_string())),
                ("DDD".to_string(), ("DDD".to_string(), "DDD".to_string())),
                ("EEE".to_string(), ("EEE".to_string(), "EEE".to_string())),
                ("GGG".to_string(), ("GGG".to_string(), "GGG".to_string())),
                ("ZZZ".to_string(), ("ZZZ".to_string(), "ZZZ".to_string())),
            ])
        )
    }
}
