use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Apod {
    title: String,
    date: String,
    explanation: String,
    url: String,
}

fn main() {
    let api_key = "Owwu0ar0RtvX84p0bjHW1r7nlzKvozGfdCpqx8yR";
    let target_date = "2023-11-14"; // e.g. the day of Webb Telescope's first image
    let url = format!(
        "https://api.nasa.gov/planetary/apod?api_key={}&date={}",
        api_key, target_date
    );
    println!(
        "Fetching Astronomy Picture of the Day for {}...\n",
        target_date
    );
    let response = reqwest::blocking::get(&url).expect("Failed to reach NASA's API");
    let apod: Apod = response.json().expect("Failed to parse JSON response");
    println!("Title: {}", apod.title);
    println!("Date: {}", apod.date);
    println!("Image URL: {}", apod.url);
    println!("\nExplanation:\n{}", apod.explanation);
}
