use core::panic;
use std::{fs, io::Result};

pub fn get_input(day: usize, year: usize) -> String {
    let file_path = format!("input/day{}", day);
    let file_content = fs::read_to_string(&file_path);

    {
        file_content.unwrap_or_else(|_| {
            println!("Fetching input for {}/{} online", day, year);
            if let Ok(online_result) = get_online_input(day, year) {
                fs::write(&file_path, &online_result).expect("Unable to write to cache");
                online_result
            } else {
                panic!("Unable to fetch input for {}/{}", day, year);
            }
        })
    }
    .trim()
    .to_owned()
}

fn get_online_input(day: usize, year: usize) -> Result<String> {
    let session_id = fs::read_to_string("input/session_id")
        .expect("Unable to read session id at input/session_id")
        .trim()
        .to_owned();

    let response = ureq::get(&format!(
        "https://adventofcode.com/{}/day/{}/input",
        year, day
    ))
    .set("Cookie", &format!("session={}", session_id))
    .call();

    response.into_string()
}
