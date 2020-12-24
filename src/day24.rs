use std::collections::HashMap;
use vek::Vec3;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<Step>;

pub fn day(input: String) -> (i32, usize) {
    let parsed_input = parse(&input);
    let (p1_result, p1_output) = part_1(&parsed_input);
    (p1_result, part_2(p1_output))
}

fn part_1(input: &TParsed) -> (i32, HashMap<Vec3<i32>, i32>) {
    let mut seen = HashMap::new();

    for steps in input {
        let mut pos = Vec3::new(0, 0, 0);
        for step in steps {
            pos += Step::vec3(*step);
        }
        *seen.entry(pos).or_insert(0) += 1
    }

    (seen.iter().map(|(_, v)| v % 2).sum(), seen)
}

fn part_2(floor: HashMap<Vec3<i32>, i32>) -> usize {
    let nbs = Step::neighbours();

    let mut grid = floor
        .iter()
        .map(|(&k, v)| (k, v % 2 == 1))
        .collect::<HashMap<_, _>>();

    for _ in 0..100 {
        let mut write_grid = HashMap::from(grid.clone());

        for (tile, is_black) in &grid {
            let mut n_black_neighbours = 0;

            for nb in &nbs {
                let neighbour = nb + tile;
                if let Some(n) = grid.get(&neighbour) {
                    n_black_neighbours += if *n { 1 } else { 0 };
                } else {
                    write_grid.entry(neighbour).or_insert_with(|| {
                        let mut n_black = 0;
                        for nb in &nbs {
                            if let Some(n) = grid.get(&(nb + neighbour)) {
                                n_black += if *n { 1 } else { 0 };
                            }
                        }
                        n_black == 2
                    });
                }
            }

            if *is_black && n_black_neighbours == 0 || n_black_neighbours > 2 {
                *write_grid.get_mut(tile).unwrap() = false;
            } else if !is_black && n_black_neighbours == 2 {
                *write_grid.get_mut(tile).unwrap() = true;
            }
        }

        grid = write_grid;
    }

    grid.values().filter(|b| **b).count()
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input).0, 10)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(part_1(&input).1), 2208)
}

#[derive(Debug, Copy, Clone)]
enum Step {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Step {
    fn vec3(val: Step) -> Vec3<i32> {
        match val {
            Step::E => Vec3::new(-1, 1, 0),
            Step::SE => Vec3::new(-1, 0, 1),
            Step::SW => Vec3::new(0, -1, 1),
            Step::W => Vec3::new(1, -1, 0),
            Step::NW => Vec3::new(1, 0, -1),
            Step::NE => Vec3::new(0, 1, -1),
        }
    }

    fn neighbours() -> [Vec3<i32>; 6] {
        [
            Step::vec3(Step::E),
            Step::vec3(Step::SE),
            Step::vec3(Step::SW),
            Step::vec3(Step::W),
            Step::vec3(Step::NW),
            Step::vec3(Step::NE),
        ]
    }
}

fn parse(input: &str) -> TParsed {
    let mut result = vec![];
    for line in input.lines() {
        let mut iter = line.chars();
        let mut t = vec![];

        loop {
            let next = iter.next();

            if let Some(next) = next {
                t.push(match next {
                    'e' => Step::E,
                    's' => match iter.next().unwrap() {
                        'e' => Step::SE,
                        'w' => Step::SW,
                        _ => panic!("Unexpected"),
                    },
                    'w' => Step::W,
                    'n' => match iter.next().unwrap() {
                        'w' => Step::NW,
                        'e' => Step::NE,
                        _ => panic!("Unexpected"),
                    },
                    _ => panic!("Unexpected"),
                });
            } else {
                break;
            }
        }

        result.push(t);
    }

    result
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
