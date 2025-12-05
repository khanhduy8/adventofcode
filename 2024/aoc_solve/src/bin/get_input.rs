use clap::{Arg, Command};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Define command-line arguments
    let matches = Command::new("Get input from aoc")
        .arg(
            Arg::new("cookie")
                .long("cookie")
                .value_name("COOKIE")
                .help("Session cookie for Advent of Code")
                .required(true),
        )
        .arg(
            Arg::new("day")
                .long("day")
                .value_name("DAY")
                .help("Day number of Advent of Code")
                .required(true),
        )
        .arg(
            Arg::new("star")
                .long("star")
                .value_name("STAR")
                .help("Star number of Day")
                .required(true),
        )
        .get_matches();

    // Get the values of the arguments
    let cookie = matches
        .get_one::<String>("cookie")
        .expect("Cookie is required");
    let day: &u32 = matches.get_one::<&u32>("day").expect("Day is required");

    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let output_file = format!("src/bin/input/day{}.txt", day);

    println!("Fetching input of day {} - {}", day, url);

    // Fetch the content
    let content = fetch_content(&url, cookie).await?;

    // Save the content
    save_to_file(&output_file, &content)?;

    println!("Saved input for Day {} to {}", day, output_file);

    Ok(())
}

async fn fetch_content(url: &str, cookie: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("Cookie", format!("session={}", cookie))
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

fn save_to_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
