type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    get_nth(input, 2020)
}

fn part_2(input: &TParsed) -> usize {
    get_nth(input, 30000000)
}

fn get_nth(input: &TParsed, n: usize) -> usize {
    let mut register = vec![0; n];

    input
        .iter()
        .enumerate()
        .for_each(|(i, val)| register[*val] = i + 1);

    let mut previous_num = *input.last().unwrap();

    for i in input.len()..n {
        let prev = register[previous_num];
        register[previous_num] = i;
        previous_num = if prev == 0 { 0 } else { i - prev };
    }

    previous_num
}

fn parse(input: &str) -> TParsed {
    input.split(',').map(|c| c.parse::<_>().unwrap()).collect()
}

#[test]
fn show_parse() {
    let input = parse(EX1);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    assert_eq!(part_1(&parse(EX1)), 436);
    assert_eq!(part_1(&parse(EX2)), 1);
    assert_eq!(part_1(&parse(EX3)), 10);
    assert_eq!(part_1(&parse(EX4)), 27);
    assert_eq!(part_1(&parse(EX5)), 78);
    assert_eq!(part_1(&parse(EX6)), 438);
    assert_eq!(part_1(&parse(EX7)), 1836);
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EX1)), 175594);
    assert_eq!(part_2(&parse(EX2)), 2578);
    assert_eq!(part_2(&parse(EX3)), 3544142);
    assert_eq!(part_2(&parse(EX4)), 261214);
    assert_eq!(part_2(&parse(EX5)), 6895259);
    assert_eq!(part_2(&parse(EX6)), 18);
    assert_eq!(part_2(&parse(EX7)), 362);
}

#[cfg(test)]
const EX1: &str = "\
0,3,6";
#[cfg(test)]
const EX2: &str = "\
1,3,2";
#[cfg(test)]
const EX3: &str = "\
2,1,3";
#[cfg(test)]
const EX4: &str = "\
1,2,3";
#[cfg(test)]
const EX5: &str = "\
2,3,1";
#[cfg(test)]
const EX6: &str = "\
3,2,1";
#[cfg(test)]
const EX7: &str = "\
3,1,2";
