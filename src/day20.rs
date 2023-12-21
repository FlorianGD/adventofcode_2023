use std::collections::{HashMap, VecDeque};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

type Memory = HashMap<String, Pulse>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    Broadcaster,
    FlipFlop(Status),
    Conjunction(Memory),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    name: String,
    destinations: Vec<String>,
    kind: ModuleKind,
}

fn destinations(input: &mut &str) -> PResult<Vec<String>> {
    let first = alpha1.parse_next(input)?;
    let rest: Vec<&str> = repeat(0.., preceded(", ", alpha1)).parse_next(input)?;
    let mut dests = vec![first];
    dests.extend(rest);
    Ok(dests.into_iter().map(|l| l.to_string()).collect())
}

fn parse_broadcaster(input: &mut &str) -> PResult<(String, Module)> {
    let dests = preceded("broadcaster -> ", destinations).parse_next(input)?;
    Ok((
        "broadcaster".to_string(),
        Module {
            name: "broadcaster".to_string(),
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
            name: name.to_string(),
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
            name: name.to_string(),
            kind: ModuleKind::Conjunction(HashMap::default()),
            destinations: dests,
        },
    ))
}

pub fn parse_input(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|mut l| {
            alt((parse_broadcaster, parse_conjunction, parse_flip_flop))
                .parse_next(&mut l)
                .unwrap()
        })
        .collect();
    init_memory(&mut modules);
    modules
}

fn init_memory(modules: &mut HashMap<String, Module>) {
    let conj = modules
        .iter()
        .filter(|(_, m)| matches!(&m.kind, ModuleKind::Conjunction(x) if x.is_empty()))
        .map(|(n, _)| n.clone())
        .collect::<Vec<String>>();
    for n in conj {
        let inputs = modules
            .iter()
            .filter(|(_, m)| m.destinations.contains(&n))
            .map(|(n, _)| (n.clone(), Pulse::Low))
            .collect::<HashMap<String, Pulse>>();
        modules
            .entry(n)
            .and_modify(|m| m.kind = ModuleKind::Conjunction(inputs));
    }
}

fn send(m: Module, pulse: Pulse, source: &String) -> (Module, Vec<(String, Pulse, String)>) {
    match m.kind {
        ModuleKind::Broadcaster => (
            m.clone(),
            m.destinations
                .clone()
                .into_iter()
                .map(|d| (d, pulse, m.name.clone()))
                .collect(),
        ),
        ModuleKind::FlipFlop(ref status) => match (pulse, status) {
            (Pulse::High, _) => (m.clone(), vec![]),
            (Pulse::Low, Status::On) => {
                let mut module = m.clone();
                module.kind = ModuleKind::FlipFlop(Status::Off);
                (
                    module,
                    m.destinations
                        .clone()
                        .into_iter()
                        .map(|d| (d, Pulse::Low, m.name.clone()))
                        .collect(),
                )
            }
            (Pulse::Low, Status::Off) => {
                let mut module = m.clone();
                module.kind = ModuleKind::FlipFlop(Status::On);
                (
                    module,
                    m.destinations
                        .clone()
                        .into_iter()
                        .map(|d| (d, Pulse::High, m.name.clone()))
                        .collect(),
                )
            }
        },
        ModuleKind::Conjunction(ref memory) => {
            let mut module = m.clone();
            let mut new_memory = memory.clone();
            *new_memory.get_mut(source).unwrap() = pulse;
            module.kind = ModuleKind::Conjunction(new_memory.clone());
            if new_memory.values().all(|p| p == &Pulse::High) {
                (
                    module,
                    m.destinations
                        .clone()
                        .into_iter()
                        .map(|d| (d, Pulse::Low, m.name.clone()))
                        .collect(),
                )
            } else {
                (
                    module,
                    m.destinations
                        .clone()
                        .into_iter()
                        .map(|d| (d, Pulse::High, m.name.clone()))
                        .collect(),
                )
            }
        }
    }
}

fn press_button(modules: &HashMap<String, Module>) -> (HashMap<String, Module>, usize, usize) {
    let mut modules = modules.clone();
    let mut num_high = 0;
    let mut num_low = 1;
    let mut queue = VecDeque::from([("broadcaster".to_string(), Pulse::Low, "button".to_string())]);

    while let Some((target, pulse, source)) = queue.pop_front() {
        // println!("{source} -{pulse:?}-> {}", target);
        if let Some(module) = modules.get(&target) {
            let (new_module, new) = send(module.clone(), pulse, &source);

            *modules.get_mut(&target).unwrap() = new_module;

            num_high += new.iter().filter(|(_, p, _)| p == &Pulse::High).count();
            num_low += new.iter().filter(|(_, p, _)| p == &Pulse::Low).count();

            queue.extend(new.into_iter());
        }
    }
    (modules, num_high, num_low)
}

pub fn part1(input: HashMap<String, Module>) -> usize {
    let mut modules = input;
    let mut high = 0;
    let mut low = 0;
    for _ in 0..1000 {
        let (new_modules, new_high, new_low) = press_button(&modules);
        high += new_high;
        low += new_low;
        modules = new_modules;
    }
    dbg!(high) * dbg!(low)
}

