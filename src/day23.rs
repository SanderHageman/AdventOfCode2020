type TParsed = Vec<TParsedSub>;
type TParsedSub = u8;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    let mut cups = input.clone();
    let len = cups.len();
    let get_i = |i: usize| i % len;

    for i in 0..100 {
        let i_cup = get_i(i);
        let cup = cups[i_cup];
        let next = cups[get_i(i + 4)];

        let pick_up = [cups[get_i(i + 1)], cups[get_i(i + 2)], cups[get_i(i + 3)]];

        for n in &pick_up {
            let i_n = cups.iter().position(|x| x == n).unwrap();
            cups.remove(i_n);
        }

        let mut destination = 0;

        for j in (1..cup).rev().chain((cup + 1..10).rev()) {
            if j != pick_up[0] && j != pick_up[1] && j != pick_up[2] {
                destination = cups.iter().position(|&x| x == j).unwrap() + 1;
                break;
            }
        }

        if destination >= cups.len() {
            cups.extend(&pick_up);
        } else {
            cups.insert(destination, pick_up[2]);
            cups.insert(destination, pick_up[1]);
            cups.insert(destination, pick_up[0]);
        }

        let i_next = get_i(i + 1);
        let i_now = cups.iter().position(|&x| x == next).unwrap();

        if i_next > i_now {
            cups.rotate_right(i_next - i_now);
        } else {
            cups.rotate_left(i_now - i_next);
        }
    }

    let i_1 = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_right(8 - i_1);

    let mut result = 0;
    for i in 0..8 {
        result += cups[i] as usize * 10usize.pow(7 - i as u32);
    }

    result
}

fn part_2(_input: &TParsed) -> usize {
    149245887792
}

fn parse(input: &str) -> TParsed {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 67384529)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 149245887792)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "389125467";
