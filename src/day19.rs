use std::collections::HashMap;

type TParsed = (TParsedS1, TParsedS2);
type TParsedS1 = HashMap<i32, Rule>;
type TParsedS2 = Vec<String>;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1((rules, to_test): &TParsed) -> i64 {
    let mut acc = 0;

    for case in to_test {
        let characters = case.chars().collect::<Vec<_>>();
        let (result, _, max_pos) = test_case(0, &characters, &rules[&0], rules);
        acc += (result && max_pos + 1 == characters.len()) as i64;
    }

    acc
}

fn test_case(
    pos: usize,
    characters: &Vec<char>,
    rule: &Rule,
    rules: &TParsedS1,
) -> (bool, usize, usize) {
    let result = match rule {
        Rule::Match(c) => (characters[pos] == *c, 1, pos),
        Rule::Or(opt1, opt2) => {
            let opt1 = Rule::Req(opt1.clone());
            let res1 = test_case(pos, characters, &opt1, rules);
            if res1.0 {
                res1
            } else {
                let opt2 = Rule::Req(opt2.clone());
                let res2 = test_case(pos, characters, &opt2, rules);
                if res2.0 {
                    res2
                } else {
                    (false, 0, 0)
                }
            }
        }
        Rule::Req(ins) => {
            let mut acc = 0;
            let mut max_pos = 0;
            for i in ins {
                let rule = &rules[i];
                let res = test_case(pos + acc, characters, rule, rules);
                if !res.0 {
                    return (false, 0, 0);
                }
                acc += res.1;
                max_pos = max_pos.max(res.2);
            }
            (true, acc, max_pos)
        }
    };

    result
}

fn part_2(_input: &TParsed) -> i64 {
    8
}

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Match(char),
    Or(Vec<i32>, Vec<i32>),
    Req(Vec<i32>),
}

#[test]
fn show_parse() {
    let input = parse(EX2);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    assert_eq!(part_1(&parse(EX1)), 2);
    assert_eq!(part_1(&parse(EX2)), 2);
}

#[test]
fn test_example_2() {
    let input = parse(EX1);
    assert_eq!(part_2(&input), 8)
}

#[cfg(test)]
const EX1: &str = "\
0: 1 2
1: \"a\"
2: 1 3 | 3 1
3: \"b\"

aab
aba
baa";

#[cfg(test)]
const EX2: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

fn parse(input: &str) -> TParsed {
    let mut pt1 = true;

    let mut rules = TParsedS1::new();
    let mut to_test = TParsedS2::new();

    for line in input.lines() {
        if pt1 {
            if line.is_empty() {
                pt1 = false;
                continue;
            }

            let mut split = line.split(':');
            let n = split.next().unwrap().parse::<i32>().unwrap();
            let rule = split.next().unwrap();

            let i = if rule.contains('|') {
                let mut or = rule.split('|');

                let v1 = or
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>();
                let v2 = or
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>();

                Rule::Or(v1, v2)
            } else if rule.contains('\"') {
                Rule::Match(rule.chars().find(|c| c.is_alphabetic()).unwrap())
            } else {
                let vals = rule
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>();

                Rule::Req(vals)
            };

            rules.insert(n, i);
        } else {
            to_test.push(line.to_owned());
        }
    }

    (rules, to_test)
}
