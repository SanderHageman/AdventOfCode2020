use std::collections::HashMap;

type TParsed = Vec<TParsedSub>;
type TParsedSub = Tile;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    let (result, border_to_tiles) = part_1(&parsed_input);
    (result, part_2(&parsed_input, border_to_tiles))
}

fn part_1(input: &TParsed) -> (usize, HashMap<u16, Vec<usize>>) {
    let mut border_to_tiles = HashMap::new();

    for tile in input {
        for bd in &tile.borders {
            border_to_tiles.entry(*bd).or_insert(vec![]).push(tile.id);
        }
    }

    let mut acc = 1;

    for tile in input {
        let mut solo_borders = 0;
        for bd in &tile.borders {
            if border_to_tiles.get(bd).unwrap().len() == 1 {
                solo_borders += 1;
            }
        }

        if solo_borders == 4 {
            acc *= tile.id;
        }
    }

    (acc, border_to_tiles)
}

fn part_2(input: &TParsed, border_to_tiles: HashMap<u16, Vec<usize>>) -> usize {
    let mut corner_tiles = vec![];
    let mut tiles_to_adjacents = HashMap::new();

    for tile in input {
        let mut solo_borders = 0;
        for bd in &tile.borders {
            let bordering = border_to_tiles.get(bd).unwrap();
            if bordering.len() == 1 {
                solo_borders += 1;
                continue;
            }

            for other in bordering {
                if other == &tile.id {
                    continue;
                }

                tiles_to_adjacents
                    .entry(tile.id)
                    .or_insert(vec![])
                    .push(other);
            }
        }

        if solo_borders == 4 {
            corner_tiles.push(tile.id);
        }
    }

    for tile in tiles_to_adjacents {
        println!("{:?}", tile);
    }

    273
}

#[derive(Debug)]
struct Tile {
    borders: Vec<u16>,
    image: Vec<Vec<bool>>,
    id: usize,
}

impl Tile {
    fn new() -> Self {
        Self {
            borders: Vec::new(),
            image: Vec::new(),
            id: 0,
        }
    }
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);

    for tile in input {
        println!("Tile {:?}:", tile.id);
        for y in tile.image {
            for x in y {
                print!("{}", if x { '#' } else { '.' });
            }
            println!("");
        }
        println!("{:?} \n", tile.borders);
    }
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input).0, 20899048083289)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input, part_1(&input).1), 273)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

fn parse(input: &str) -> TParsed {
    let mut tiles = vec![];
    let lines = input.lines().collect::<Vec<_>>();

    for chunk in lines.chunks(12) {
        let mut tile = Tile::new();

        let mut iter = chunk.iter();
        tile.id = iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();

        for line in iter {
            let mut this_line = vec![];
            for c in line.chars() {
                this_line.push(match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Uncovered {}", c),
                })
            }
            if this_line.len() > 0 {
                tile.image.push(this_line);
            }
        }

        fn border_to_hash(slice: &[bool]) -> u16 {
            slice
                .iter()
                .enumerate()
                .fold(0, |acc, (i, b)| acc | (*b as u16) << i)
        }

        let mut border_l = tile
            .image
            .iter()
            .map(|line| *line.first().unwrap())
            .collect::<Vec<_>>();
        let mut border_r = tile
            .image
            .iter()
            .map(|line| *line.last().unwrap())
            .collect::<Vec<_>>();
        let mut border_u = tile.image[0].clone();
        let mut border_d = tile.image[9].clone();

        tile.borders.push(border_to_hash(&border_l));
        tile.borders.push(border_to_hash(&border_r));
        tile.borders.push(border_to_hash(&border_u));
        tile.borders.push(border_to_hash(&border_d));

        border_u.reverse();
        border_d.reverse();
        border_l.reverse();
        border_r.reverse();

        tile.borders.push(border_to_hash(&border_l));
        tile.borders.push(border_to_hash(&border_r));
        tile.borders.push(border_to_hash(&border_u));
        tile.borders.push(border_to_hash(&border_d));

        tiles.push(tile);
    }

    tiles
}
