use adventofcode_2023::day01;
use anyhow::Result;
use aoc_next::{aoc_main, parser, solution, solver, Aoc};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2023,
    solutions: &[
        solution! {1, parser!{ day01::parse_input }, solver!{ day01::part1 }},
        solution! {1, parser!{ day01::parse_input_p2 }, solver!{ day01::part1 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}
