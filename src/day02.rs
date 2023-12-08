use anyhow::Result;
use rustc_hash::FxHashMap as HashMap;
use std::str::FromStr;
use winnow::{
    ascii::digit1,
    combinator::{alt, delimited, separated, terminated},
    PResult, Parser,
};

pub type Draws = Vec<Vec<Color>>;

pub struct Game {
    id: u32,
    draws: Draws,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        line.parse(s).map_err(|e| e.to_string())
    }
}
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn line(input: &mut &str) -> PResult<Game> {
    let (id, draws) = (parse_game, draws).parse_next(input)?;
    Ok(Game { id, draws })
}

fn draws(input: &mut &str) -> PResult<Draws> {
    separated(1.., colors, "; ").parse_next(input)
}

fn colors(input: &mut &str) -> PResult<Vec<Color>> {
    separated(1.., color, ", ").parse_next(input)
}

fn parse_game(input: &mut &str) -> PResult<u32> {
    delimited("Game ", digit1, ": ")
        .parse_to()
        .parse_next(input)
}

fn green(input: &mut &str) -> PResult<Color> {
    let val = terminated(digit1, " green").parse_to().parse_next(input)?;
    Ok(Color::Green(val))
}
fn red(input: &mut &str) -> PResult<Color> {
    let val = terminated(digit1, " red").parse_to().parse_next(input)?;
    Ok(Color::Red(val))
}
fn blue(input: &mut &str) -> PResult<Color> {
    let val = terminated(digit1, " blue").parse_to().parse_next(input)?;
    Ok(Color::Blue(val))
}

fn color(input: &mut &str) -> PResult<Color> {
    alt((red, green, blue)).parse_next(input)
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| l.parse().unwrap())
        // .map(|mut l| {
        //     let game_id = parse_prefix(&mut l).unwrap();
        // })
        .collect()
}

pub fn part1(games: Vec<Game>) -> u32 {
    games
        .into_iter()
        .filter_map(|g| {
            if g.draws.into_iter().all(|cs| {
                cs.into_iter().all(|c| match c {
                    Color::Red(r) => r <= 12,
                    Color::Green(g) => g <= 13,
                    Color::Blue(b) => b <= 14,
                })
            }) {
                Some(g.id)
            } else {
                None
            }
        })
        .sum()
}
pub fn part2(games: Vec<Game>) -> u32 {
    games
        .into_iter()
        .map(|g| {
            let mut res = HashMap::from_iter([
                (Color::Red(0), 0_u32),
                (Color::Green(0), 0_u32),
                (Color::Blue(0), 0_u32),
            ]);
            g.draws.into_iter().for_each(|cs| {
                cs.into_iter().for_each(|c| match c {
                    Color::Red(r) => {
                        if let Some(x) = res.get_mut(&Color::Red(0)) {
                            *x = r.max(*x)
                        };
                    }
                    Color::Green(g) => {
                        if let Some(x) = res.get_mut(&Color::Green(0)) {
                            *x = g.max(*x)
                        };
                    }
                    Color::Blue(b) => {
                        if let Some(x) = res.get_mut(&Color::Blue(0)) {
                            *x = b.max(*x)
                        };
                    }
                });
            });
            res.values().product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let games = parse_input(input);
        assert_eq!(part1(games), 8);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};
        let games = parse_input(input);
        assert_eq!(part2(games), 2286);
    }

    #[test]
    fn test_parse_game() {
        let mut input = "Game 1: ";
        let result = parse_game(&mut input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_green() {
        let mut input = "1 green";
        let result = green(&mut input);
        assert_eq!(result, Ok(Color::Green(1)));
    }
    #[test]
    fn test_color() {
        let mut input = "1 green";
        let result = color(&mut input);
        assert_eq!(result, Ok(Color::Green(1)));
    }

    #[test]
    fn test_color_red() {
        let mut input = "11 red";
        let result = color(&mut input);
        assert_eq!(result, Ok(Color::Red(11)));
    }

    #[test]
    fn test_colors() {
        let mut input = "11 red, 2 blue, 3 green";
        let result = colors(&mut input);
        assert_eq!(
            result,
            Ok(vec![Color::Red(11), Color::Blue(2), Color::Green(3)])
        );
    }
    #[test]
    fn test_draws() {
        let mut input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let result = draws(&mut input);
        assert_eq!(
            result,
            Ok(vec![
                vec![Color::Blue(3), Color::Red(4)],
                vec![Color::Red(1), Color::Green(2), Color::Blue(6)],
                vec![Color::Green(2)]
            ])
        );
    }

    // #[test]
    // fn test_bag() {
    //     let mut input = "12 green, 1 red, 2 blue";
    //     let result = parse_bag(&mut input);
    //     assert_eq!(
    //         result,
    //         Ok((
    //             input,
    //             Bag {
    //                 red: 1,
    //                 green: 12,
    //                 blue: 2
    //             }
    //         ))
    //     );
    // }
    // #[test]
    // fn test_bag_simple() {
    //     let mut input = "12 red, 1 blue";
    //     let result = parse_bag_p(&mut input);
    //     assert_eq!(result, Ok((input, (12, 0, 1))));
    // }
}
