use winnow::{ascii::digit1, PResult, Parser};

pub fn num<T: std::str::FromStr>(input: &mut &str) -> PResult<T> {
    digit1.parse_to().parse_next(input)
}
