use reqwest::cookie::Jar;
use reqwest::Client;
use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

pub async fn get_aoc_input(day: u32) -> Result<String, Box<dyn std::error::Error>> {
    // path originates from the workspace directory here
    let session =
        std::io::read_to_string(File::open(r"D:\git\adventofcode2022\secrets\session.txt")?)?;
    let uri = &format!("https://www.adventofcode.com/2022/day/{}/input", day);

    match create_client(uri, &session).get(uri).send().await {
        Ok(response) => match response.text().await {
            Ok(text) => Ok(text),
            Err(e) => Err(Box::new(e)),
        },
        Err(e) => Err(Box::new(e)),
    }
}

fn create_client(uri: &str, session: &str) -> Client {
    Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new( {
            let jar = Jar::default();
            jar.add_cookie_str(
                &format!(
                    "session={}; User-Agent=advent-of-code-input-fetcher; Domain=adventofcode.com",
                    session
                ),
                &uri.parse::<reqwest::Url>().unwrap(),
            );
            jar
        }))
        .build()
        .unwrap()
}
