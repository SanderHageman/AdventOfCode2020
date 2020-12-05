type TParsed = Vec<Seat>;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    input
        .iter()
        .max_by(|x, y| x.id().cmp(&y.id()))
        .unwrap()
        .id()
}

fn part_2(input: &TParsed) -> usize {
    let mut seated = vec![vec![false; 8]; 128];
    for seat in input {
        assert!(!seated[seat.r][seat.c]);
        seated[seat.r][seat.c] = true;
    }
    let seated = seated;

    // we start 'counting' empty seats once
    // we've encoutered occupied ones
    let mut can_count = false;
    let mut result: Option<Seat> = None;

    'outer: for (i, row) in seated.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col {
                can_count = true;
            } else if can_count {
                result = Some(Seat { r: i, c: j });
                break 'outer;
            }
        }
    }

    result.expect("Unable to find part 2 seat").id()
}

#[derive(Debug)]
struct Seat {
    r: usize,
    c: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.r * 8 + self.c
    }

    fn find_position(input: &str) -> (usize, usize) {
        let mut range_row = (0, 127);
        let mut range_col = (0, 7);

        for c in input.chars() {
            let rr = range_row.1 - range_row.0 + 1;
            let rc = range_col.1 - range_col.0 + 1;

            match c {
                'F' => range_row.1 -= rr / 2,
                'B' => range_row.0 += rr / 2,
                'L' => range_col.1 -= rc / 2,
                'R' => range_col.0 += rc / 2,
                _ => panic!("Uncovered input {}", c),
            }
        }

        assert_eq!(range_row.0, range_row.1);
        assert_eq!(range_col.0, range_col.1);
        (range_row.0, range_col.0)
    }
}

impl From<&str> for Seat {
    fn from(val: &str) -> Self {
        let t = Seat::find_position(val);
        Seat { r: t.0, c: t.1 }
    }
}

fn parse(input: &str) -> TParsed {
    input.lines().map(|line| Seat::from(line)).collect()
}

#[test]
fn test_example_1() {
    // No example to validate whole part
    assert_eq!(Seat::from(test::EXAMPLE_INPUT_1).id(), 357);
    assert_eq!(Seat::from(test::EXAMPLE_INPUT_2).id(), 567);
    assert_eq!(Seat::from(test::EXAMPLE_INPUT_3).id(), 119);
    assert_eq!(Seat::from(test::EXAMPLE_INPUT_4).id(), 820);
}

#[cfg(test)]
mod test {
    pub const EXAMPLE_INPUT_1: &str = "FBFBBFFRLR";
    pub const EXAMPLE_INPUT_2: &str = "BFFFBBFRRR";
    pub const EXAMPLE_INPUT_3: &str = "FFFBBBFRRR";
    pub const EXAMPLE_INPUT_4: &str = "BBFFBBFRLL";
}
