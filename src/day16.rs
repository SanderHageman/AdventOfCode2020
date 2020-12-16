use std::collections::HashSet;

type TParsed = (Vec<DoubleRange>, Vec<u32>, Vec<Vec<u32>>);

pub fn day(input: String) -> (u32, u64) {
    let parsed_input = parse(&input);
    let (p1_result, p1_output) = part_1(&parsed_input);
    (
        p1_result,
        part_2(&(parsed_input.0, parsed_input.1, p1_output)),
    )
}

fn part_1((ranges, _, other_tickets): &TParsed) -> (u32, Vec<Vec<u32>>) {
    let mut acc = 0;

    let mut filtered = vec![];

    for other in other_tickets {
        let mut valid = true;
        for n in other {
            if !ranges.iter().any(|range| range.in_range(*n)) {
                acc += n;
                valid = false;
            }
        }
        if valid {
            filtered.push(other.clone());
        }
    }

    (acc, filtered)
}

fn part_2((ranges, own_ticket, other_tickets): &TParsed) -> u64 {
    let mut options = vec![];

    for (i, n) in own_ticket.iter().enumerate() {
        options.push(vec![]);
        for range in ranges {
            if range.in_range(*n) {
                options[i].push(range.clone());
            }
        }
    }

    for ticket in other_tickets {
        for (i, n) in ticket.iter().enumerate() {
            let mut to_remove = vec![];

            for (j, range) in options[i].iter().enumerate() {
                if !range.in_range(*n) {
                    to_remove.push(j);
                }
            }

            for j in to_remove {
                options[i].remove(j);
            }
        }
    }

    let mut taken = HashSet::new();

    'outer: loop {
        for opt in &mut options {
            if opt.len() == 1 {
                taken.insert(opt[0].name.to_owned());
                continue;
            } else {
                let mut to_remove = vec![];
                for (i, range) in opt.iter().enumerate() {
                    if taken.contains(&range.name) {
                        to_remove.push(i);
                    }
                }

                for j in to_remove {
                    opt.remove(j);
                }
            }
        }

        for opt in &options {
            if opt.len() > 1 {
                continue 'outer;
            }
        }

        break;
    }

    let mut result = vec![];
    for (i, opt) in options.iter().enumerate() {
        assert!(opt[0].in_range(own_ticket[i]));
        if !opt[0].name.starts_with("departure ") {
            continue;
        }
        result.push(own_ticket[i]);
    }

    result.iter().map(|x| *x as u64).product()
}

#[derive(Debug, Clone)]
struct DoubleRange {
    llow: u32,
    lhigh: u32,
    rlow: u32,
    rhigh: u32,
    name: String,
}

impl DoubleRange {
    fn new(llow: u32, lhigh: u32, rlow: u32, rhigh: u32, name: String) -> Self {
        Self {
            llow,
            lhigh,
            rlow,
            rhigh,
            name,
        }
    }

    fn in_range(&self, val: u32) -> bool {
        (val >= self.llow && val <= self.lhigh) || (val >= self.rlow && val <= self.rhigh)
    }
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input).0, 71)
}

#[test]
fn test_example_2() {
    let parsed_input = parse(&EXAMPLE_INPUT2);
    let (_, p1_output) = part_1(&parsed_input);
    assert_eq!(part_2(&(parsed_input.0, parsed_input.1, p1_output)), 1716)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
departure class: 0-1 or 4-19
departure row: 0-5 or 8-19
departure seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

fn parse(input: &str) -> TParsed {
    use regex::Regex;
    lazy_static! {
        static ref RANGES: Regex = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
    }

    let mut ranges = vec![];
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if RANGES.is_match(line) {
            let captures = RANGES.captures(line).unwrap();

            let range = DoubleRange::new(
                captures[2].parse().unwrap(),
                captures[3].parse().unwrap(),
                captures[4].parse().unwrap(),
                captures[5].parse().unwrap(),
                captures[1].to_owned(),
            );

            ranges.push(range);
        } else {
            break;
        }
    }

    lines.next();
    let own_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<_>>();

    let mut other_tickets = vec![];
    lines.next();
    lines.next();

    while let Some(line) = lines.next() {
        other_tickets.push(
            line.split(',')
                .filter_map(|c| c.parse().ok())
                .collect::<Vec<_>>(),
        )
    }

    (ranges, own_ticket, other_tickets)
}
