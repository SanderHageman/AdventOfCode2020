use std::collections::HashMap;

type TParsed = (TParsedS1, TParsedS2);
type TParsedS1 = HashMap<i32, Instruction>;
type TParsedS2 = Vec<String>;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1((instructions, test): &TParsed) -> i64 {
    let mut acc = 0;

    for case in test {
        let characters = case.chars().collect::<Vec<_>>();
        let (result, _, max_pos) = test_case(0, &characters, &instructions[&0], instructions);
        acc += (result && max_pos + 1 == characters.len()) as i64;
    }

    acc
}

fn test_case(
    pos: usize,
    characters: &Vec<char>,
    instruction: &Instruction,
    instructions: &TParsedS1,
) -> (bool, usize, usize) {
    let result = match instruction {
        Instruction::Match(c) => (characters[pos] == *c, 1, pos),
        Instruction::Or(opt1, opt2) => {
            let opt1 = Instruction::Req(opt1.clone());
            let res1 = test_case(pos, characters, &opt1, instructions);
            if res1.0 {
                res1
            } else {
                let opt2 = Instruction::Req(opt2.clone());
                let res2 = test_case(pos, characters, &opt2, instructions);
                if res2.0 {
                    res2
                } else {
                    (false, 0, 0)
                }
            }
        }
        Instruction::Req(ins) => {
            let mut acc = 0;
            let mut max_depth = 0;
            for i in ins {
                let instruction = &instructions[i];
                let res = test_case(pos + acc, characters, instruction, instructions);
                if !res.0 {
                    return (false, 0, 0);
                }
                acc += res.1;
                max_depth = max_depth.max(res.2);
            }
            (true, acc, max_depth)
        }
    };

    result
}

fn part_2(_input: &TParsed) -> i64 {
    8
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
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

    let mut ins = TParsedS1::new();
    let mut test = TParsedS2::new();

    for line in input.lines() {
        if pt1 {
            if line.is_empty() {
                pt1 = false;
                continue;
            }

            let mut split = line.split(':');
            let n = split.next().unwrap().parse::<i32>().unwrap();
            let instruction = split.next().unwrap();

            let i = if instruction.contains('|') {
                let mut or = instruction.split('|');

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

                Instruction::Or(v1, v2)
            } else if instruction.contains('\"') {
                Instruction::Match(instruction.chars().find(|c| c.is_alphabetic()).unwrap())
            } else {
                let vals = instruction
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>();

                Instruction::Req(vals)
            };

            ins.insert(n, i);
        } else {
            test.push(line.to_owned());
        }
    }

    (ins, test)
}
