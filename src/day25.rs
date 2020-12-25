type TParsed = Vec<TParsedSub>;
type TParsedSub = usize;

pub fn day(input: String) -> (usize, &'static str) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), "The End!")
}

fn part_1(input: &TParsed) -> usize {
    let mut value = 1;
    let mut counter = 0usize;

    loop {
        value = value * 7;
        value %= 20201227;
        counter += 1;

        if value == input[1] {
            break;
        }
    }

    value = 1;
    for _ in 0..counter {
        value = value * input[0];
        value %= 20201227;
    }

    value
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 14897079)
}

fn parse(input: &str) -> TParsed {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
5764801
17807724";
