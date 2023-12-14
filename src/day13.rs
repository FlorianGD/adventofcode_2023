use crate::helpers::transpose;

use itertools::equal;

type Matrix<T> = Vec<Vec<T>>;

pub fn parse_input(input: &str) -> Vec<Matrix<i8>> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            '.' => 0,
                            '#' => 1,
                            c => panic!("invalid input: {c:?}"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn find_symmetry(m: &Matrix<i8>) -> Option<usize> {
    let n = m.len();
    for span in 1..=n / 2 {
        if equal(m[0..span].iter(), m[span..2 * span].iter().rev()) {
            return Some(span);
        } else if equal(
            m[n - 2 * span..n - span].iter(),
            m[n - span..n].iter().rev(),
        ) {
            return Some(n - span);
        }
    }
    None
}

pub fn part1(input: Vec<Matrix<i8>>) -> usize {
    input
        .into_iter()
        .map(|m| match find_symmetry(&m) {
            Some(n) => 100 * n,
            None => match find_symmetry(&transpose(m)) {
                Some(n) => n,
                None => panic!("no symetry found"),
            },
        })
        .sum()
}

pub fn part2(input: Vec<Matrix<i8>>) -> usize {
    input
        .into_iter()
        .map(|m| match find_almost_symmetry(&m) {
            Some(n) => 100 * n,
            None => match find_almost_symmetry(&transpose(m)) {
                Some(n) => n,
                None => panic!("no symmetry found"),
            },
        })
        .sum()
}

fn find_almost_symmetry(m: &Matrix<i8>) -> Option<usize> {
    let n = m.len();
    for span in 1..=n / 2 {
        if (m[0..span].iter().zip(m[span..2 * span].iter().rev()))
            .map(|(v1, v2)| {
                v1.iter()
                    .zip(v2.iter())
                    .map(|(e1, e2)| (e1 - e2).abs())
                    .sum::<i8>()
            })
            .sum::<i8>()
            == 1
        {
            return Some(span);
        } else if (m[n - 2 * span..n - span]
            .iter()
            .zip(m[n - span..n].iter().rev()))
        .map(|(v1, v2)| {
            v1.iter()
                .zip(v2.iter())
                .map(|(e1, e2)| (e1 - e2).abs())
                .sum::<i8>()
        })
        .sum::<i8>()
            == 1
        {
            return Some(n - span);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "#.##..##.
        ..#.##.#.

        ..##..##.
        #.#.##.#.
        "};
        println!("{input}");
        let expected = vec![
            vec![
                vec![1, 0, 1, 1, 0, 0, 1, 1, 0],
                vec![0, 0, 1, 0, 1, 1, 0, 1, 0],
            ],
            vec![
                vec![0, 0, 1, 1, 0, 0, 1, 1, 0],
                vec![1, 0, 1, 0, 1, 1, 0, 1, 0],
            ],
        ];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_find_symmetry() {
        let input = indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
        };

        let parsed = parse_input(input);
        let symmetries = parsed.iter().map(find_symmetry).collect::<Vec<_>>();
        assert_eq!(symmetries, vec![None, Some(4)]);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
        };

        let parsed = parse_input(input);
        assert_eq!(part1(parsed), 405);
    }

    #[test]
    fn test_find_almost_symmetry() {
        let input = indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
        };

        let parsed = parse_input(input);
        let symmetries = parsed.iter().map(find_almost_symmetry).collect::<Vec<_>>();
        assert_eq!(symmetries, vec![Some(3), Some(1)]);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
        "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.

        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
        };

        let parsed = parse_input(input);
        assert_eq!(part2(parsed), 400);
    }
}
