use num::Complex;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashMap as HashMap;

type Coord = Complex<isize>;
type Grid = HashMap<Coord, u32>;

const LEFT: Coord = Complex::new(-1, 0);
const RIGHT: Coord = Complex::new(1, 0);
// imaginary axis is flipped
const UP: Coord = Complex::new(0, -1);
const DOWN: Coord = Complex::new(0, 1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    pos: Coord,
    prev_dir: Option<(Complex<isize>, u8)>,
}

impl Pos {
    fn new(pos: Coord, prev_dir: Option<(Complex<isize>, u8)>) -> Self {
        Pos { pos, prev_dir }
    }
    fn successors(&self, grid: &Grid) -> Vec<(Self, u32)> {
        let mut results = vec![];
        // rotate left is still in bounds
        match self.prev_dir {
            None => {
                // first position only
                return vec![
                    (
                        Pos::new(self.pos + RIGHT, Some((RIGHT, 1))),
                        *grid.get(&(self.pos + RIGHT)).unwrap(),
                    ),
                    (
                        Pos::new(self.pos + DOWN, Some((DOWN, 1))),
                        *grid.get(&(self.pos + DOWN)).unwrap(),
                    ),
                ];
            }
            Some((dir, count)) => {
                let rotated_left = self.pos + dir * Complex::new(0, 1);
                let rotated_right = self.pos + dir * Complex::new(0, -1);
                let in_front = self.pos + dir;
                if let Some(val) = grid.get(&rotated_left) {
                    results.push((
                        Pos::new(rotated_left, Some((dir * Complex::new(0, 1), 1))),
                        *val,
                    ));
                }
                if let Some(val) = grid.get(&rotated_right) {
                    results.push((
                        Pos::new(rotated_right, Some((dir * Complex::new(0, -1), 1))),
                        *val,
                    ));
                }
                if let Some(val) = grid.get(&in_front) {
                    if count < 3 {
                        results.push((Pos::new(in_front, Some((dir, count + 1))), *val));
                    }
                }
            }
        }
        results
    }
    fn successors_p2(&self, grid: &Grid) -> Vec<(Self, u32)> {
        let mut results = vec![];
        // rotate left is still in bounds
        match self.prev_dir {
            None => {
                // first position only
                return vec![
                    (
                        Pos::new(self.pos + RIGHT, Some((RIGHT, 4))),
                        (1..=4)
                            .map(|i| *grid.get(&(self.pos + i * RIGHT)).unwrap())
                            .sum(),
                    ),
                    (
                        Pos::new(self.pos + DOWN, Some((DOWN, 4))),
                        (1..=4)
                            .map(|i| *grid.get(&(self.pos + i * DOWN)).unwrap())
                            .sum(),
                    ),
                ];
            }
            Some((dir, count)) => {
                let rotated_left = self.pos + dir * Complex::new(0, 1);
                let rotated_right = self.pos + dir * Complex::new(0, -1);
                let in_front = self.pos + dir;
                if let Some(val) = grid.get(&rotated_left) {
                    results.push((
                        Pos::new(rotated_left, Some((dir * Complex::new(0, 1), 1))),
                        *val,
                    ));
                }
                if let Some(val) = grid.get(&rotated_right) {
                    results.push((
                        Pos::new(rotated_right, Some((dir * Complex::new(0, -1), 1))),
                        *val,
                    ));
                }
                if let Some(val) = grid.get(&in_front) {
                    if count < 3 {
                        results.push((Pos::new(in_front, Some((dir, count + 1))), *val));
                    }
                }
            }
        }
        results
    }
}

pub fn parse_input(input: &str) -> (Grid, Coord) {
    let grid = input
        .lines()
        .enumerate()
        .flat_map(|(j, l)| {
            l.chars().enumerate().map(move |(i, c)| {
                (
                    Complex::new(i as isize, j as isize),
                    c.to_digit(10).unwrap(),
                )
            })
        })
        .collect();
    let j_max = input.lines().count() as isize - 1;
    let i_max = input.lines().next().unwrap().len() as isize - 1;
    (grid, Complex::new(i_max, j_max))
}

pub fn part1((grid, bottom_right): (Grid, Coord)) -> u32 {
    let initial_pos = Pos::new(Complex::new(0, 0), None);
    if let Some((_path, cost)) = dijkstra(
        &initial_pos,
        |p| p.successors(&grid),
        |p| p.pos == bottom_right,
    ) {
        cost
    } else {
        panic!("no result found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn get_input() -> &'static str {
        indoc! {
            "2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533"
        }
    }

    #[test]
    fn test_parse_input() {
        let input = get_input();
        let (grid, bottom_right) = parse_input(input);
        assert_eq!(bottom_right, Complex::new(12, 12));
        assert_eq!(grid.get(&Complex::new(0, 0)), Some(&2));
        assert_eq!(grid.get(&bottom_right), Some(&3));
    }

    #[test]
    fn test_successors() {
        let input = get_input();
        let (grid, _bottom_right) = parse_input(input);
        let pos = Pos::new(Complex::new(0, 0), None);
        let successors = pos.successors(&grid);
        assert_eq!(
            successors,
            vec![
                (Pos::new(Complex::new(1, 0), Some((RIGHT, 1))), 4),
                (Pos::new(Complex::new(0, 1), Some((DOWN, 1))), 3)
            ]
        );
        let pos = Pos::new(Complex::new(1, 1), Some((DOWN, 2)));
        let successors = pos.successors(&grid);
        assert_eq!(
            successors,
            vec![
                (Pos::new(Complex::new(0, 1), Some((LEFT, 1))), 3),
                (Pos::new(Complex::new(2, 1), Some((RIGHT, 1))), 1),
                (Pos::new(Complex::new(1, 2), Some((DOWN, 3))), 2),
            ]
        );
        let pos = Pos::new(Complex::new(1, 1), Some((DOWN, 3)));
        let successors = pos.successors(&grid);
        assert_eq!(
            successors,
            vec![
                (Pos::new(Complex::new(0, 1), Some((LEFT, 1))), 3),
                (Pos::new(Complex::new(2, 1), Some((RIGHT, 1))), 1),
            ]
        );
    }

    #[test]
    fn test_part1() {
        let input = get_input();
        assert_eq!(part1(parse_input(input)), 102)
    }
}
