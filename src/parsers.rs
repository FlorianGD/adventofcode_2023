use winnow::combinator::opt;
use winnow::{ascii::digit1, PResult, Parser};

pub fn num<T: std::str::FromStr>(input: &mut &str) -> PResult<T> {
    digit1.parse_to().parse_next(input)
}

pub fn neg_num<T: std::str::FromStr>(input: &mut &str) -> PResult<T> {
    (opt('-'), digit1).recognize().parse_to().parse_next(input)
}

#[cfg(test)]

mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_num() {
        assert_eq!(num::<u16>.parse_next(&mut "123"), Ok(123u16));
        assert_eq!(num::<i16>.parse_next(&mut "123"), Ok(123i16));
    }
    #[test]
    fn test_neg_num() {
        assert_eq!(neg_num::<isize>.parse_next(&mut "123"), Ok(123isize));
        assert_eq!(neg_num::<usize>.parse_next(&mut "123"), Ok(123usize));
        assert_eq!(neg_num::<isize>.parse_next(&mut "-123"), Ok(-123isize));
    }
}
