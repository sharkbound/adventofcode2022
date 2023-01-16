use bytes::complete::tag;
use character::complete::{line_ending, multispace1};
use character::streaming::alpha1;
use nom::bytes;
use nom::bytes::complete::{is_a, take_while};
use nom::character;
use nom::multi::separated_list1;
use nom::sequence;
use nom::IResult;
use sequence::preceded;

/*
example:
https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2022/rust/day-05/src/lib.rs
*/


fn main() {
    let to_parse = r#"Items
    milk
    eggs
    crackers
Notes:
    This is a note
    This is another note
"#;

    match parse(to_parse) {
        Ok((_, (items, notes))) => {
            println!("PASS! \n\tItems: {:?}\n\tNotes: {:?}", items, notes);
        }
        Err(_) => {
            println!("FAIL!")
        }
    };
}

fn _space_prefixed_word(input: &str) -> IResult<&str, &str> {
    let (input, item) = preceded(multispace1, alpha1)(input)?;
    Ok((input, item))
}

fn _space_prefixed_and_separated_word(input: &str) -> IResult<&str, &str> {
    let (input, item) = preceded(multispace1, take_while(|x| x != '\n'))(input)?;
    Ok((input, item))
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, _) = tag("Items")(input)?;
    let (input, _) = line_ending(input)?;
    let (input, items) = separated_list1(line_ending, _space_prefixed_word)(input)?;
    let (input, _) = is_a("\n :\r")(input)?;
    let (input, _) = tag("Notes:")(input)?;
    let (input, notes) = separated_list1(line_ending, _space_prefixed_and_separated_word)(input)?;
    Ok((input, (items, notes)))
}

// fn parse(input: &str) -> IResult<&str, Vec<&str>> {
//     let (input, _) = bytes::complete::tag("Items")(input)?;
//     let (input, _) = character::complete::line_ending(input)?;
//     let (mut input, _) = bytes::streaming::is_a(" \t\n")(input)?;
//     let mut items = vec![];
//     let mut _trash = "";
//     loop {
//         (input, _trash) = bytes::complete::take_till(|x: char| x.is_alphanumeric())(input)?;
//         input = match not_whitespace(input) {
//             Ok((rest, item)) => {
//                 items.push(item);
//                 rest
//             }
//             Err(_) => break,
//         };
//     }
//     Ok((input, items))
// }
