use adventofcode_2023::{day01, day02, day03, day04, day05};
use anyhow::Result;
use aoc_next::{aoc_main, parser, solution, solver, Aoc};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2023,
    solutions: &[
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, parser!{ day01::parse_input_p2 }, solver!{ day01::part1 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part1 }},
        solution! {2, parser!{ day02::parse_input }, solver!{ day02::part2 }},
        solution! {3, parser!{ day03::parse_input }, solver!{ day03::part1 }},
        solution! {3, parser!{ day03::parse_input }, solver!{ day03::part2 }},
        solution! {4, parser!{ day04::parse_input }, solver!{ day04::part1 }},
        solution! {4, parser!{ day04::parse_input }, solver!{ day04::part2 }},
        solution! {5, parser!{ day05::parse_input }, solver!{ day05::part1 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
