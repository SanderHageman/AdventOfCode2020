use std::collections::HashMap;

type TParsed = HashMap<String, TParsedSub>;
type TParsedSub = Rule;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    fn has_shiny<'a>((name, rule): &'a (&String, &Rule), input: &TParsed) -> bool {
        match name.as_str() {
            "shiny gold" => true,
            _ => rule
                .children
                .iter()
                .filter_map(|c| input.get_key_value(&c.name))
                .any(|r| has_shiny(&r, input)),
        }
    }
    input.iter().filter(|r| has_shiny(r, input)).count() - 1 // exclude shiny gold bag
}

fn part_2(input: &TParsed) -> usize {
    fn get_required_bags<'a>(rule: &'a &Rule, input: &TParsed) -> usize {
        rule.children
            .iter()
            .map(|c| (c.qty, input.get(&c.name).unwrap()))
            .map(|(qty, r)| qty * (get_required_bags(&r, input) + 1))
            .sum()
    }

    get_required_bags(&input.get("shiny gold").unwrap(), input)
}

fn parse(input: &str) -> TParsed {
    use regex::Regex;
    lazy_static! {
        static ref NAME: Regex = Regex::new(r"(\w* \w*) bags contain ").unwrap();
        static ref CHILD: Regex = Regex::new(r"(?P<qty>\d+) (?P<name>\w* \w*) bag").unwrap();
    }

    let mut result = TParsed::new();

    for line in input.lines() {
        let name = NAME.captures(line).unwrap()[1].to_owned();
        let children = CHILD
            .captures_iter(line)
            .map(|cap| Req {
                name: cap["name"].to_owned(),
                qty: cap["qty"].parse::<usize>().unwrap_or_default(),
            })
            .collect();

        result.insert(name.to_owned(), Rule { name, children });
    }

    result
}

#[derive(Debug)]
struct Rule {
    name: String,
    children: Vec<Req>,
}

#[derive(Debug)]
struct Req {
    name: String,
    qty: usize,
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 4)
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EXAMPLE_INPUT)), 32);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT2)), 126);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
