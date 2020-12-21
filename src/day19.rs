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
        let (result, _, max_pos) = test_case(0, &characters, &rules[&0], rules, 0);
        acc += (result && max_pos + 1 == characters.len()) as i64;
    }

    acc
}

fn part_2((rules, to_test): &TParsed) -> i64 {
    let mut rules = rules.clone();

    rules
        .entry(8)
        .and_modify(|r| *r = Rule::Or(vec![42], vec![42, 8]));

    rules
        .entry(11)
        .and_modify(|r| *r = Rule::Or(vec![42, 31], vec![42, 11, 31]));

    let mut acc = 0;

    for case in to_test {
        let characters = case.chars().collect::<Vec<_>>();

        for i in 0..characters.len() {
            let (res, _, max) = test_case(0, &characters, &rules[&0], &rules, i);
            if res && max + 1 == characters.len() {
                acc += 1;
                break;
            }
        }
    }

    acc
}

fn test_case(
    pos: usize,
    characters: &Vec<char>,
    rule: &Rule,
    rules: &TParsedS1,
    prefer_opt2_until: usize,
) -> (bool, usize, usize) {
    let result = match rule {
        Rule::Match(c_match) => {
            let res = if let Some(c_case) = characters.get(pos) {
                c_match == c_case
            } else {
                false
            };
            (res, 1, pos)
        }
        Rule::Or(opt1, opt2) => {
            let use_opt2_first = prefer_opt2_until > pos;
            let carry = opt1;
            let opt1 = if use_opt2_first { opt2 } else { opt1 };
            let opt2 = if use_opt2_first { carry } else { opt2 };

            let opt1 = Rule::Req(opt1.clone());
            let res1 = test_case(pos, characters, &opt1, rules, prefer_opt2_until);
            if res1.0 {
                res1
            } else {
                let opt2 = Rule::Req(opt2.clone());
                let res2 = test_case(pos, characters, &opt2, rules, prefer_opt2_until);
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
                let res = test_case(pos + acc, characters, rule, rules, prefer_opt2_until);
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
    assert_eq!(part_1(&parse(EX3)), 3);
}

#[test]
fn test_example_2() {
    let input = parse(EX3);
    assert_eq!(part_2(&input), 12)
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

#[cfg(test)]
const EX3: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

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
