use nom::{IResult};
use nom::bytes::complete::{is_not, tag, take_till};
use nom::bytes::streaming::{is_a};
use nom::character::complete::{newline};
use nom::character::{is_alphabetic, is_alphanumeric};


static TO_PARSE: &'static str = "\
Items
    milk
    eggs
    crackers
";


fn main() {
    let ints = "1 -2 10 86 1996 1857 1000";
    println!("{:?}", match parse(TO_PARSE) {
        Ok(value) => {
            println!("{:?}", value);
        }
        Err(e) => {
            match e {
                nom::Err::Incomplete(v) => { println!("{:?}", v) }
                nom::Err::Error(e) => { println!("code: {:?}", e.code) }
                nom::Err::Failure(f) => { println!("{:?}", f.code) }
            }
        }
    });
}

fn not_whitespace(input: &str) -> IResult<&str, &str> {
    is_not(" \r\n\t")(input)
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = tag("Items")(input)?;
    let (input, _) = newline(input)?;
    let (mut input, _) = is_a(" \t\n")(input)?;
    let mut items = vec![];
    let mut _trash = "";
    loop {
        (input, _trash) = take_till(|x: char| x.is_alphanumeric())(input)?;
        input = match not_whitespace(input) {
            Ok((rest, item)) => {
                items.push(item);
                rest
            }
            Err(_) => return Ok((input, items)),
        };
    }
    Ok(("", items))
}