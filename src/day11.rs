use itertools::Itertools;
use num::complex::Complex;

type Coord = Complex<isize>;
pub fn parse_input(input: &str) -> Vec<Coord> {
    let mut v_offset = 0;
    let mut galaxies = vec![];
    input.lines().enumerate().for_each(|(j, line)| {
        if line.chars().all(|c| c == '.') {
            v_offset += 1;
        } else {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .for_each(|(i, _)| {
                    galaxies.push(Complex::new(i, j + v_offset));
                })
        }
    });
    // expand horizontally
    let h_len = input.lines().next().unwrap().len();
    let empty_cols = (0..h_len)
        .filter(|i| !galaxies.iter().any(|g| g.re == *i))
        .collect::<Vec<_>>();
    galaxies
        .into_iter()
        .map(|c| {
            let h_offset = empty_cols.iter().filter(|&i| *i < c.re).count();
            Complex::new((c.re + h_offset) as isize, c.im as isize)
        })
        .collect()
}

pub fn part1(galaxies: Vec<Coord>) -> isize {
    galaxies
        .into_iter()
        .combinations(2)
        .map(|a| (a[1] - a[0]).l1_norm())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = indoc! {
        "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "};
        let parsed = parse_input(input);
        assert_eq!(part1(parsed), 374);
    }

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
        "};
        // 0000000000111
        // 0123456789012
        // ....1........
        // .........2...
        // 3............
        // .............
        // .............
        // ........4....
        // .5...........
        // ............6
        // .............
        // .............
        // .........7...
        // 8....9.......
        let parsed = parse_input(input);
        assert_eq!(
            parsed,
            vec![
                Complex::new(4, 0),
                Complex::new(9, 1),
                Complex::new(0, 2),
                Complex::new(8, 5),
                Complex::new(1, 6),
                Complex::new(12, 7),
                Complex::new(9, 10),
                Complex::new(0, 11),
                Complex::new(5, 11),
            ]
        );
    }
}
