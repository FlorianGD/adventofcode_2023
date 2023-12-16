use num::complex::Complex;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
type Coord = Complex<isize>;

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    SplitVertical,    // |
    SplitHorizontal,  // -
    ReflectUpToRight, // /
    ReflectUpToLeft,  // \
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn val(&self) -> Complex<isize> {
        match self {
            Direction::Left => Complex::new(-1, 0),
            Direction::Right => Complex::new(1, 0),
            // imaginary axis is flipped
            Direction::Up => Complex::new(0, -1),
            Direction::Down => Complex::new(0, 1),
        }
    }
}

pub fn parse_input(input: &str) -> (HashMap<Coord, Move>, Coord) {
    let mut m = HashMap::default();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = Complex::new(j as isize, i as isize);

            match c {
                '.' => (),
                '-' => {
                    m.insert(pos, Move::SplitHorizontal);
                }
                '|' => {
                    m.insert(pos, Move::SplitVertical);
                }
                '/' => {
                    m.insert(pos, Move::ReflectUpToRight);
                }
                '\\' => {
                    m.insert(pos, Move::ReflectUpToLeft);
                }
                _ => panic!("Invalid character: {}", c),
            };
        }
    }
    let len = input.lines().count() as isize - 1;
    (m, Complex::new(len, len))
}

fn next(dir: &Direction, m: &Move) -> Vec<Direction> {
    match (&dir, &m) {
        (Direction::Left, Move::SplitVertical) => vec![Direction::Up, Direction::Down],
        (Direction::Left, Move::SplitHorizontal) => vec![Direction::Left],
        (Direction::Left, Move::ReflectUpToRight) => vec![Direction::Down],
        (Direction::Left, Move::ReflectUpToLeft) => vec![Direction::Up],
        (Direction::Right, Move::SplitVertical) => vec![Direction::Up, Direction::Down],
        (Direction::Right, Move::SplitHorizontal) => vec![Direction::Right],
        (Direction::Right, Move::ReflectUpToRight) => vec![Direction::Up],
        (Direction::Right, Move::ReflectUpToLeft) => vec![Direction::Down],
        (Direction::Up, Move::SplitVertical) => vec![Direction::Up],
        (Direction::Up, Move::SplitHorizontal) => vec![Direction::Right, Direction::Left],
        (Direction::Up, Move::ReflectUpToRight) => vec![Direction::Right],
        (Direction::Up, Move::ReflectUpToLeft) => vec![Direction::Left],
        (Direction::Down, Move::SplitVertical) => vec![Direction::Down],
        (Direction::Down, Move::SplitHorizontal) => vec![Direction::Right, Direction::Left],
        (Direction::Down, Move::ReflectUpToRight) => vec![Direction::Left],
        (Direction::Down, Move::ReflectUpToLeft) => vec![Direction::Right],
    }
}

pub fn part1((grid, bottom_right): (HashMap<Coord, Move>, Coord)) -> usize {
    let pos = Complex::new(0, 0);
    let dir = Direction::Right;
    let mut seen = HashSet::default();
    let mut visited = HashSet::default();
    let mut positions = vec![(pos, dir)];
    while let Some((pos, dir)) = positions.pop() {
        if pos.re < 0
            || pos.re > bottom_right.re
            || pos.im < 0
            || pos.im > bottom_right.im
            || visited.contains(&(pos, dir))
        {
            continue;
        }
        seen.insert(pos);
        visited.insert((pos, dir));
        match grid.get(&pos) {
            None => {
                // . or outside
                positions.push((pos + dir.val(), dir));
            }
            Some(m) => {
                for new_dir in next(&dir, m) {
                    positions.push((pos + new_dir.val(), new_dir));
                }
            }
        }
    }

    seen.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
        "};
        let (grid, bottom_right) = parse_input(input);
        let expected = HashMap::from_iter([
            (Complex::new(1, 0), Move::SplitVertical),
            (Complex::new(5, 0), Move::ReflectUpToLeft),
            (Complex::new(0, 1), Move::SplitVertical),
            (Complex::new(2, 1), Move::SplitHorizontal),
            (Complex::new(4, 1), Move::ReflectUpToLeft),
            (Complex::new(5, 2), Move::SplitVertical),
            (Complex::new(6, 2), Move::SplitHorizontal),
            (Complex::new(8, 3), Move::SplitVertical),
            (Complex::new(9, 5), Move::ReflectUpToLeft),
            (Complex::new(4, 6), Move::ReflectUpToRight),
            (Complex::new(6, 6), Move::ReflectUpToLeft),
            (Complex::new(7, 6), Move::ReflectUpToLeft),
            (Complex::new(1, 7), Move::SplitHorizontal),
            (Complex::new(3, 7), Move::SplitHorizontal),
            (Complex::new(4, 7), Move::ReflectUpToRight),
            (Complex::new(7, 7), Move::SplitVertical),
            (Complex::new(1, 8), Move::SplitVertical),
            (Complex::new(6, 8), Move::SplitHorizontal),
            (Complex::new(7, 8), Move::SplitVertical),
            (Complex::new(9, 8), Move::ReflectUpToLeft),
            (Complex::new(2, 9), Move::ReflectUpToRight),
            (Complex::new(3, 9), Move::ReflectUpToRight),
            (Complex::new(5, 9), Move::SplitVertical),
        ]);
        assert_eq!(grid, expected);
        assert_eq!(bottom_right, Complex::new(9, 9));
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
        r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
        "};
        let expected = 46;
        assert_eq!(part1(parse_input(input)), expected);
    }
}
