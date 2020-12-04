pub fn day(input: String) -> (i32, i32) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input_num: &Vec<i32>) -> i32 {
    for (ix, x) in input_num.iter().enumerate() {
        for y in input_num[ix + 1..].iter() {
            if x + y == 2020 {
                return x * y;
            };
        }
    }

    panic!("Unable to find answer")
}

fn part_2(input_num: &Vec<i32>) -> i32 {
    for (ix, x) in input_num.iter().enumerate() {
        for (iy, y) in input_num[ix + 1..].iter().enumerate() {
            for z in input_num[iy + 1..].iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                };
            }
        }
    }

    panic!("Unable to find answer")
}

fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert!(part_1(&input) == 514579)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert!(part_2(&input) == 241861950)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
1721
979
366
299
675
1456";
