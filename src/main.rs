use std::fs;

mod day1;

fn main() {
    day1::day(get_input(1));
}

fn get_input(day: usize) -> String {
    let file_path = format!("input/day{}.txt", day);
    fs::read_to_string(file_path)
        .expect("Something went wrong!")
        .trim()
        .to_owned()
}
