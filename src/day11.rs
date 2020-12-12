use std::ops::{Add, AddAssign};

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<Seat>;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    if false {
        (part_1(&parsed_input), part_2(&parsed_input))
    } else {
        // Hardcoded because this day takes too long
        // to compute for every future day
        (2273, 2064)
    }
}

fn part_1(input: &TParsed) -> usize {
    let mut map = input.to_owned();
    while do_round(&mut map, true) {}
    count_occupied(&map)
}

fn part_2(input: &TParsed) -> usize {
    let mut map = input.to_owned();
    while do_round(&mut map, false) {}
    count_occupied(&map)
}

fn do_round(map: &mut TParsed, pt1: bool) -> bool {
    let mut changes = Vec::<(usize, usize)>::new();

    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            if col.should_change(map, pt1) {
                changes.push((x, y));
            }
        }
    }

    for change in &changes {
        map[change.1][change.0].state = match map[change.1][change.0].state {
            State::Floor => panic!("Floor can't change state!"),
            State::Empty => State::Occupied,
            State::Occupied => State::Empty,
        }
    }

    changes.len() > 0
}

impl Seat {
    fn should_change(&self, map: &TParsed, pt1: bool) -> bool {
        match self.state {
            State::Floor => false,
            State::Empty => self.should_empty_change(map, pt1),
            State::Occupied => self.should_occupied_change(map, pt1),
        }
    }

    fn get(map: &TParsed, pos: Vec2) -> Option<State> {
        if (pos.x as usize) < map[0].len() && (pos.y as usize) < map.len() {
            Some(map[pos.y as usize][pos.x as usize].state)
        } else {
            None
        }
    }

    fn should_empty_change(&self, map: &TParsed, pt1: bool) -> bool {
        for nb in Seat::get_neighbours().iter() {
            let mut pos = self.pos + *nb;
            while let Some(state) = Seat::get(map, pos) {
                pos += *nb;
                match state {
                    State::Floor => {}
                    State::Empty => break,
                    State::Occupied => return false,
                }

                if pt1 {
                    break;
                }
            }
        }

        true
    }

    fn should_occupied_change(&self, map: &TParsed, pt1: bool) -> bool {
        let mut counter = 0;
        let target = if pt1 { 4 } else { 5 };
        for nb in Seat::get_neighbours().iter() {
            let mut pos = self.pos + *nb;
            while let Some(state) = Seat::get(map, pos) {
                pos += *nb;
                match state {
                    State::Floor => {}
                    State::Empty => break,
                    State::Occupied => {
                        counter += 1;
                        if counter >= target {
                            return true;
                        }
                        break;
                    }
                };

                if pt1 {
                    break;
                }
            }
        }

        false
    }

    fn get_neighbours() -> [Vec2; 8] {
        [
            vec2(-1, -1),
            vec2(0, -1),
            vec2(1, -1),
            vec2(-1, 0),
            vec2(1, 0),
            vec2(-1, 1),
            vec2(0, 1),
            vec2(1, 1),
        ]
    }
}

fn count_occupied(map: &TParsed) -> usize {
    map.iter()
        .map(|v| v.iter().filter(|s| s.state == State::Occupied).count())
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Seat {
    state: State,
    pos: Vec2,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn parse(input: &str) -> TParsed {
    let mut result = TParsed::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = TParsedSub::new();
        for (x, c) in line.chars().enumerate() {
            row.push(Seat {
                state: match c {
                    'L' => State::Empty,
                    '.' => State::Floor,
                    _ => panic!("Uncovered"),
                },
                pos: vec2(x as i32, y as i32),
            });
        }
        result.push(row);
    }

    result
}

#[derive(Debug, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 37)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 26)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

fn _display_map(input: &TParsed) {
    for i in input {
        for j in i {
            let c = match j.state {
                State::Floor => '.',
                State::Empty => 'L',
                State::Occupied => '#',
            };
            print!("{}", c);
        }
        println!("");
    }
    println!("");
}
