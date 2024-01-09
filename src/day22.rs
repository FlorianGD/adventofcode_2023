use itertools::Itertools;

use std::{fmt::Debug, str::FromStr};
use winnow::{
    combinator::{separated_pair, terminated},
    PResult, Parser,
};

use crate::parsers::num;

type Coord = (isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Span {
    None,
    X(isize),
    Y(isize),
    Z(isize),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Brick {
    coord: Coord,
    span: Span,
}

impl Debug for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Brick ({}, {}, {}), span={:?}",
            self.coord.0, self.coord.1, self.coord.2, self.span
        )?;
        Ok(())
    }
}

impl Brick {
    fn new(coord: Coord, span: Span) -> Self {
        Brick { coord, span }
    }

    fn from_coords(p1: Coord, p2: Coord) -> Brick {
        let span = match (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2) {
            (0, 0, 0) => Span::None,
            (n, 0, 0) if n > 0 => Span::X(n),
            (0, n, 0) if n > 0 => Span::Y(n),
            (0, 0, n) if n > 0 => Span::Z(n),
            _ => panic!("Weird span for brick {p1:?}, {p2:?}"),
        };
        Brick::new(p1, span)
    }

    /// None if not on the same plane, Some(i), i > 0 if self is above other
    fn distance_z(&self, other: &Self) -> Option<isize> {
        let (x1, y1, z1) = self.coord;
        let (x2, y2, z2) = other.coord;
        match (self.span, other.span) {
            (Span::None, Span::None) => {
                if x1 == x2 && y1 == y2 {
                    Some(z1 - z2)
                } else {
                    None
                }
            }
            (Span::None, Span::X(n)) => {
                if y1 == y2 && x1 >= x2 && x1 <= x2 + n {
                    Some(z1 - z2)
                } else {
                    None
                }
            }
            (Span::None, Span::Y(n)) => {
                if x1 == x2 && y1 >= y2 && y1 <= y2 + n {
                    Some(z1 - z2)
                } else {
                    None
                }
            }
            (Span::None, Span::Z(n)) => {
                if x1 == x2 && y1 == y2 {
                    if z1 > z2 {
                        Some(z1 - (z2 + n))
                    } else {
                        Some(z1 - z2)
                    }
                } else {
                    None
                }
            }
            (Span::Z(_), Span::None) => other.distance_z(self).map(|i| -i),
            (Span::X(_), Span::None) => other.distance_z(self).map(|i| -i),
            (Span::Y(_), Span::None) => other.distance_z(self).map(|i| -i),
            (Span::Z(n1), Span::Z(n2)) => {
                if x1 == x2 && y1 == y2 {
                    if z1 > z2 {
                        Some(z1 - (z2 + n2))
                    } else {
                        Some(z1 + n1 - z2)
                    }
                } else {
                    None
                }
            }
            (Span::Z(n1), Span::X(n2)) => {
                if y1 == y2 && x1 >= x2 && x1 <= x2 + n2 {
                    if z1 > z2 {
                        Some(z1 - z2)
                    } else {
                        Some(z1 + n1 - z2)
                    }
                } else {
                    None
                }
            }
            (Span::X(_), Span::Z(_)) => other.distance_z(self).map(|i| -i),
            (Span::Z(n1), Span::Y(n2)) => {
                if x1 == x2 && y1 >= y2 && y1 <= y2 + n2 {
                    if z1 > z2 {
                        Some(z1 - z2)
                    } else {
                        Some(z1 + n1 - z2)
                    }
                } else {
                    None
                }
            }
            (Span::Y(_), Span::Z(_)) => other.distance_z(self).map(|i| -i),
            (Span::X(n1), Span::X(n2)) => {
                if y1 == y2 {
                    if (x1 >= x2 && x1 <= x1 + n2) || (x2 >= x1 && x2 <= x2 + n1) {
                        Some(z1 - z2)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            (Span::Y(n1), Span::Y(n2)) => {
                if x1 == x2 {
                    if (y1 >= y2 && y1 <= y1 + n2) || (y2 >= y1 && y2 <= y2 + n1) {
                        Some(z1 - z2)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            (Span::X(n1), Span::Y(n2)) => {
                if y1 >= y2 && y1 <= y2 + n2 && x2 >= x1 && x2 <= x1 + n1 {
                    Some(z1 - z2)
                } else {
                    None
                }
            }
            (Span::Y(_), Span::X(_)) => other.distance_z(self).map(|i| -i),
        }
    }
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s;

        if let Ok((p1, p2)) = parse_brick(&mut input) {
            Ok(Brick::from_coords(p1, p2))
        } else {
            Err(format!("Could not parse brick {s}"))
        }
    }
}

fn parse_coord(input: &mut &str) -> PResult<Coord> {
    (
        terminated(num::<isize>, ','),
        terminated(num::<isize>, ','),
        num::<isize>,
    )
        .parse_next(input)
}

fn parse_brick(input: &mut &str) -> PResult<(Coord, Coord)> {
    separated_pair(parse_coord, '~', parse_coord).parse_next(input)
}

pub fn parse_input(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        .sorted_by_key(|x: &Brick| x.coord.2)
        .collect()
}

fn settle_down(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut new_bricks = Vec::default();
    for b in bricks {
        if b.coord.2 == 1 {
            new_bricks.push(b.clone());
        } else if let Some(n) = new_bricks.iter().filter_map(|br| b.distance_z(br)).min() {
            new_bricks.push(Brick::new(
                (b.coord.0, b.coord.1, b.coord.2 - n + 1),
                b.span,
            ))
        } else {
            new_bricks.push(Brick::new((b.coord.0, b.coord.1, 1), b.span));
        }
    }
    new_bricks
}

pub fn part1(bricks: Vec<Brick>) -> usize {
    let bricks = settle_down(&bricks)
        .into_iter()
        .sorted_by_key(|x: &Brick| x.coord.2)
        .collect_vec();

    bricks
        .iter()
        .map(|b| {
            let brick_level = match b.span {
                Span::Z(n) => b.coord.2 + n,
                _ => b.coord.2,
            };
            let above = bricks
                .iter()
                .filter(|br| br.coord.2 == brick_level + 1)
                .collect_vec();
            dbg!(&above);
            if above.is_empty() {
                println!("Safe to discard {b:?}");
                1
            } else {
                let same_level = bricks
                    .iter()
                    .filter(|br| {
                        br != &b
                            && match br.span {
                                Span::Z(n) => br.coord.2 + n == brick_level,
                                _ => br.coord.2 == brick_level,
                            }
                    })
                    .collect_vec();
                if same_level.is_empty() {
                    0
                } else {
                    dbg!(&same_level);
                    if above.iter().any(|&brick_above| {
                        same_level.iter().all(|&brick_same_level| {
                            brick_above.distance_z(brick_same_level).is_none()
                        })
                    }) {
                        println!("NOT Safe to discard {b:?}");
                        dbg!(above
                            .iter()
                            .filter(|&brick_above| {
                                same_level.iter().all(|&brick_same_level| {
                                    brick_above.distance_z(brick_same_level).is_none()
                                })
                            })
                            .collect_vec());
                        0
                    } else {
                        println!("Safe to discard {b:?}");
                        1
                    }
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn data() -> &'static str {
        indoc! {
            "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9"
        }
    }
    #[test]
    fn test_parse_input() {
        let input = data();
        let parsed = parse_input(input);
        assert_eq!(parsed.len(), 7);
        assert_eq!(parsed[0], Brick::from_coords((1, 0, 1), (1, 2, 1)));
    }
    #[test]
    fn test_distance_z_span_none() {
        let b = Brick::new((1, 0, 1), Span::None);
        let b2 = Brick::new((1, 0, 3), Span::None);
        assert_eq!(b.distance_z(&b2), Some(-2));
        let b2 = Brick::new((1, 1, 3), Span::None);
        assert_eq!(b.distance_z(&b2), None);
        let b2 = Brick::new((2, 0, 3), Span::None);
        assert_eq!(b.distance_z(&b2), None);
        let b2 = Brick::new((1, 0, 2), Span::Z(1));
        assert_eq!(b.distance_z(&b2), Some(-1));
        let b2 = Brick::new((1, 0, 6), Span::None);
        let b3 = Brick::new((1, 0, 2), Span::Z(2));
        assert_eq!(b2.distance_z(&b3), Some(2));
    }

    #[test]
    fn test_settle_down() {
        let input = data();
        let parsed = parse_input(input);
        let settled = settle_down(&parsed);
        let expected = parse_input(indoc! {
            "1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,2~2,2,2
            0,0,3~0,2,3
            2,0,3~2,2,3
            0,1,4~2,1,4
            1,1,5~1,1,6"
        });
        assert_eq!(settled, expected);
    }

    #[test]
    fn test_part1() {
        let input = data();
        let bricks = parse_input(input);
        assert_eq!(part1(bricks), 5);
    }
}
