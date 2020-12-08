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
    let mut changed_i = usize::MAX;

    for _ in 0..input.len() {
        let mut a = input.clone();
        for (i, ins) in input.iter().enumerate() {
            if changed_i != usize::MAX && i <= changed_i {
                continue;
            }

            changed_i = i;
            a[i] = match ins {
                cpu::Instruction::Acc(_) => continue,
                cpu::Instruction::Jmp(cnt) => cpu::Instruction::Nop(*cnt),
                cpu::Instruction::Nop(cnt) => cpu::Instruction::Jmp(*cnt),
            };

            break;
        }

        let mut cpu = cpu::CPU::new_owned(a);
        let mut set = HashSet::new();

        loop {
            let pos = cpu.next();
            if !set.insert(pos) {
                break;
            }

            if pos.is_none() {
                return cpu.get_acc_value();
            }
        }
    }

    panic!("Unable to find answer")
}

fn parse(input: &str) -> TParsed {
    input.lines().map(cpu::Instruction::from).collect()
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    dbg!(input);
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
