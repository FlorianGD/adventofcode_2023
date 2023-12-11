use itertools::Itertools;
use num::complex::Complex;

type Coord = Complex<isize>;

fn expand_galaxies(galaxies: Vec<Coord>, offset: isize) -> Vec<Coord> {
    let h_len = galaxies.iter().map(|c| c.re).max().unwrap();
    let v_len = galaxies.iter().map(|c| c.im).max().unwrap();
    let empty_cols = (0..h_len)
        .filter(|i| !galaxies.iter().any(|g| g.re == *i))
        .collect::<Vec<_>>();
    let empty_rows = (0..v_len)
        .filter(|i| !galaxies.iter().any(|g| g.im == *i))
        .collect::<Vec<_>>();
    //expand
    galaxies
        .into_iter()
        .map(|c| {
            let h_offset = empty_cols.iter().filter(|&i| *i < c.re).count() as isize;
            let v_offset = empty_rows.iter().filter(|&j| *j < c.im).count() as isize;
            Complex::new(
                c.re + h_offset * (offset - 1),
                c.im + v_offset * (offset - 1),
            )
        })
        .collect()
}

pub fn parse_input(input: &str) -> Vec<Coord> {
    let mut galaxies = vec![];
    input.lines().enumerate().for_each(|(j, line)| {
        line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .for_each(|(i, _)| {
                galaxies.push(Complex::new(i as isize, j as isize));
            })
    });
    galaxies
}

pub fn part1(galaxies: Vec<Coord>) -> isize {
    let galaxies = expand_galaxies(galaxies, 2);
    galaxies
        .into_iter()
        .combinations(2)
        .map(|a| (a[1] - a[0]).l1_norm())
        .sum()
}

pub fn part2(galaxies: Vec<Coord>) -> isize {
    let galaxies = expand_galaxies(galaxies, 1_000_000);
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
            expand_galaxies(parsed, 2),
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
