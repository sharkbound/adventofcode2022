use nom::{bytes, character, IResult};


static TO_PARSE: &'static str = "\
Items
    milk
    eggs
    crackers
";


fn main() {
    let ints = "1 -2 10 86 1996 1857 1000";
    match parse(TO_PARSE) {
        Ok(value) => {
            println!("PASS! {:?}", value);
        }
        Err(_) => {
            println!("FAIL!")
        }
    };
}


fn not_whitespace(input: &str) -> IResult<&str, &str> {
    bytes::complete::is_not(" \r\n\t")(input)
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = bytes::complete::tag("Items")(input)?;
    let (input, _) = character::complete::line_ending(input)?;
    let (mut input, _) = bytes::streaming::is_a(" \t\n")(input)?;
    let mut items = vec![];
    let mut _trash = "";
    loop {
        (input, _trash) = bytes::complete::take_till(|x: char| x.is_alphanumeric())(input)?;
        input = match not_whitespace(input) {
            Ok((rest, item)) => {
                items.push(item);
                rest
            }
            Err(_) => break,
        };
    }
    Ok((input, items))
}