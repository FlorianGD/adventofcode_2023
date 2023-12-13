use itertools::equal;

type Matrix = Vec<Vec<i8>>;

pub fn parse_input(input: &str) -> Vec<Matrix> {
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
                            _ => panic!("invalid input"),
                        })
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn find_symmetry(m: &Matrix) -> Option<usize> {
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

//https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part1(input: Vec<Matrix>) -> usize {
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
        let symetries = parsed.iter().map(|b| find_symmetry(b)).collect::<Vec<_>>();
        assert_eq!(symetries, vec![None, Some(4)]);
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
}
