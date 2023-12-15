use crate::parsers::num;

use winnow::{
    ascii::alpha1,
    combinator::{alt, separated_pair, terminated},
    PResult, Parser,
};

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .strip_suffix('\n')
        .unwrap_or(input)
        .split(',')
        .map(|x| x.as_bytes().to_vec())
        .collect()
}

fn hash(chars: &[u8]) -> usize {
    chars
        .iter()
        .fold(0usize, |acc, c| ((acc + *c as usize) * 17) % 256)
}

pub fn part1(input: Vec<Vec<u8>>) -> usize {
    input.iter().map(|p| hash(p)).sum()
}

#[derive(Clone)]
pub struct Lens {
    label: String,
    focal: u8,
}

impl Lens {
    fn new(label: &str, focal: u8) -> Self {
        Lens {
            label: label.to_string(),
            focal,
        }
    }

    fn change_focal(&mut self, focal: u8) {
        self.focal = focal;
    }
}

enum Operation {
    Dash,
    Equal(u8),
}

pub struct Instruction {
    label: String,
    operation: Operation,
}

impl std::str::FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s;
        alt((parse_dash, parse_equal))
            .parse_next(&mut s)
            .map_err(|e| e.to_string())
    }
}

fn parse_dash(input: &mut &str) -> PResult<Instruction> {
    let label = terminated(alpha1, '-').parse_next(input)?;
    Ok(Instruction {
        label: label.to_string(),
        operation: Operation::Dash,
    })
}

fn parse_equal(input: &mut &str) -> PResult<Instruction> {
    let (label, focal) = separated_pair(alpha1, '=', num).parse_next(input)?;
    Ok(Instruction {
        label: label.to_string(),
        operation: Operation::Equal(focal),
    })
}

pub fn parse_input_p2(input: &str) -> Vec<Instruction> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

fn focusing_power(boxes: Vec<Vec<Lens>>) -> usize {
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            (i + 1)
                * v.into_iter()
                    .enumerate()
                    .map(|(j, l)| (j + 1) * l.focal as usize)
                    .sum::<usize>()
        })
        .sum()
}

pub fn part2(instructions: Vec<Instruction>) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for instr in instructions {
        let box_num = hash(instr.label.as_bytes());
        match instr.operation {
            Operation::Dash => {
                boxes[box_num].retain(|l| l.label != instr.label);
            }
            Operation::Equal(n) => {
                if let Some(l) = boxes[box_num].iter_mut().find(|x| x.label == instr.label) {
                    l.change_focal(n)
                } else {
                    boxes[box_num].push(Lens::new(&instr.label, n))
                }
            }
        }
    }
    focusing_power(boxes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hash() {
        let chars = "HASH".as_bytes();
        assert_eq!(hash(chars), 52);
    }

    #[test]
    fn test_parse_input() {
        let input = "abc,def";
        let expected = vec![vec![b'a', b'b', b'c'], vec![b'd', b'e', b'f']];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(parse_input(input)), 1320);
    }

    #[test]
    fn test_focusing_power() {
        let boxes = vec![
            vec![Lens::new("rn", 1), Lens::new("cm", 2)],
            vec![],
            vec![],
            vec![Lens::new("ot", 7), Lens::new("ab", 5), Lens::new("pc", 6)],
        ];
        assert_eq!(focusing_power(boxes), 145);
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

        assert_eq!(part2(parse_input_p2(input)), 145);
    }
}
