use std::collections::HashMap;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Group;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    input.iter().map(|g| g.map.len()).sum()
}

fn part_2(input: &TParsed) -> usize {
    input.iter().map(|g| g.get_all_yes()).sum()
}

fn parse(input: &str) -> TParsed {
    let mut result = TParsed::new();
    let mut group = Group::new();

    for line in input.lines() {
        // new group
        if line.is_empty() {
            result.push(group);
            group = Group::new();
            continue;
        }

        group.group_size += 1;
        for c in line.chars() {
            (*group.map.entry(c).or_insert(0)) += 1;
        }
    }

    result.push(group);
    result
}

#[derive(Debug)]
struct Group {
    group_size: usize,
    map: HashMap<char, usize>,
}

impl Group {
    fn new() -> Group {
        Group {
            group_size: 0,
            map: HashMap::new(),
        }
    }

    fn get_all_yes(&self) -> usize {
        self.map.iter().filter(|c| *c.1 >= self.group_size).count()
    }
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 11)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 6)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
