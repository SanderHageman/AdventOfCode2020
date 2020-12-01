use std::{fs, io::Result};

pub fn get_input(day: usize, year: usize) -> String {
    let file_path = format!("input/day{}", day);

    let file_string = fs::read_to_string(file_path.clone());

    let result: String;

    if file_string.is_err() {
        let online = get_online_input(day, year);
        if let Ok(online_result) = online {
            println!("Fetching input for {}/{} online", day, year);
            fs::write(file_path.clone(), online_result.clone()).expect("Unable to write to cache");
            result = online_result;
        } else {
            result = String::new();
        }
    } else {
        result = file_string.unwrap();
    }

    result.trim().to_owned()
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
