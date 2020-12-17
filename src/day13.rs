type TParsed = (usize, TParsedSub);
type TParsedSub = Vec<usize>;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1((target, input): &TParsed) -> usize {
    let mut min_wait = usize::MAX;
    let mut min_bus = usize::MAX;

    for bus_id in input.iter().filter(|x| *x > &0usize) {
        let wait_time = bus_id - (target % bus_id);
        if wait_time < min_wait {
            min_wait = wait_time;
            min_bus = *bus_id;
        }
    }

    min_wait * min_bus
}

fn part_2((_, input): &TParsed) -> usize {
    let mut nums = input
        .iter()
        .enumerate()
        .filter(|(_, x)| *x > &0)
        .map(|(i, x)| (*x, i))
        .collect::<Vec<_>>();

    let (mut step, _) = nums.remove(0);
    let nums = nums;

    let mut counter = 0;

    'nums: for (num, offset) in nums {
        let mut target = num as i64 - offset as i64;
        while target < 0 {
            target += num as i64;
        }

        let target = target as usize;

        loop {
            counter += step;
            if counter % num == target {
                step *= num;
                continue 'nums;
            };
        }
    }

    counter
}

fn parse(input: &str) -> TParsed {
    let target = input.lines().nth(0).unwrap().parse::<usize>().unwrap();
    let split = input
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap_or_default())
        .collect::<Vec<_>>();
    (target, split)
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 295)
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EXAMPLE_INPUT)), 1068781);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT2)), 3417);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT3)), 754018);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT4)), 779210);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT5)), 1261476);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT6)), 1202161486);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
939
7,13,x,x,59,x,31,19";

#[cfg(test)]
const EXAMPLE_INPUT2: &str = "\
0
17,x,13,19";

#[cfg(test)]
const EXAMPLE_INPUT3: &str = "\
0
67,7,59,61";

#[cfg(test)]
const EXAMPLE_INPUT4: &str = "\
0
67,x,7,59,61";

#[cfg(test)]
const EXAMPLE_INPUT5: &str = "\
0
67,7,x,59,61";

#[cfg(test)]
const EXAMPLE_INPUT6: &str = "\
0
1789,37,47,1889";
