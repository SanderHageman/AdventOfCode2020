use glm::*;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Instruction;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    let mut forward = vec2(1f32, 0f32);
    let mut pos = vec2(0f32, 0f32);

    for i in input {
        match i.ins {
            'N' => pos.y += i.val,
            'S' => pos.y -= i.val,
            'E' => pos.x += i.val,
            'W' => pos.x -= i.val,
            'L' | 'R' => forward = rotate(i, forward),
            'F' => pos += forward * i.val,
            _ => panic!("Uncovered instruction: {:?}", i),
        }
    }

    pos.abs().sum() as usize
}

fn part_2(input: &TParsed) -> usize {
    let mut pos = vec2(0f32, 0f32);
    let mut waypoint_pos = vec2(10f32, 1f32);

    for i in input {
        match i.ins {
            'N' => waypoint_pos.y += i.val,
            'S' => waypoint_pos.y -= i.val,
            'E' => waypoint_pos.x += i.val,
            'W' => waypoint_pos.x -= i.val,
            'L' | 'R' => waypoint_pos = rotate(i, waypoint_pos),
            'F' => pos += waypoint_pos * i.val,
            _ => panic!("Uncovered instruction: {:?}", i),
        }
    }

    pos.abs().sum() as usize
}

fn rotate(ins: &Instruction, mut vec: Vec2) -> Vec2 {
    let sign = if ins.ins == 'L' {
        vec2(-1f32, 1f32)
    } else {
        vec2(1f32, -1f32)
    };

    for _ in 0..(ins.val / 90f32).round() as usize {
        vec.swap_rows(0, 1);
        vec.component_mul_assign(&sign);
    }

    vec
}

fn parse(input: &str) -> TParsed {
    input.lines().map(Instruction::from).collect()
}

#[derive(Debug)]
struct Instruction {
    ins: char,
    val: f32,
}

impl From<&str> for Instruction {
    fn from(val: &str) -> Self {
        Self {
            ins: val.chars().nth(0).unwrap(),
            val: val.get(1..).unwrap().parse().unwrap(),
        }
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
    assert_eq!(part_1(&input), 25)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 286)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
F10
N3
F7
R90
F11";
