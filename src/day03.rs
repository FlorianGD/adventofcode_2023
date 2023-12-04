use num::complex::Complex;
use std::collections::{HashMap, HashSet};
type Coord = Complex<isize>;

type Symbols = HashMap<Coord, char>;
type Numbers = Vec<BBox>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct BBox {
    left: Coord,
    val: String,
}

impl BBox {
    fn new(left: Coord, val: &str) -> Self {
        BBox {
            left,
            val: val.to_string(),
        }
    }
    fn surroundings(&self) -> HashSet<Coord> {
        let mut res = HashSet::new();
        let right_limit = self.val.len() as isize + self.left.re;
        for p in self.left.re..right_limit {
            for j in -1..=1 {
                for i in -1..=1 {
                    if j == 0 && (self.left.re..right_limit).contains(&(p + i)) {
                        continue;
                    }
                    res.insert(Complex::new(p + i, self.left.im + j));
                }
            }
        }
        res
    }

    fn is_part_number(&self, symbols: &Symbols) -> bool {
        self.surroundings().iter().any(|c| symbols.contains_key(c))
    }
}

pub fn parse_input(input: &str) -> (Symbols, Numbers) {
    let mut symbols: HashMap<Complex<isize>, char> = HashMap::new();
    let mut numbers = Vec::new();
    input.lines().enumerate().for_each(|(j, line)| {
        line.chars()
            .enumerate()
            .filter(|&(_, c)| !c.is_ascii_digit() && c != '.')
            .for_each(|(i, c)| {
                symbols.insert(Complex::new(i as isize, j as isize), c);
            });
        let mut val = String::new();
        let mut start = None;
        let mut prev: isize = -1;
        for (i, c) in line
            .chars()
            .enumerate()
            .filter(|&(_, c)| c.is_ascii_digit())
        {
            if start.is_none() {
                start = Some(Complex::new(i as isize, j as isize));
                val.push(c);
                prev = i as isize;
                continue;
            }
            if i as isize == prev + 1 && !val.is_empty() {
                val.push(c);
                prev += 1;
            } else {
                let bbox = BBox::new(start.unwrap(), &val);
                numbers.push(bbox);
                start = Some(Complex::new(i as isize, j as isize));
                val = format!("{}", c);
                prev = i as isize;
            }
        }
        if !val.is_empty() {
            let bbox = BBox::new(start.unwrap(), &val);
            numbers.push(bbox);
        }
    });
    (symbols, numbers)
}

pub fn part1((symbols, numbers): (Symbols, Numbers)) -> usize {
    numbers
        .into_iter()
        .filter(|b| b.is_part_number(&symbols))
        .map(|b| b.val.parse::<usize>().unwrap())
        .sum()
}

pub fn part2((symbols, numbers): (Symbols, Numbers)) -> usize {
    let stars: Symbols = symbols.into_iter().filter(|(_, c)| *c == '*').collect();
    let mut hm = HashMap::new();
    for bbox in numbers {
        for s in bbox.surroundings() {
            if stars.contains_key(&s) {
                hm.entry(s).or_insert(Vec::new()).push(bbox.val.to_string())
            }
        }
    }
    hm.values()
        .filter(|v| v.len() == 2)
        .map(|v| {
            v.iter()
                .map(|x| x.parse::<usize>().unwrap())
                .product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_surrondings() {
        let left = Complex::new(1, 1);
        let bbox = BBox::new(left, "123");
        let s = bbox.surroundings();
        assert_eq!(
            s,
            HashSet::from_iter([
                Complex { re: 0, im: 0 },
                Complex { re: 0, im: 1 },
                Complex { re: 0, im: 2 },
                Complex { re: 1, im: 0 },
                Complex { re: 1, im: 2 },
                Complex { re: 2, im: 0 },
                Complex { re: 2, im: 2 },
                Complex { re: 3, im: 0 },
                Complex { re: 3, im: 2 },
                Complex { re: 4, im: 0 },
                Complex { re: 4, im: 1 },
                Complex { re: 4, im: 2 },
            ])
        );
    }
    #[test]
    fn test_parse() {
        let input = indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        };
        let (symbols, numbers) = parse_input(input);
        let expected_symbols = HashMap::from([
            (Complex { re: 5, im: 8 }, '*'),
            (Complex { re: 3, im: 1 }, '*'),
            (Complex { re: 6, im: 3 }, '#'),
            (Complex { re: 3, im: 4 }, '*'),
            (Complex { re: 3, im: 8 }, '$'),
            (Complex { re: 5, im: 5 }, '+'),
        ]);
        assert_eq!(symbols, expected_symbols);
        let expected_numbers = [
            BBox {
                left: Complex { re: 0, im: 0 },
                val: "467".to_string(),
            },
            BBox {
                left: Complex { re: 5, im: 0 },
                val: "114".to_string(),
            },
            BBox {
                left: Complex { re: 2, im: 2 },
                val: "35".to_string(),
            },
            BBox {
                left: Complex { re: 6, im: 2 },
                val: "633".to_string(),
            },
            BBox {
                left: Complex { re: 0, im: 4 },
                val: "617".to_string(),
            },
            BBox {
                left: Complex { re: 7, im: 5 },
                val: "58".to_string(),
            },
            BBox {
                left: Complex { re: 2, im: 6 },
                val: "592".to_string(),
            },
            BBox {
                left: Complex { re: 6, im: 7 },
                val: "755".to_string(),
            },
            BBox {
                left: Complex { re: 1, im: 9 },
                val: "664".to_string(),
            },
            BBox {
                left: Complex { re: 5, im: 9 },
                val: "598".to_string(),
            },
        ];
        assert_eq!(numbers, expected_numbers);
    }
    #[test]
    fn test_par1() {
        let input = indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        };
        let result = part1(parse_input(input));
        assert_eq!(result, 4361);
    }
    #[test]
    fn test_par2() {
        let input = indoc! {
            "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
        };
        let result = part2(parse_input(input));
        assert_eq!(result, 467835);
    }
}
