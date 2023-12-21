use num::complex::Complex;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
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

fn possible_next(pos: Coord, map: &HashSet<Coord>) -> Vec<Coord> {
    let mut n = vec![];
    for p in [pos + LEFT, pos + RIGHT, pos + UP, pos + DOWN] {
        if map.contains(&p) {
            n.push(p);
        }
    }
    n
}
// A tester: regarder en 10 steps là où on peut aller en étant exhaustif
// puis avec les min de reach et trouver la relation magique
pub fn part1((start, map): (Coord, HashSet<Coord>)) -> usize {
    visit(start, map, 10)
}
fn visit(start: Coord, map: HashSet<Coord>, max_iter: usize) -> usize {
    let mut seen: HashMap<Coord, usize> = HashMap::default();
    let mut queue = Vec::from_iter([(0, start)]);
    while let Some((mut iteration, pos)) = queue.pop() {
        if iteration > max_iter {
            continue;
        }
        match seen.get_mut(&pos) {
            Some(step) => {
                *step = *step.min(&mut iteration);
                continue;
            }
            None => {
                seen.insert(pos, iteration);
            }
        }

        let step = iteration + 1;
        for p in possible_next(pos, &map) {
            queue.push((step, p));
        }
    }
    println!("{:?}", seen);
    seen.into_iter().filter(|(_, n)| n % 2 == 0).count()
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

        assert_eq!(visit(start, map.clone(), 1), 2);
        assert_eq!(visit(start, map.clone(), 2), 4);
        assert_eq!(visit(start, map.clone(), 6), 16);
    }
}
