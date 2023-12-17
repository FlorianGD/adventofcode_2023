use num::Complex;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashMap as HashMap;

type Coord = Complex<isize>;
type Grid = HashMap<Coord, u32>;

const RIGHT: Coord = Complex::new(1, 0);
// imaginary axis is flipped
const DOWN: Coord = Complex::new(0, 1);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    pos: Coord,
    prev_dir: Option<(Complex<isize>, u8)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos2 {
    pos: Coord,
    dir: Option<Coord>,
}
impl Pos2 {
    fn new(pos: Coord, dir: Option<Complex<isize>>) -> Self {
        Pos2 { pos, dir }
    }

    fn next_in_dir(&self, grid: &Grid, dir: Coord, bottom_right: Coord) -> Vec<(Self, u32)> {
        let mut results = vec![];
        // First, we try to go by 4 in the direction dir
        let mut i = 0;
        let mut cost = 0;
        let mut new_pos = self.pos + dir;
        while let Some(val) = grid.get(&new_pos) {
            if i < 3 {
                i += 1;
                cost += val;
                new_pos += dir;
            } else {
                break;
            }
        }
        if new_pos.re < 0
            || new_pos.re > bottom_right.re
            || new_pos.im < 0
            || new_pos.im > bottom_right.im
        {
            return vec![];
        }
        cost += grid.get(&new_pos).unwrap_or(&0);
        // we just push the position where we are
        results.push((Pos2::new(new_pos, Some(dir)), cost));
        // Then, we try to add the next 6 elements in the same direction
        while let Some(val) = grid.get(&(new_pos + dir)) {
            if i < 9 {
                i += 1;
                cost += val;
                new_pos += dir;
                results.push((Pos2::new(new_pos, Some(dir)), cost));
            } else {
                break;
            }
        }
        results
    }

    fn successors_p2(&self, grid: &Grid, bottom_right: Coord) -> Vec<(Self, u32)> {
        let mut results = vec![];

        match self.dir {
            None => {
                results.extend(self.next_in_dir(grid, RIGHT, bottom_right));
                results.extend(self.next_in_dir(grid, DOWN, bottom_right));
            }
            Some(dir) => {
                let rotated_left = dir * Complex::new(0, 1);
                results.extend(self.next_in_dir(grid, rotated_left, bottom_right));
                let rotated_right = dir * Complex::new(0, -1);
                results.extend(self.next_in_dir(grid, rotated_right, bottom_right));
            }
        }
        results
    }
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

pub fn part2((grid, bottom_right): (Grid, Coord)) -> u32 {
    let initial_pos = Pos2::new(Complex::new(0, 0), None);
    if let Some((_path, cost)) = dijkstra(
        &initial_pos,
        |p| p.successors_p2(&grid, bottom_right),
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

    const LEFT: Coord = Complex::new(-1, 0);
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
        assert_eq!(part1(parse_input(input)), 102);
    }

    #[test]
    fn test_successors_p2() {
        let input = get_input();
        let (grid, bottom_right) = parse_input(input);
        let pos = Pos2::new(Complex::new(0, 0), None);
        let successors = pos.successors_p2(&grid, bottom_right);
        assert_eq!(successors.len(), 14);
        // let successors = Pos2::new(bottom_right, Some(RIGHT)).successors_p2(&grid, bottom_right);
        // assert_eq!(successors, vec![]);
    }

    #[test]
    fn test_part2() {
        let input = get_input();
        let result = part2(parse_input(input));
        assert_eq!(result, 94);
    }
    #[test]
    fn test_part2_edge_case() {
        let input = indoc! {
            "
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991"
        };
        let result = part2(parse_input(input));
        assert_eq!(result, 71);
    }
}
