use itertools::*;
use std::collections::{HashMap, HashSet};
use vek::Vec2;

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
        for bd in &tile.default_borders {
            border_to_tiles.entry(*bd).or_insert(vec![]).push(tile.id);
        }
    }

    let mut acc = 1;

    for tile in input {
        let mut solo_borders = 0;
        for bd in &tile.default_borders {
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

fn part_2(tiles: &TParsed, border_to_tiles: HashMap<u16, Vec<usize>>) -> usize {
    let mut tiles_to_adjacents = HashMap::new();

    for tile in tiles {
        for bd in &tile.default_borders {
            let bordering = border_to_tiles.get(bd).unwrap();
            for other in bordering {
                if other == &tile.id {
                    continue;
                }

                tiles_to_adjacents
                    .entry(tile.id)
                    .or_insert(HashSet::new())
                    .insert(other);
            }
        }
    }

    let mut id_to_tile = tiles
        .iter()
        .map(|t| (t.id, t.clone()))
        .collect::<HashMap<_, _>>();

    let mut current_tile_id = *tiles_to_adjacents
        .iter()
        .find(|(_, v)| v.len() == 2)
        .unwrap()
        .0;

    {
        // set the initial rotation so that we can start the search Top-Left
        let mut adjacent = tiles_to_adjacents[&current_tile_id]
            .iter()
            .map(|&x| id_to_tile[x].clone())
            .collect_vec();
        let corner_tile = id_to_tile.get_mut(&current_tile_id).unwrap();

        for tile in adjacent.iter_mut() {
            for &pos in &[Pos::U, Pos::R, Pos::D, Pos::L] {
                if tile.rotate_to_fit(&corner_tile, pos) {
                    match pos {
                        Pos::U => flip_v(&mut corner_tile.image),
                        Pos::L => flip_h(&mut corner_tile.image),
                        Pos::R | Pos::D => { /* Do Nothing */ }
                    }
                    break;
                }
            }
        }

        corner_tile.break_fuse();
    }

    let mut direction = Pos::D;
    let mut just_changed_direction = true;

    loop {
        let mut adjacent = tiles_to_adjacents[&current_tile_id]
            .iter()
            .map(|&x| id_to_tile.remove(x).unwrap())
            .collect_vec();

        let current_tile = &id_to_tile[&current_tile_id];
        assert!(current_tile.fuse);

        let mut found = false;

        for tile in adjacent.iter_mut().filter(|t| !t.fuse) {
            if tile.rotate_to_fit(&current_tile, direction) {
                current_tile_id = tile.id;
                found = true;
                break;
            }
        }

        for tile in adjacent.drain(..) {
            id_to_tile.insert(tile.id, tile);
        }

        if !found {
            if just_changed_direction {
                // previous loop we also changed direction
                // we're in the center now
                break;
            } else {
                direction = direction.rotate_ccw();
                just_changed_direction = true;
            }
        } else {
            just_changed_direction = false;
        }
    }

    let grid_width = (id_to_tile.len() as f32).sqrt() as usize;
    let image_width = 8;

    let line_len = image_width * grid_width;
    let mut complete_image = vec![vec![false; line_len]; line_len];

    for tile in id_to_tile.values() {
        let mut y_pos = tile.pos.y as usize * image_width;
        let tile_image = tile.image_stripped_borders();

        for y in &tile_image {
            let mut x_pos = tile.pos.x as usize * image_width;
            for &x in y {
                complete_image[y_pos][x_pos] = x;
                x_pos += 1;
            }
            y_pos += 1;
        }
    }

    //create offsets to find seamonster
    let mut offsets = vec![];
    const SEA_MONSTER: &str = "\
    ..................#.
    #....##....##....###
    .#..#..#..#..#..#...";

    {
        let sea_monster_len = SEA_MONSTER.lines().next().unwrap().len();
        let mut offset = 0;
        for line in SEA_MONSTER.lines() {
            for c in line.chars() {
                match c {
                    '.' => {
                        offset += 1;
                    }
                    '#' => {
                        offsets.push(offset);
                        offset += 1;
                    }
                    _ => { /* Do Nothing*/ }
                }
            }
            offset += line_len - sea_monster_len;
        }

        let bck = offsets[0];
        for offset in offsets.iter_mut() {
            *offset -= bck;
        }
    }

    let actions = [
        nop, rotate_cw, rotate_cw, rotate_cw, flip_h, nop, rotate_cw, rotate_cw, rotate_cw,
    ];

    let mut found = None;
    for action in &actions {
        action(&mut complete_image);
        if let Some(result) = find_monsters(&complete_image, &offsets) {
            found = Some(result);
            break;
        }
    }

    let total_water_count = complete_image.iter().flatten().filter(|&b| *b).count();
    let monster_count = offsets.len() * found.unwrap();

    total_water_count - monster_count
}

fn find_monsters(image: &Vec<Vec<bool>>, offsets: &Vec<usize>) -> Option<usize> {
    let flat_image = image.iter().flatten().cloned().collect_vec();
    let mut found = 0;

    'outer: for (i, b) in flat_image.iter().enumerate() {
        if !b {
            continue;
        }

        for &offset in offsets {
            let test = i + offset;

            if !flat_image.get(test).unwrap_or(&false) {
                continue 'outer;
            }
        }
        found += 1;
    }

    if found > 0 {
        Some(found)
    } else {
        None
    }
}

fn rotate_cw(image: &mut Vec<Vec<bool>>) {
    let copy = image.clone();
    let len = image.len();

    for y in 0..len {
        for x in 0..len {
            image[y][x] = copy[len - 1 - x][y];
        }
    }
}

fn flip_h(image: &mut Vec<Vec<bool>>) {
    for row in image.iter_mut() {
        row.reverse();
    }
}

fn flip_v(image: &mut Vec<Vec<bool>>) {
    let len = image[0].len();
    for col in 0..len {
        for row in 0..len / 2 {
            let mut temp = image[len - 1 - row][col];
            std::mem::swap(&mut image[row][col], &mut temp);
            std::mem::swap(&mut image[len - 1 - row][col], &mut temp);
        }
    }
}

fn nop(_: &mut Vec<Vec<bool>>) {
    /* Do Nothing */
}

#[derive(Debug, Clone)]
struct Tile {
    default_borders: Vec<u16>,
    image: Vec<Vec<bool>>,
    id: usize,
    pos: Vec2<i8>,
    fuse: bool,
}

impl Tile {
    fn new() -> Self {
        Self {
            default_borders: Vec::new(),
            image: Vec::new(),
            id: 0,
            pos: Vec2::zero(),
            fuse: false,
        }
    }

    fn image_stripped_borders(&self) -> Vec<Vec<bool>> {
        let mut result = self.image.clone();
        {
            result.remove(0);
            result.pop();

            result.iter_mut().for_each(|row| {
                row.remove(0);
                row.pop();
            });
        }
        result
    }

    fn break_fuse(&mut self) {
        self.check_fuse();
        self.fuse = true;
    }

    fn check_fuse(&self) {
        if self.fuse {
            panic!("Fuse broken! ({})", self.id)
        }
    }

    fn _get_borders(&self) -> Vec<u16> {
        vec![
            self.get_bd(Pos::U),
            self.get_bd(Pos::R),
            self.get_bd(Pos::D),
            self.get_bd(Pos::L),
        ]
    }

    fn border_to_hash<'a, I>(iter: I) -> u16
    where
        I: IntoIterator<Item = &'a bool>,
    {
        iter.into_iter()
            .enumerate()
            .fold(0, |acc, (i, b)| acc | (*b as u16) << i)
    }

    fn get_bd(&self, pos: Pos) -> u16 {
        match pos {
            Pos::U => Tile::border_to_hash(self.image.first().unwrap()),
            Pos::R => Tile::border_to_hash(self.image.iter().map(|row| row.last().unwrap())),
            Pos::D => Tile::border_to_hash(self.image.last().unwrap()),
            Pos::L => Tile::border_to_hash(self.image.iter().map(|row| row.first().unwrap())),
        }
    }

    fn rotate_to_fit(&mut self, other: &Tile, pos_other: Pos) -> bool {
        self.check_fuse();

        let bd_other = other.get_bd(pos_other);

        let target_pos = match pos_other {
            Pos::U => Pos::D,
            Pos::R => Pos::L,
            Pos::D => Pos::U,
            Pos::L => Pos::R,
        };

        let actions = [
            nop, rotate_cw, rotate_cw, rotate_cw, flip_h, nop, rotate_cw, rotate_cw, rotate_cw,
        ];

        let mut result = false;

        for action in &actions {
            action(&mut self.image);
            if self.get_bd(target_pos) == bd_other {
                self.pos = other.pos
                    + match pos_other {
                        Pos::U => Vec2::new(0, -1),
                        Pos::R => Vec2::new(1, 0),
                        Pos::D => Vec2::new(0, 1),
                        Pos::L => Vec2::new(-1, 0),
                    };

                result = true;
                self.break_fuse();
                break;
            }
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pos {
    U,
    R,
    D,
    L,
}

impl Pos {
    fn rotate_ccw(&self) -> Pos {
        match self {
            Pos::U => Pos::L,
            Pos::R => Pos::U,
            Pos::D => Pos::R,
            Pos::L => Pos::D,
        }
    }
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);

    for tile in input {
        _print_tile(&tile);
    }
}

fn _print_tile(tile: &Tile) {
    println!("Tile {:?} at Pos {:?}:", tile.id, tile.pos);
    println!(
        "Fuse: {}, Borders {:?}",
        if tile.fuse { "broken" } else { "intact" },
        tile._get_borders()
    );

    for y in &tile.image {
        for x in y {
            print!("{}", if *x { '#' } else { '.' });
        }
        println!("");
    }
    println!("");
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

        tile.default_borders.push(Tile::border_to_hash(&border_l));
        tile.default_borders.push(Tile::border_to_hash(&border_r));
        tile.default_borders.push(Tile::border_to_hash(&border_u));
        tile.default_borders.push(Tile::border_to_hash(&border_d));

        border_u.reverse();
        border_d.reverse();
        border_l.reverse();
        border_r.reverse();

        tile.default_borders.push(Tile::border_to_hash(&border_l));
        tile.default_borders.push(Tile::border_to_hash(&border_r));
        tile.default_borders.push(Tile::border_to_hash(&border_u));
        tile.default_borders.push(Tile::border_to_hash(&border_d));

        tiles.push(tile);
    }

    tiles
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
