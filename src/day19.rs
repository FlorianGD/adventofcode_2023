use crate::parsers::num;
use ranges::Ranges;
use std::collections::HashMap;
use std::ops::Bound;
use std::ops::RangeBounds;
use winnow::ascii::alpha1;
use winnow::combinator::{delimited, preceded, repeat, terminated};
use winnow::token::one_of;
use winnow::{PResult, Parser};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Comp {
    Lt,
    Gt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    name: char,
    comp: Comp,
    val: usize,
    dest: String,
}

impl Rule {
    fn new(name: char, comp: Comp, val: usize, dest: String) -> Self {
        Rule {
            name,
            comp,
            val,
            dest,
        }
    }
    fn validate(&self, part: &Part) -> Option<String> {
        let val = match self.name {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => unreachable!(),
        };
        if match self.comp {
            Comp::Lt => val < self.val,
            Comp::Gt => val > self.val,
        } {
            Some(self.dest.to_string())
        } else {
            None
        }
    }
}

///a<2006:qkq
fn rule(input: &mut &str) -> PResult<Rule> {
    let (name, comp) = (one_of(['x', 'm', 'a', 's']), one_of(['<', '>'])).parse_next(input)?;
    let comp = match comp {
        '>' => Comp::Gt,
        '<' => Comp::Lt,
        _ => unreachable!(),
    };
    let val = num.parse_next(input)?;
    let dest = preceded(':', alpha1).parse_next(input)?;
    Ok(Rule::new(name, comp, val, dest.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workflow {
    rules: Vec<Rule>,
    final_dest: String,
}

impl Workflow {
    fn new(rules: Vec<Rule>, final_dest: String) -> Self {
        Workflow { rules, final_dest }
    }

    fn process(&self, part: &Part) -> String {
        for rule in &self.rules {
            if let Some(dest) = rule.validate(part) {
                return dest;
            }
        }
        self.final_dest.clone()
    }

    fn process_p2(&self, part: &PartRange) -> Vec<(String, PartRange)> {
        let mut part = part.clone();
        let mut res = vec![];
        for rule in &self.rules {
            let (accepted, rejected) = part.apply_rule(rule);
            if !accepted.is_empty() {
                res.push((rule.dest.to_string(), accepted));
            }
            if rejected.is_empty() {
                return res;
            }
            part = rejected;
        }
        if !part.is_empty() {
            res.push((self.final_dest.clone(), part))
        }
        res
    }
}

fn workflow(input: &mut &str) -> PResult<(String, Workflow)> {
    let name = alpha1.parse_next(input)?;
    let rules = preceded('{', repeat(1.., terminated(rule, ','))).parse_next(input)?;
    let final_dest = terminated(alpha1, '}').parse_next(input)?;
    Ok((
        name.to_string(),
        Workflow::new(rules, final_dest.to_string()),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Part { x, m, a, s }
    }
}

fn part(input: &mut &str) -> PResult<Part> {
    let (x, m, a, s) = delimited(
        '{',
        (
            preceded("x=", num),
            preceded(",m=", num),
            preceded(",a=", num),
            preceded(",s=", num),
        ),
        '}',
    )
    .parse_next(input)?;
    Ok(Part::new(x, m, a, s))
}

pub fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (in1, in2) = input.split_once("\n\n").unwrap();
    let workflows = in1
        .lines()
        .map(|mut l| workflow.parse_next(&mut l).unwrap())
        .collect();
    let parts = in2
        .lines()
        .map(|mut l| part.parse_next(&mut l).unwrap())
        .collect();
    (workflows, parts)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status {
    A,
    R,
}

fn process_all(workflows: &HashMap<String, Workflow>, part: &Part) -> Status {
    let mut current_workflow = workflows.get("in").unwrap();
    loop {
        match current_workflow.process(part).as_str() {
            "A" => return Status::A,
            "R" => return Status::R,
            s => current_workflow = workflows.get(s).unwrap(),
        }
    }
}

pub fn part1((workflows, parts): (HashMap<String, Workflow>, Vec<Part>)) -> usize {
    let mut result = 0;
    for part in parts {
        match process_all(&workflows, &part) {
            Status::A => {
                result += part.x + part.m + part.a + part.s;
            }
            Status::R => (),
        }
    }
    result
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartRange {
    x: Ranges<isize>,
    m: Ranges<isize>,
    a: Ranges<isize>,
    s: Ranges<isize>,
}

impl Default for PartRange {
    fn default() -> Self {
        let default = Ranges::from(1..4001);
        PartRange::new(default.clone(), default.clone(), default.clone(), default)
    }
}

fn ranges_width(ranges: &Ranges<isize>) -> isize {
    ranges
        .as_slice()
        .iter()
        .map(|r| match (r.start_bound(), r.end_bound()) {
            (Bound::Included(s), Bound::Excluded(e)) => e - s,
            // (Bound::Excluded(s), Bound::Excluded(e)) => e - s - 1,
            // (Bound::Excluded(s), Bound::Included(e)) => e - s + 1,
            // (Bound::Included(s), Bound::Included(e)) => e - s + 2,
            _ => unreachable!(),
        })
        .sum()
}

impl PartRange {
    fn new(x: Ranges<isize>, m: Ranges<isize>, a: Ranges<isize>, s: Ranges<isize>) -> Self {
        PartRange { x, m, a, s }
    }
    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.m.is_empty() || self.a.is_empty() || self.s.is_empty()
    }
    fn set_range_as_name(&self, name: char, range: Ranges<isize>) -> Self {
        let mut part = self.clone();
        match name {
            'x' => part.x = range,
            'm' => part.m = range,
            'a' => part.a = range,
            's' => part.s = range,
            _ => unreachable!(),
        };
        part
    }

    fn possibilities(&self) -> isize {
        ranges_width(&self.x)
            * ranges_width(&self.m)
            * ranges_width(&self.a)
            * ranges_width(&self.s)
    }

    /// PartRange that moves to dest, PartRange rejected
    fn apply_rule(&self, rule: &Rule) -> (Self, Self) {
        let val = rule.val as isize;
        let (accepted, rejected) = match rule.comp {
            Comp::Lt => (Ranges::from(1..val), Ranges::from(val..4001)),
            Comp::Gt => (Ranges::from(val + 1..4001), Ranges::from(1..val + 1)),
        };
        match rule.name {
            'x' => {
                let accepted_range = self.x.clone() & accepted;
                let rejected_range = self.x.clone() & rejected;
                (
                    self.set_range_as_name('x', accepted_range),
                    self.set_range_as_name('x', rejected_range),
                )
            }
            'm' => {
                let accepted_range = self.m.clone() & accepted;
                let rejected_range = self.m.clone() & rejected;
                (
                    self.set_range_as_name('m', accepted_range),
                    self.set_range_as_name('m', rejected_range),
                )
            }
            'a' => {
                let accepted_range = self.a.clone() & accepted;
                let rejected_range = self.a.clone() & rejected;
                (
                    self.set_range_as_name('a', accepted_range),
                    self.set_range_as_name('a', rejected_range),
                )
            }
            's' => {
                let accepted_range = self.s.clone() & accepted;
                let rejected_range = self.s.clone() & rejected;
                (
                    self.set_range_as_name('s', accepted_range),
                    self.set_range_as_name('s', rejected_range),
                )
            }
            _ => unreachable!(),
        }
    }
}

fn process_p2_all(workflows: &HashMap<String, Workflow>) -> isize {
    let start_part = PartRange::default();
    let mut current_workflows = vec![("in".to_string(), start_part)];

    let mut result = 0;
    while let Some((workflow_name, part)) = current_workflows.pop() {
        if workflow_name.as_str() == "A" {
            result += part.possibilities();
        } else if workflow_name.as_str() == "R" {
            continue;
        } else {
            let workflow = workflows.get(&workflow_name).expect("unknown workflow");
            for (s, p) in workflow.process_p2(&part) {
                current_workflows.push((s, p));
            }
        }
    }
    result
}

pub fn part2((workflows, _parts): (HashMap<String, Workflow>, Vec<Part>)) -> isize {
    process_p2_all(&workflows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rule() {
        assert_eq!(
            rule(&mut "x<2006:qkq"),
            Ok(Rule::new('x', Comp::Lt, 2006, "qkq".to_string()))
        );
    }

    #[test]
    fn test_workflow() {
        let expected = (
            "px".to_string(),
            Workflow::new(
                vec![
                    Rule::new('a', Comp::Lt, 2006, "qkq".to_string()),
                    Rule::new('m', Comp::Gt, 2090, "A".to_string()),
                ],
                "rfg".to_string(),
            ),
        );
        assert_eq!(
            workflow.parse_next(&mut "px{a<2006:qkq,m>2090:A,rfg}"),
            Ok(expected)
        );
    }

    #[test]
    fn test_validate() {
        let part = Part::new(787, 2655, 1222, 2876);
        let rule = Rule::new('a', Comp::Lt, 20006, "qkq".to_string());
        assert_eq!(rule.validate(&part), Some("qkq".to_string()));
    }

    #[test]
    fn test_part() {
        let mut input = "{x=787,m=2655,a=1222,s=2876}";
        let expected = Part::new(787, 2655, 1222, 2876);
        assert_eq!(part(&mut input), Ok(expected));
    }

    fn data() -> &'static str {
        indoc! {
        "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"
        }
    }

    #[test]
    fn test_workflow_process() {
        let (_, workflow) = workflow
            .parse_next(&mut "px{a<2006:qkq,m>2090:A,rfg}")
            .unwrap();
        let part = Part::new(787, 2655, 1222, 2876);
        assert_eq!(workflow.process(&part), "qkq".to_string());
    }

    #[test]
    fn test_process_all() {
        let (workflows, parts) = parse_input(data());
        let status = process_all(&workflows, parts.first().unwrap());
        assert_eq!(status, Status::A);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(data())), 19114);
    }

    #[test]
    fn test_apply_rule() {
        let r = rule.parse_next(&mut "a<2006:qkq").unwrap();
        let part_range = PartRange::default();
        let (a, r) = part_range.apply_rule(&r);
        let expected_accepted = PartRange::new(
            Ranges::from(1..4001),
            Ranges::from(1..4001),
            Ranges::from(1..2006),
            Ranges::from(1..4001),
        );
        let expected_rejected = PartRange::new(
            Ranges::from(1..4001),
            Ranges::from(1..4001),
            Ranges::from(2006..4001),
            Ranges::from(1..4001),
        );
        assert_eq!(expected_accepted, a);
        assert_eq!(expected_rejected, r);
    }

    #[test]
    fn test_ranges_possibilities() {
        let p = PartRange::default();
        assert_eq!(p.possibilities(), 4000isize.pow(4));
        let p2 = p.set_range_as_name('a', Ranges::from(vec![1..101, 1000..2000]));
        assert_eq!(p2.possibilities(), 4000isize.pow(3) * 1100);
    }

    #[test]
    fn test_process_p2() {
        let (_, workflow) = workflow
            .parse_next(&mut "px{a<2006:qkq,m>2090:A,rfg}")
            .unwrap();
        let part_range = PartRange::default();
        let result = workflow.process_p2(&part_range);
        assert_eq!(
            result,
            vec![
                (
                    "qkq".to_string(),
                    PartRange::default().set_range_as_name('a', Ranges::from(1..2006))
                ),
                (
                    "A".to_string(),
                    PartRange::default()
                        .set_range_as_name('a', Ranges::from(2006..4001))
                        .set_range_as_name('m', Ranges::from(2091..4001))
                ),
                (
                    "rfg".to_string(),
                    PartRange::default()
                        .set_range_as_name('a', Ranges::from(2006..4001))
                        .set_range_as_name('m', Ranges::from(1..2091))
                )
            ]
        );
    }

    #[test]
    fn test_part2() {
        let input = data();
        assert_eq!(part2(parse_input(input)), 167409079868000);
    }
}
