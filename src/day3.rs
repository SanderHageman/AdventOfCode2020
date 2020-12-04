type TParsed = Vec<Vec<bool>>;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> i64 {
    get_trees_for_slope(&(3, 1), input)
}

fn part_2(input: &TParsed) -> i64 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|slope| get_trees_for_slope(slope, input))
        .product()
}

fn get_trees_for_slope(slope: &(i32, i32), map: &TParsed) -> i64 {
    let mut pos = slope.clone();
    let mut trees = 0i64;

    while let Some(is_tree) = is_tree(&pos, map) {
        pos.0 += slope.0;
        pos.1 += slope.1;
        trees += is_tree as i64;
    }

    trees
}

fn is_tree(pos: &(i32, i32), map: &TParsed) -> Option<bool> {
    let map_x = map[0].len() as i32;
    let map_y = map.len() as i32;
    let target_x = (pos.0 % map_x) as usize;

    if pos.1 < map_y {
        Some(map[pos.1 as usize][target_x])
    } else {
        None
    }
}

fn parse(input: &str) -> TParsed {
    input
        .lines()
        .map(|line| line.chars().map(|x| x == '#').collect())
        .collect()
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 7)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 336)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
