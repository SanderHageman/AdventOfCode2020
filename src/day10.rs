type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (u64, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> u64 {
    let mut ones = 0;
    let mut threes = 0;

    let mut full_sequence = input.clone();
    full_sequence.insert(0, 0);
    full_sequence.push(*input.last().unwrap() + 3);
    let full_sequence = full_sequence;

    for wnd in full_sequence.windows(2) {
        match wnd[1] - wnd[0] {
            // safe because sorted
            1 => ones += 1,
            3 => threes += 1,
            _ => {}
        }
    }

    ones * threes
}

fn part_2(input: &TParsed) -> usize {
    let mut value_track = vec![0; *input.last().unwrap()];
    let mut max = 0;

    for val in input {
        assert!(*val >= 1);

        let lo = *val as isize - 4;
        let mut acc = 0;

        for i in lo..*val as isize - 1 {
            if i < 0 {
                acc = 1;
                continue;
            }
            acc += value_track[i as usize];
        }

        value_track[*val - 1] = acc;
        max = max.max(acc);
    }

    max
}

fn parse(input: &str) -> TParsed {
    let mut result: Vec<_> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    result.sort();
    result
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    assert_eq!(part_1(&parse(EXAMPLE_INPUT)), 35);
    assert_eq!(part_1(&parse(EXAMPLE_INPUT2)), 220);
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EXAMPLE_INPUT)), 8);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT2)), 19208);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
