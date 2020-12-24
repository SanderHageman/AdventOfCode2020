use std::collections::HashMap;
use vek::Vec3;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<Step>;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    let mut seen = HashMap::new();

    for steps in input {
        let mut pos = Vec3::new(0, 0, 0);
        for step in steps {
            pos += match step {
                Step::E => Vec3::new(-1, 1, 0),
                Step::SE => Vec3::new(-1, 0, 1),
                Step::SW => Vec3::new(0, -1, 1),
                Step::W => Vec3::new(1, -1, 0),
                Step::NW => Vec3::new(1, 0, -1),
                Step::NE => Vec3::new(0, 1, -1),
            }
        }
        *seen.entry(pos).or_insert(0) += 1
    }

    seen.iter().map(|(_, v)| v % 2).sum()
}

#[derive(Debug)]
enum Step {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn part_2(_input: &TParsed) -> usize {
    0
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 10)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 0)
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
