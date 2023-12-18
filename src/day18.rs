use crate::helpers::Direction;
use num::Complex;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Instr {
    dir: Direction,
    val: isize,
    color: String,
}

impl Instr {
    fn new(dir: Direction, val: isize, color: String) -> Self {
        Instr { dir, val, color }
    }
}

impl std::str::FromStr for Instr {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let dir = match split.next().unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };
        let mut val = 0;
        if let Some(value) = split.next() {
            val = value.parse().map_err(|_| String::from("No value"))?;
        }
        let mut color = String::default();
        if let Some(col) = split.next() {
            color = String::from(
                col.strip_prefix("(#")
                    .expect("no (#")
                    .strip_suffix(')')
                    .expect("no )"),
            )
        }
        Ok(Instr::new(dir, val, color))
    }
}

pub fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1(input: Vec<Instr>) -> isize {
    // shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
    // A &= \frac 1 2 \sum_{i=1}^n (y_i + y_{i+1})(x_i - x_{i+1})\\
    let mut prev = Complex::new(0, 0);
    let mut area: isize = 0;
    let mut length = 0;
    for instr in input {
        let pos = prev + instr.dir.val() * instr.val;
        area += (pos.im + prev.im) * (prev.re - pos.re);
        length += instr.val;
        prev = pos;
    }
    (area.abs() + length) / 2 + 1
}

pub fn part2(input: Vec<Instr>) -> isize {
    // shoelace formula https://en.wikipedia.org/wiki/Shoelace_formula
    // A &= \frac 1 2 \sum_{i=1}^n (y_i + y_{i+1})(x_i - x_{i+1})\\
    let mut prev = Complex::new(0, 0);
    let mut area: isize = 0;
    let mut length = 0;
    for instr in input {
        //0 means R, 1 means D, 2 means L, and 3 means U
        let dir = match instr.color.chars().last() {
            None => panic!("no color"),
            Some('0') => Direction::Right,
            Some('1') => Direction::Down,
            Some('2') => Direction::Left,
            Some('3') => Direction::Up,
            Some(c) => panic!("Not a correct last char {c}"),
        };
        let val = isize::from_str_radix(&instr.color[0..(instr.color.len() - 1)], 16).unwrap();
        let pos = prev + dir.val() * val;
        //A &= \frac 1 2 \sum_{i=1}^n (x_iy_{i+1}-x_{i+1}y_i)
        area += (prev.re * pos.im) - (pos.re * prev.im);
        length += val;
        prev = pos;
    }
    (area.abs() + length) / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn data() -> &'static str {
        indoc! {
        "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
        }
    }

    #[test]
    fn test_parse_instr() {
        let expected = Instr::new(Direction::Right, 6, String::from("70c710"));
        assert_eq!("R 6 (#70c710)".parse(), Ok(expected));
    }

    #[test]
    fn test_parse_input() {
        use super::Direction::*;
        let input = data();
        let expected = vec![
            Instr::new(Right, 6, String::from("70c710")),
            Instr::new(Down, 5, String::from("0dc571")),
            Instr::new(Left, 2, String::from("5713f0")),
            Instr::new(Down, 2, String::from("d2c081")),
            Instr::new(Right, 2, String::from("59c680")),
            Instr::new(Down, 2, String::from("411b91")),
            Instr::new(Left, 5, String::from("8ceee2")),
            Instr::new(Up, 2, String::from("caa173")),
            Instr::new(Left, 1, String::from("1b58a2")),
            Instr::new(Up, 2, String::from("caa171")),
            Instr::new(Right, 2, String::from("7807d2")),
            Instr::new(Up, 3, String::from("a77fa3")),
            Instr::new(Left, 2, String::from("015232")),
            Instr::new(Up, 2, String::from("7a21e3")),
        ];
        assert_eq!(parse_input(input), expected);
    }
    #[test]
    fn test_part1() {
        let input = data();
        assert_eq!(part1(parse_input(input)), 62);
    }
    #[test]
    fn test_part2() {
        let input = data();
        assert_eq!(part2(parse_input(input)), 952408144115);
    }
}
