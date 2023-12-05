use itertools::Itertools;
use std::ops::Range;
use winnow::ascii::{alpha1, digit1, line_ending, space1};
use winnow::combinator::{preceded, separated, separated_pair, terminated};
use winnow::{PResult, Parser};

type Map = (String, String, Vec<Mapping>);

trait Offset {
    fn offset(&self, offset: isize) -> Self;
}

impl Offset for Range<isize> {
    fn offset(&self, offset: isize) -> Self {
        Range {
            start: self.start + offset,
            end: self.end + offset,
        }
    }
}
trait Intersection {
    fn intersect_remainder(
        &self,
        other: &Range<isize>,
    ) -> (Option<Range<isize>>, Option<Vec<Range<isize>>>);
}

impl Intersection for Range<isize> {
    /// intersection of the ranges, remainder of self
    fn intersect_remainder(
        &self,
        other: &Range<isize>,
    ) -> (Option<Range<isize>>, Option<Vec<Range<isize>>>) {
        let (intersection, remainder);
        // no intersection
        if self.start >= other.end || self.end < other.start {
            intersection = None;
            remainder = Some(vec![self.clone()]);
        } else if self.start < other.start {
            // other included in self
            if self.end > other.end {
                intersection = Some(other.clone());
                remainder = Some(vec![self.start..other.start, other.end..self.end]);
            } else {
                // self.end <= other.end
                intersection = Some(other.start..self.end);
                remainder = Some(vec![self.start..other.start]);
            }
        } else if self.start > other.start {
            // self included in other
            if self.end <= other.end {
                intersection = Some(self.clone());
                remainder = None;
            } else {
                //self.end > other.end
                intersection = Some(self.start..other.end);
                remainder = Some(vec![other.end..self.end])
            }
        } else {
            // self.start == other.start
            if self.end <= other.end {
                intersection = Some(self.clone());
                remainder = None;
            } else {
                //self.end > other.end
                intersection = Some(self.start..other.end);
                remainder = Some(vec![other.end..self.end]);
            }
        }
        (intersection, remainder)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Mapping {
    source: Range<isize>,
    offset: isize,
}
impl Mapping {
    fn new(source: Range<isize>, offset: isize) -> Self {
        Mapping { source, offset }
    }
}

fn num(input: &mut &str) -> PResult<isize> {
    digit1.parse_to().parse_next(input)
}

fn seeds(input: &mut &str) -> PResult<Vec<isize>> {
    preceded("seeds: ", separated(1.., num, space1)).parse_next(input)
}

fn mapping(input: &mut &str) -> PResult<Mapping> {
    let (dest, source, len) = (num, preceded(" ", num), preceded(" ", num)).parse_next(input)?;
    Ok(Mapping::new(source..source + len, dest - source))
}

fn mappings(input: &mut &str) -> PResult<Vec<Mapping>> {
    separated(1.., mapping, line_ending).parse_next(input)
}

fn block(input: &mut &str) -> PResult<Map> {
    let (source_name, dest_name) = terminated(
        separated_pair(alpha1, "-to-", alpha1),
        (" map:", line_ending),
    )
    .parse_next(input)?;
    let mappings = mappings.parse_next(input)?;
    Ok((source_name.to_string(), dest_name.to_string(), mappings))
}

fn blocks(input: &mut &str) -> PResult<Vec<Map>> {
    separated(1.., block, (line_ending, line_ending)).parse_next(input)
}

pub fn parse_input(input: &str) -> (Vec<isize>, Vec<Map>) {
    let mut input = input;
    separated_pair(seeds, (line_ending, line_ending), blocks)
        .parse_next(&mut input)
        .unwrap()
}

fn next_parts(parts: &Vec<isize>, maps: &Vec<Mapping>) -> Vec<isize> {
    let mut res = Vec::with_capacity(parts.len());
    let mut parts = parts.clone();
    for m in maps {
        parts
            // .into_iter()
            .retain(|part| {
                if m.source.contains(part) {
                    res.push(part + m.offset);
                    return false;
                }
                true
            });
    }
    ([res, parts]).concat()
}

pub fn part1((seeds, maps): (Vec<isize>, Vec<Map>)) -> isize {
    let mut new_parts = seeds.clone();
    for (_, _, map) in maps {
        new_parts = next_parts(&new_parts, &map);
    }
    if let Some(&m) = new_parts.iter().min() {
        m
    } else {
        panic!("no result")
    }
}

fn next_parts_p2(parts: &Vec<Range<isize>>, maps: &Vec<Mapping>) -> Vec<Range<isize>> {
    let mut res = Vec::new();
    let mut parts = parts.clone();

    while let Some(current_part) = parts.pop() {
        dbg!(&current_part);
        for m in maps {
            match current_part.intersect_remainder(&m.source) {
                (None, _) => {
                    continue;
                }
                (Some(x), None) => {
                    res.push(x.offset(m.offset));
                    break;
                }
                (Some(x), Some(rem)) => {
                    res.push(x.offset(m.offset));
                    for r in rem {
                        parts.push(r);
                    }
                    break;
                }
            }
        }
        res.push(current_part);
    }

    res
}

pub fn part2((seeds, maps): (Vec<isize>, Vec<Map>)) -> isize {
    let mut new_seeds = Vec::new();
    dbg!(&new_seeds);
    dbg!(&seeds);
    while let Some((x, y)) = seeds.clone().into_iter().next_tuple() {
        new_seeds.push(x..x + y);
    }
    dbg!(&new_seeds);
    for (_, _, map) in maps {
        new_seeds = next_parts_p2(&new_seeds, &map);
        dbg!(&new_seeds);
    }
    if let Some(m) = new_seeds.iter().min_by_key(|x| x.start) {
        m.start
    } else {
        panic!("no result")
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_intersect_remainder() {
        let me = 0isize..2;
        let other = 0isize..2;
        assert_eq!(me.intersect_remainder(&other), (Some(0..2), None));
        let me = 0isize..1;
        let other = 0isize..2;
        assert_eq!(me.intersect_remainder(&other), (Some(0..1), None));
        let me = 0isize..3;
        let other = 0isize..2;
        assert_eq!(
            me.intersect_remainder(&other),
            (Some(0..2), Some(vec![2..3]))
        );
        let me = -1isize..3;
        let other = 0isize..2;
        assert_eq!(
            me.intersect_remainder(&other),
            (Some(0..2), Some(vec![-1..0, 2..3]))
        );
        let me = 2isize..3;
        let other = 0isize..2;
        assert_eq!(me.intersect_remainder(&other), (None, Some(vec![2..3])));
        let me = 2isize..3;
        let other = 0isize..1;
        assert_eq!(me.intersect_remainder(&other), (None, Some(vec![2..3])));
    }
    #[test]
    fn test_offset() {
        let me = 0isize..2;
        assert_eq!(me.offset(1), 1..3);
        assert_eq!(me.offset(-1), -1..1);
        assert_eq!(me.offset(0), 0..2);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
          "seeds: 79 14 55 13

          seed-to-soil map:
          50 98 2
          52 50 48
          
          soil-to-fertilizer map:
          0 15 37
          37 52 2
          39 0 15
          
          fertilizer-to-water map:
          49 53 8
          0 11 42
          42 0 7
          57 7 4
          
          water-to-light map:
          88 18 7
          18 25 70
          
          light-to-temperature map:
          45 77 23
          81 45 19
          68 64 13
          
          temperature-to-humidity map:
          0 69 1
          1 0 69
          
          humidity-to-location map:
          60 56 37
          56 93 4"
        };
        let (seeds, blocks) = parse_input(input);
        let result = part1((seeds, blocks));
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
          "seeds: 79 14 55 13

          seed-to-soil map:
          50 98 2
          52 50 48
          
          soil-to-fertilizer map:
          0 15 37
          37 52 2
          39 0 15
          
          fertilizer-to-water map:
          49 53 8
          0 11 42
          42 0 7
          57 7 4
          
          water-to-light map:
          88 18 7
          18 25 70
          
          light-to-temperature map:
          45 77 23
          81 45 19
          68 64 13
          
          temperature-to-humidity map:
          0 69 1
          1 0 69
          
          humidity-to-location map:
          60 56 37
          56 93 4"
        };
        let (seeds, blocks) = parse_input(input);
        let result = part2((seeds, blocks));
        assert_eq!(result, 46);
    }

    #[test]
    fn test_next_parts() {
        let input = indoc! {
        "seeds: 79 14 55 13
        
        seed-to-soil map:
        50 98 2
        52 50 48"};
        let (seeds, blocks) = parse_input(input);
        let expected = HashSet::from([81, 14, 57, 13]);
        let (_, _, maps) = &blocks[0];
        assert_eq!(HashSet::from_iter(next_parts(&seeds, maps)), expected);
    }

    #[test]
    fn test_seeds() {
        let mut input = "seeds: 2 3 6";
        let expected = vec![2, 3, 6];
        assert_eq!(seeds(&mut input), Ok(expected));
    }

    #[test]
    fn test_mapping() {
        let mut input = "2 3 6";
        let expected = Mapping::new(3..9, -1);
        assert_eq!(mapping(&mut input), Ok(expected));
    }

    #[test]
    fn test_mappings() {
        let mut input = indoc! {
        "50 98 2
        52 50 48"};
        let expected = vec![Mapping::new(98..100, -48), Mapping::new(50..98, 2)];
        assert_eq!(mappings(&mut input), Ok(expected));
    }

    #[test]
    fn test_block() {
        let mut input = indoc! {
        "seed-to-soil map:
        50 98 2
        52 50 48"};
        let expected = vec![Mapping::new(98..100, -48), Mapping::new(50..98, 2)];
        assert_eq!(
            block(&mut input),
            Ok(("seed".to_string(), "soil".to_string(), expected))
        );
    }
    #[test]
    fn test_blocks() {
        let mut input = indoc! {
        "seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4"};
        let expected = vec![
            (
                "seed".to_string(),
                "soil".to_string(),
                vec![Mapping::new(98..100, -48), Mapping::new(50..98, 2)],
            ),
            (
                "soil".to_string(),
                "fertilizer".to_string(),
                vec![
                    Mapping::new(15..52, -15),
                    Mapping::new(52..54, -15),
                    Mapping::new(0..15, 39),
                ],
            ),
            (
                "fertilizer".to_string(),
                "water".to_string(),
                vec![
                    Mapping::new(53..61, -4),
                    Mapping::new(11..53, -11),
                    Mapping::new(0..7, 42),
                    Mapping::new(7..11, 50),
                ],
            ),
        ];
        assert_eq!(blocks(&mut input), Ok(expected));
    }
    #[test]
    fn test_parse_input() {
        let mut input = indoc! {
        "seeds: 79 14 55 13
        
        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4"};
        let expected = vec![
            (
                "seed".to_string(),
                "soil".to_string(),
                vec![Mapping::new(98..100, -48), Mapping::new(50..98, 2)],
            ),
            (
                "soil".to_string(),
                "fertilizer".to_string(),
                vec![
                    Mapping::new(15..52, -15),
                    Mapping::new(52..54, -15),
                    Mapping::new(0..15, 39),
                ],
            ),
            (
                "fertilizer".to_string(),
                "water".to_string(),
                vec![
                    Mapping::new(53..61, -4),
                    Mapping::new(11..53, -11),
                    Mapping::new(0..7, 42),
                    Mapping::new(7..11, 50),
                ],
            ),
        ];
        assert_eq!(parse_input(input), (vec![79, 14, 55, 13], expected));
    }
}
