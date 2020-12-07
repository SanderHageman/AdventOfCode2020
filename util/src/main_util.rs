use std::{fs, io::Result};

#[macro_export]
macro_rules! main {
    ($(day $val:expr)+) => {
        paste!{
            $(mod [<day $val>];)+
            pub fn main() {
                let start = std::time::Instant::now();

                $(
                    let start_local = std::time::Instant::now();
                    let result = [<day $val>]::day(main_util::get_input($val, 2020));
                    let end_local = std::time::Instant::now();
                    let time = end_local.duration_since(start_local).as_millis();
                    println!("Result day {:02} ({:03}ms): \t{:?}\t{:?}", $val, time, result.0, result.1);
                )+

                let end = std::time::Instant::now();
                println!("Execution took {}ms", end.duration_since(start).as_millis());
            }
        }
    };
}

pub fn get_input(day: usize, year: usize) -> String {
    let file_path = format!("input/day{:02}", day);
    let file_content = fs::read_to_string(&file_path);

    {
        file_content.unwrap_or_else(|_| {
            println!("Fetching input for {}/{} online", day, year);
            let result = get_online_input(day, year).expect("Unable to fetch input");
            fs::write(&file_path, &result).expect("Unable to write to cache");
            result
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
