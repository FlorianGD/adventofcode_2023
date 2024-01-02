use itertools::Itertools;
use memoize::memoize;
use num::complex::Complex;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::collections::VecDeque;

type Coord = Complex<isize>;
const LEFT: Coord = Complex::new(-1, 0);
const RIGHT: Coord = Complex::new(1, 0);
// imaginary axis is flipped
const UP: Coord = Complex::new(0, -1);
const DOWN: Coord = Complex::new(0, 1);

pub fn parse_input(input: &str) -> (Coord, HashSet<Coord>) {
    let mut m = HashSet::default();
    let mut start = Complex::new(-1, -1);
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let pos = Complex::new(j as isize, i as isize);
            match c {
                '.' => {
                    m.insert(pos);
                }
                'S' => {
                    start = pos;
                    m.insert(pos);
                }
                _ => (),
            };
        }
    }
    (start, m)
}

#[memoize(Ignore: map)]
fn possible_next(pos: Coord, map: &HashSet<Coord>) -> Vec<Coord> {
    [pos + LEFT, pos + RIGHT, pos + UP, pos + DOWN]
        .into_iter()
        .filter(|p| map.contains(p))
        .collect()
}

pub fn part1((start, map): (Coord, HashSet<Coord>)) -> usize {
    visit(start, map, 64)
}

fn visit_all(start: Coord, map: HashSet<Coord>, max_iter: usize) -> usize {
    let mut reached: HashSet<Complex<isize>> = HashSet::default();
    let mut queue = Vec::from_iter([(0, start)]);
    while let Some((iteration, pos)) = queue.pop() {
        // dbg!(&iteration, &pos);
        if iteration == max_iter {
            reached.insert(pos);
            continue;
        }
        let step = iteration + 1;
        for p in possible_next(pos, &map) {
            queue.push((step, p))
        }
    }
    println!("{:?}", reached);
    reached.len()
}

fn visit(start: Coord, map: HashSet<Coord>, max_iter: usize) -> usize {
    let mut seen: HashMap<Coord, usize> = HashMap::default();
    let mut queue = VecDeque::from_iter([(0, start)]);
    while let Some((mut iteration, pos)) = queue.pop_front() {
        let should_continue = match seen.get_mut(&pos) {
            Some(step) => {
                let min = *step.min(&mut iteration);
                if min < *step {
                    *step = *step.min(&mut iteration);
                    true
                } else {
                    false
                }
            }
            None => {
                seen.insert(pos, iteration);
                true
            }
        };

        if !should_continue {
            continue;
        }
        let step = seen.get(&pos).unwrap_or(&iteration) + 1;
        if step <= max_iter {
            for p in possible_next(pos, &map) {
                queue.push_back((step, p));
            }
        }
    }
    let filtered = seen.into_iter().filter(|(_, n)| n % 2 == 0).collect_vec();
    filtered.len()
}

fn visit_p2(start: Coord, map: HashSet<Coord>, max_iter: usize) -> usize {
    let max_right = map.iter().map(|x| x.re).max().unwrap() + 1;
    let max_down = map.iter().map(|x| x.im).max().unwrap() + 1;
    let mut seen: HashMap<Coord, usize> = HashMap::default();
    let mut queue = VecDeque::from_iter([(0, start)]);
    while let Some((mut iteration, pos)) = queue.pop_front() {
        let should_continue = match seen.get_mut(&pos) {
            Some(step) => {
                let min = *step.min(&mut iteration);
                if min < *step {
                    *step = *step.min(&mut iteration);
                    true
                } else {
                    false
                }
            }
            None => {
                seen.insert(pos, iteration);
                true
            }
        };

        if !should_continue {
            continue;
        }
        let step = seen.get(&pos).unwrap_or(&iteration) + 1;
        if step <= max_iter {
            for p in possible_next_p2(pos, &map, Complex::new(max_right, max_down)) {
                queue.push_back((step, p));
            }
        }
    }
    let filtered = seen.into_iter().filter(|(_, n)| n % 2 == 0).collect_vec();
    filtered.len()
}

#[memoize(Ignore: map)]
fn possible_next_p2(pos: Coord, map: &HashSet<Coord>, bottom_right: Coord) -> Vec<Coord> {
    [pos + LEFT, pos + RIGHT, pos + UP, pos + DOWN]
        .into_iter()
        .filter(|p| {
            let pos_modulo = Complex::new(
                p.re.rem_euclid(bottom_right.re),
                p.im.rem_euclid(bottom_right.im),
            );
            map.contains(&pos_modulo)
        })
        .collect()
}

pub fn part2((start, map): (Coord, HashSet<Coord>)) -> usize {
    for i in (0..1000).step_by(2) {
        println!("{i}, {}", visit_p2(start, map.clone(), i));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn data() -> &'static str {
        indoc! {
            "...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ..........."
        }
    }

    #[test]
    fn test_parse_input() {
        let input = data();
        let (start, map) = parse_input(input);

        assert_eq!(start, Complex::new(5, 5));
        assert!(map.contains(&Complex::new(0, 0)));
        assert!(map.contains(&Complex::new(5, 5)));
        assert!(!map.contains(&Complex::new(1, 2)));
    }

    #[test]
    fn test_visit() {
        let input = data();
        let (start, map) = parse_input(input);

        assert_eq!(visit(start, map.clone(), 1), 1);
        assert_eq!(visit(start, map.clone(), 2), 4);
        assert_eq!(visit(start, map.clone(), 6), 16);
    }

    #[test]
    fn test_part2() {
        let input = data();
        let (start, map) = parse_input(input);
        assert_eq!(visit_p2(start, map.clone(), 6), 16);
        assert_eq!(visit_p2(start, map.clone(), 10), 50);
        assert_eq!(visit_p2(start, map.clone(), 50), 1594);
        assert_eq!(visit_p2(start, map.clone(), 100), 6536);
        assert_eq!(visit_p2(start, map.clone(), 500), 167004);
    }
}
