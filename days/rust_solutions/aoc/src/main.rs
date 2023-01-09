use aoc::get_aoc_input;

#[tokio::main]
async fn main() {
    let day = 12;
    let input = get_aoc_input(day).await;

    match input {
        Ok(value) => {
            println!("Text: {}", value);
        }
        Err(err) => {
            println!("Cannot download AOC input for day: {}\nErr: \n\t{}", day, err);
            std::process::exit(1);
        }
    }

    let cmd_args = std::env::args().collect::<Vec<_>>();
    // let input_file = match  day{
    //
    // };
}
