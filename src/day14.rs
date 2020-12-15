use std::collections::HashMap;
use std::iter::FromIterator;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Instruction;

pub fn day(input: String) -> (u64, u64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> u64 {
    let mut current_mask = &Instruction::Mask(String::new());
    let mut registers = HashMap::new();

    for ins in input {
        match ins {
            Instruction::Mask(_) => current_mask = &ins,
            Instruction::Mem(addr, val) => {
                registers.insert(*addr, U36::masked_u64(val, &current_mask));
            }
        }
    }

    registers.values().map(|r| r.val).sum()
}

fn part_2(input: &TParsed) -> u64 {
    let mut current_mask = &Instruction::Mask(String::new());
    let mut registers = HashMap::new();

    for ins in input {
        match ins {
            Instruction::Mask(_) => current_mask = &ins,
            Instruction::Mem(addr, val) => {
                let masked_values = U36F::masked_u64(&(*addr as u64), &current_mask);
                for addr in masked_values.nums {
                    registers.insert(addr, *val);
                }
            }
        }
    }

    registers.values().sum()
}

#[derive(Debug)]
enum Instruction {
    Mask(String),
    Mem(usize, u64),
}

#[derive(Debug, Clone)]
struct U36 {
    val: u64,
}

impl U36 {
    fn masked_u64(val: &u64, mask: &Instruction) -> Self {
        let val_as_string = format!("{:036b}", val);
        if let Instruction::Mask(mask) = mask {
            let val = Self::apply_mask(val_as_string.as_str(), mask.as_str());
            Self {
                val: Self::as_num(&val),
            }
        } else {
            panic!("Can't apply mem to mem")
        }
    }

    fn as_num(val: &str) -> u64 {
        u64::from_str_radix(val, 2).unwrap_or_default()
    }

    fn apply_mask(val: &str, mask: &str) -> String {
        mask.chars()
            .zip(val.chars())
            .map(|(c_mask, c_val)| match c_mask {
                'X' => c_val,
                _ => c_mask,
            })
            .collect()
    }
}

fn parse(input: &str) -> TParsed {
    use regex::Regex;
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"mask = ((X|1|0){36})").unwrap();
        static ref MEM: Regex = Regex::new(r"mem\[(?P<addr>\d*)\] = (?P<val>\d*)").unwrap();
    }

    let mut result = TParsed::new();
    for line in input.lines() {
        result.push(if MASK.is_match(line) {
            Instruction::Mask(MASK.captures(line).unwrap()[1].to_owned())
        } else {
            let cap = MEM.captures(line).unwrap();
            Instruction::Mem(
                cap["addr"].parse::<usize>().unwrap(),
                cap["val"].parse::<u64>().unwrap(),
            )
        });
    }

    result
}

#[derive(Debug, Clone)]
struct U36F {
    nums: Vec<u64>,
}

impl U36F {
    fn masked_u64(val: &u64, mask: &Instruction) -> Self {
        let val_as_string = format!("{:036b}", val);
        let masked_value = if let Instruction::Mask(mask) = mask {
            Self::apply_mask(val_as_string.as_str(), mask.as_str())
        } else {
            panic!("Can't apply mem to mem")
        };

        Self {
            nums: Self::as_nums(&masked_value),
        }
    }

    fn as_nums(val: &str) -> Vec<u64> {
        let chars = val.chars().collect::<Vec<_>>();
        let x_indices = val
            .char_indices()
            .rev()
            .filter_map(|(i, c)| if c == 'X' { Some(i) } else { None })
            .collect::<Vec<_>>();

        let mut result = vec![];

        for i in 0..1 << x_indices.len() {
            let mut out = chars.clone();

            let set = format!("{:b}", i);
            for (i, ii) in (&x_indices).iter().enumerate() {
                out[*ii] = set.chars().rev().nth(i).unwrap_or('0');
            }

            let out = String::from_iter(out);
            result.push(u64::from_str_radix(&out, 2).unwrap_or_default())
        }

        result
    }

    fn apply_mask(val: &str, mask: &str) -> String {
        mask.chars()
            .zip(val.chars())
            .map(|(c_mask, c_val)| match c_mask {
                'X' => 'X',
                '0' => c_val,
                '1' => c_mask,
                _ => panic!("Uncovered: {}", c_mask),
            })
            .collect()
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
    assert_eq!(part_1(&input), 165)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT2);
    assert_eq!(part_2(&input), 208)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
