use super::cpu;
use std::collections::HashSet;

type TParsed = Vec<TParsedSub>;
type TParsedSub = cpu::Instruction;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> i64 {
    let mut cpu = cpu::CPU::new(input);
    let mut set = HashSet::new();
    let mut acc = 0;

    while set.insert(cpu.next()) {
        acc = cpu.get_acc_value();
    }

    acc
}

fn part_2(input: &TParsed) -> i64 {
    'outer: for (i, ins) in input.iter().enumerate() {
        let instruction = match ins {
            cpu::Instruction::Acc(_) => continue,
            cpu::Instruction::Jmp(cnt) => cpu::Instruction::Nop(*cnt),
            cpu::Instruction::Nop(cnt) => cpu::Instruction::Jmp(*cnt),
        };

        let mut input_clone = input.clone();
        input_clone[i] = instruction;

        let mut cpu = cpu::CPU::new_owned(input_clone);
        let mut set = HashSet::new();

        while let Some(pos) = cpu.next() {
            if !set.insert(pos) {
                continue 'outer;
            }
        }

        return cpu.get_acc_value();
    }

    panic!("Unable to find answer")
}

fn parse(input: &str) -> TParsed {
    input.lines().map(cpu::Instruction::from).collect()
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 5)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 8)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
