use num::complex::Complex;
use std::collections::HashMap;

type Coord = Complex<isize>;
const LEFT: Coord = Complex::new(-1, 0);
const RIGHT: Coord = Complex::new(1, 0);
// imaginary axis is flipped
const UP: Coord = Complex::new(0, -1);
const DOWN: Coord = Complex::new(0, 1);
type State = HashMap<Coord, Coord>;

pub fn parse_input(input: &str) -> (Coord, HashMap<Coord, State>) {
    let mut m = HashMap::default();
    let mut start = Complex::new(-1, -1);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = Complex::new(j as isize, i as isize);
            match c {
                '.' => (),
                'S' => start = pos,
                '-' => {
                    m.insert(pos, HashMap::from_iter([(RIGHT, RIGHT), (LEFT, LEFT)]));
                }
                '|' => {
                    m.insert(pos, HashMap::from_iter([(UP, UP), (DOWN, DOWN)]));
                }
                'J' => {
                    m.insert(pos, HashMap::from_iter([(RIGHT, UP), (DOWN, LEFT)]));
                }
                '7' => {
                    m.insert(pos, HashMap::from_iter([(RIGHT, DOWN), (UP, LEFT)]));
                }
                'L' => {
                    m.insert(pos, HashMap::from_iter([(LEFT, UP), (DOWN, RIGHT)]));
                }
                'F' => {
                    m.insert(pos, HashMap::from_iter([(UP, RIGHT), (LEFT, DOWN)]));
                }
                _ => panic!("Invalid character: {}", c),
            };
        }
    }
    (start, m)
}

fn next(pos: Coord, dir: Coord, transitions: &HashMap<Coord, State>) -> Option<(Coord, Coord)> {
    match transitions.get(&(pos + dir)) {
        Some(states) => match states.get(&dir) {
            Some(new_dir) => Some((pos + dir, *new_dir)),
            None => None,
        },
        None => None,
    }
}

pub fn part1((start, transitions): (Coord, HashMap<Coord, State>)) -> usize {
    let mut len = 1;
    let (mut pos, mut dir) = [LEFT, RIGHT, UP, DOWN]
        .iter()
        .filter_map(|d| next(start, *d, &transitions))
        .next()
        .unwrap();
    // start is not in the graph, so we made a round when we get None
    // I do not know why I cannot assign pos and dir directly
    while let Some(x) = next(pos, dir, &transitions) {
        (pos, dir) = x;
        len += 1;
    }
    len / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1_simple() {
        let input = indoc! {
        "-L|F7
         7S-7|
         L|7||
         -L-J|
         L|-JF"};
        let (start, m) = parse_input(input);
        assert_eq!(part1((start, m)), 4);
    }

    #[test]
    fn test_part1_complex() {
        let input = indoc! {
                "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"};
        let (start, m) = parse_input(input);
        assert_eq!(part1((start, m)), 8);
    }
    #[test]
    fn test_next() {
        let input = indoc! {
        ".....
        .S-7.
        .|.|.
        .L-J.
        ....."};
        let (start, m) = parse_input(input);
        assert_eq!(next(start, UP, &m), None);
        assert_eq!(next(start, DOWN, &m), Some((Complex::new(1, 2), DOWN)));
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        ".....
        .S-7.
        .|.|.
        .L-J.
        ....."};
        let (start, m) = parse_input(input);
        assert_eq!(start, Complex::new(1, 1));
        assert_eq!(
            m[&Complex::new(2, 1)],
            HashMap::from_iter([(RIGHT, RIGHT), (LEFT, LEFT)])
        );
    }
}
