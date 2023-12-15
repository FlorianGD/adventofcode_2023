pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .split(',')
        .map(|x| x.to_string().replace('\n', "").into_bytes())
        .collect()
}

fn hash(chars: &[u8]) -> usize {
    chars
        .iter()
        .fold(0usize, |acc, c| ((acc + *c as usize) * 17) % 256)
}

pub fn part1(input: Vec<Vec<u8>>) -> usize {
    input.iter().map(|p| hash(p)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hash() {
        let chars = "HASH".as_bytes();
        assert_eq!(hash(chars), 52);
    }

    #[test]
    fn test_parse_input() {
        let input = "abc,def";
        let expected = vec![vec![b'a', b'b', b'c'], vec![b'd', b'e', b'f']];
        assert_eq!(parse_input(input), expected);
    }

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(parse_input(input)), 1320);
    }
}
