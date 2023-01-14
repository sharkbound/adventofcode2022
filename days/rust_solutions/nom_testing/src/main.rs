use nom::{character, IResult};
use nom::bytes::complete::{tag};
use nom::multi::separated_list1;


fn main() {
    let ints = "1 2 10 86 1996 1857 1000";
    dbg!(parse_ints(ints).ok());
}


fn parse_ints(input: &str) -> IResult<&str, Vec<i32>> {
    fn parse(input: &str) -> IResult<&str, i32> {
        character::complete::i32(input)
    }
    // separated_list1(tag(" "), |x| character::complete::i32(x))(input)
    separated_list1(tag(" "), parse)(input)
}