fn press_button_p2(modules: &HashMap<String, Module>) -> (HashMap<String, Module>, Option<()>) {
    let mut modules = modules.clone();
    let mut queue = VecDeque::from([("broadcaster".to_string(), Pulse::Low, "button".to_string())]);

    while let Some((target, pulse, source)) = queue.pop_front() {
        // println!("{source} -{pulse:?}-> {}", target);
        if let Some(module) = modules.get(&target) {
            let (new_module, new) = send(module.clone(), pulse, &source);

            *modules.get_mut(&target).unwrap() = new_module;

            let mut filt = new
                .iter()
                .filter(|(target, pulse, _)| target == &"rx".to_string() && pulse == &Pulse::Low)
                .peekable();
            if filt.peek().is_none() {
                queue.extend(new.into_iter());
            } else {
                return (modules, Some(()));
            }
        }
    }
    (modules, None)
}

pub fn part2(input: HashMap<String, Module>) -> usize {
    let mut modules = input;
    let mut res = 0;
    loop {
        let (new_modules, maybe_res) = press_button_p2(&modules);
        res += 1;
        if maybe_res.is_some() {
            return res;
        }
        modules = new_modules;
    }
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
                    name: "broadcaster".to_string(),
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
                    name: "a".to_string(),
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
                    name: "inv".to_string(),
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
                name: "c".to_string(),
                kind: ModuleKind::FlipFlop(Status::Off),
                destinations: vec!["inv".to_string()]
            }
        );
        let expected_inputs = HashMap::from([("c".to_string(), Pulse::Low)]);
        // check the initialization of conjunction modules
        assert_eq!(
            res.get("inv").unwrap(),
            &Module {
                name: "inv".to_string(),
                kind: ModuleKind::Conjunction(expected_inputs),
                destinations: vec!["a".to_string()]
            }
        );
    }

    #[test]
    fn test_send_broadcaster() {
        let modules = parse_input(data());
        let first_module = modules.get("broadcaster").unwrap();
        let (module, dests) = send(first_module.clone(), Pulse::Low, &"button".to_string());
        assert_eq!(
            dests,
            vec![
                ("a".to_string(), Pulse::Low, "broadcaster".to_string()),
                ("b".to_string(), Pulse::Low, "broadcaster".to_string()),
                ("c".to_string(), Pulse::Low, "broadcaster".to_string())
            ]
        );
        assert_eq!(
            module,
            Module {
                name: "broadcaster".to_string(),
                kind: ModuleKind::Broadcaster,
                destinations: vec!["a".to_string(), "b".to_string(), "c".to_string()]
            }
        );
    }
    #[test]
    fn test_send_flipflop() {
        let modules = parse_input(data());
        let first_module = modules.get("a").unwrap();
        let (module, dests) = send(first_module.clone(), Pulse::Low, &"broadcaster".to_string());
        assert_eq!(dests, vec![("b".to_string(), Pulse::High, "a".to_string())]);
        assert_eq!(
            module,
            Module {
                name: "a".to_string(),
                kind: ModuleKind::FlipFlop(Status::On),
                destinations: vec!["b".to_string()]
            }
        );
    }
    #[test]
    fn test_send_conjunction() {
        let modules = parse_input(data());
        let first_module = modules.get("inv").unwrap();
        let (module, dests) = send(first_module.clone(), Pulse::High, &"c".to_string());
        assert_eq!(
            dests,
            vec![("a".to_string(), Pulse::Low, "inv".to_string())]
        );
        let expected_inputs = HashMap::from([("c".to_string(), Pulse::High)]);
        assert_eq!(
            module,
            Module {
                name: "inv".to_string(),
                kind: ModuleKind::Conjunction(expected_inputs),
                destinations: vec!["a".to_string()]
            }
        );
        let (module, dests) = send(module.clone(), Pulse::High, &"c".to_string());
        assert_eq!(
            dests,
            vec![("a".to_string(), Pulse::Low, "inv".to_string())]
        );
        let expected_inputs = HashMap::from([("c".to_string(), Pulse::High)]);
        assert_eq!(
            module,
            Module {
                name: "inv".to_string(),
                kind: ModuleKind::Conjunction(expected_inputs),
                destinations: vec!["a".to_string()]
            }
        );
        let (module, dests) = send(module.clone(), Pulse::Low, &"c".to_string());
        assert_eq!(
            dests,
            vec![("a".to_string(), Pulse::High, "inv".to_string())]
        );
        let expected_inputs = HashMap::from([("c".to_string(), Pulse::Low)]);
        assert_eq!(
            module,
            Module {
                name: "inv".to_string(),
                kind: ModuleKind::Conjunction(expected_inputs),
                destinations: vec!["a".to_string()]
            }
        );
    }
    #[test]
    fn test_press_button() {
        let modules = parse_input(data());
        let result = press_button(&modules);
        assert_eq!(result, (modules.clone(), 4, 8));
    }
    #[test]
    fn test_part1() {
        let modules = parse_input(data());
        let result = part1(modules);
        assert_eq!(result, 32000000);
    }

    fn data2() -> &'static str {
        indoc! {
        "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output
        "
        }
    }

    #[test]
    fn test_part1_2nd_example() {
        assert_eq!(part1(parse_input(data2())), 11687500);
    }
}
