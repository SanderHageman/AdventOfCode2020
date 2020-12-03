#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod util;

fn main() {
    day1::day(util::get_input(1, 2020));
    day2::day(util::get_input(2, 2020));
    day3::day(util::get_input(3, 2020));
}
