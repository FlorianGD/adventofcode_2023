use winnow::combinator::opt;
use winnow::{ascii::digit1, PResult, Parser};

pub fn num<T: std::str::FromStr>(input: &mut &str) -> PResult<T> {
    digit1.parse_to().parse_next(input)
}

//pub fn neg_num(input: &mut &str) -> PResult<isize> {
//    (opt('-'), digit1).parse_to().parse_next(input)
//}
