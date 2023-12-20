use std::collections::HashMap;
use winnow::{
    ascii::alpha1,
    combinator::{alt, preceded, repeat, separated_pair},
    PResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    On,
    Off,
}

#[derive(Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}
type Memory = HashMap<String, Pulse>;
#[derive(Debug, Clone)]
enum ModuleKind {
    Broadcaster,
    FlipFlop(Status),
    Conjunction(Memory),
}

#[derive(Debug, Clone)]
pub struct Module {
    destinations: Vec<String>,
    kind: ModuleKind,
}

fn destinations(input: &mut &str) -> PResult<Vec<String>> {
    let first = alpha1.parse_next(input)?;
    let rest: Vec<&str> = repeat(0.., preceded(", ", alpha1)).parse_next(input)?;
    let mut dests = vec![first];
    dests.extend(rest.into_iter());
    Ok(dests.into_iter().map(|l| l.to_string()).collect())
}

fn parse_broadcaster(input: &mut &str) -> PResult<(String, Module)> {
    let dests = preceded("broadcaster -> ", destinations).parse_next(input)?;
    Ok((
        "broadcaster".to_string(),
        Module {
            kind: ModuleKind::Broadcaster,
            destinations: dests,
        },
    ))
}

fn parse_flip_flop(input: &mut &str) -> PResult<(String, Module)> {
    let (name, dests) =
        separated_pair(preceded('%', alpha1), " -> ", destinations).parse_next(input)?;
    Ok((
        name.to_string(),
        Module {
            kind: ModuleKind::FlipFlop(Status::Off),
            destinations: dests,
        },
    ))
}

fn parse_conjunction(input: &mut &str) -> PResult<(String, Module)> {
    let (name, dests) =
        separated_pair(preceded('&', alpha1), " -> ", destinations).parse_next(input)?;
    Ok((
        name.to_string(),
        Module {
            kind: ModuleKind::Conjunction(HashMap::default()),
            destinations: dests,
        },
    ))
}

pub fn parse_input(input: &str) -> HashMap<String, Module> {
    input
        .lines()
        .map(|mut l| {
            alt((parse_broadcaster, parse_conjunction, parse_flip_flop))
                .parse_next(&mut l)
                .unwrap()
        })
        .collect()
}

fn output(m: &mut Module, pulse: Pulse) -> Vec<(String, Pulse)> {
    match m.kind {
        ModuleKind::Broadcaster => m.destinations.into_iter().map(|d| (d, pulse)).collect(),
        ModuleKind::FlipFlop(status) => match (pulse, status) {
            (Pulse::High, _) => vec![],
            (Pulse::Low, Status::On) => {
                m.kind = ModuleKind::FlipFlop(Status::Off);
                m.destinations
                    .clone()
                    .into_iter()
                    .map(|d| (d, Pulse::Low))
                    .collect()
            }
            (Pulse::Low, Status::Off) => {
                m.kind = ModuleKind::FlipFlop(Status::On);
                m.destinations
                    .clone()
                    .into_iter()
                    .map(|d| (d, Pulse::High))
                    .collect()
            }
        },
        ModuleKind::Conjunction(memory) => todo!(),
    }
}

pub fn part1(input: HashMap<String, Module>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    fn data() -> &'static str {
        indoc! {
            "broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a"
        }
    }

    #[test]
    fn test_destinations() {
        assert_eq!(
            destinations(&mut "a, b, c"),
            Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        );
        assert_eq!(destinations(&mut "a"), Ok(vec!["a".to_string()]))
    }

    #[test]
    fn test_parse_broadcaster() {
        let mut input = "broadcaster -> cx, zq, tv, rh";
        assert_eq!(
            parse_broadcaster(&mut input),
            Ok((
                "broadcaster".to_string(),
                Module {
                    kind: ModuleKind::Broadcaster,
                    destinations: vec![
                        "cx".to_string(),
                        "zq".to_string(),
                        "tv".to_string(),
                        "rh".to_string()
                    ]
                }
            ))
        )
    }

    #[test]
    fn test_parse_flipflop() {
        let mut input = "%a -> b";
        assert_eq!(
            parse_flip_flop(&mut input),
            Ok((
                "a".to_string(),
                Module {
                    kind: ModuleKind::FlipFlop(Status::Off),
                    destinations: vec!["b".to_string()]
                }
            ))
        )
    }

    #[test]
    fn test_parse_conjunction() {
        let mut input = "&inv -> a";
        assert_eq!(
            parse_conjunction(&mut input),
            Ok((
                "inv".to_string(),
                Module {
                    kind: ModuleKind::Conjunction(HashMap::default()),
                    destinations: vec!["a".to_string()]
                }
            ))
        )
    }

    #[test]
    fn test_parse_input() {
        let input = data();
        let res = parse_input(input);
        assert_eq!(
            res.get("c").unwrap(),
            &Module {
                kind: ModuleKind::FlipFlop(Status::Off),
                destinations: vec!["inv".to_string()]
            }
        )
    }
}
