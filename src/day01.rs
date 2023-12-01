pub fn parse_input(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()))
        .map(|mut cs| {
            let first = cs.next().unwrap();
            let last = cs.last().unwrap_or(first);
            format!("{}{}", first, last).parse().unwrap()
        })
        .collect()
}
pub fn parse_input_p2(input: &str) -> Vec<u16> {
    input
        .lines()
        .map(|l| {
            l.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|l| l.chars().filter(|c| c.is_ascii_digit()).collect())
        .map(|cs: Vec<char>| {
            let first = cs.first().unwrap();
            let last = cs.iter().last().unwrap_or(first);
            format!("{}{}", first, last).parse().unwrap()
        })
        .collect()
}

pub fn part1(input: Vec<u16>) -> u16 {
    input.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let input = indoc! {
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "};
        let vec = parse_input(input);
        assert_eq!(vec, vec![12, 38, 15, 77]);
    }
    #[test]
    fn test_part1() {
        let input = indoc! {
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "};
        let vec = parse_input(input);
        let result = part1(vec);
        assert_eq!(result, 142);
    }
    #[test]
    fn test_part2() {
        let input = indoc! {
        "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        "};
        let vec = parse_input_p2(input);
        let result = part1(vec);
        assert_eq!(result, 281);
    }
}
