use std::collections::HashMap;
use vek::{Vec3, Vec4};

type TParsed = Vec<TParsedSub>;
type TParsedSub = (Vec3<isize>, bool);

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> usize {
    let nbs = get_nbs();
    let mut grid = input.iter().cloned().collect::<HashMap<_, _>>();

    for _ in 0..6 {
        let mut write_grid = HashMap::from(grid.clone());

        for (cube, cube_active) in &grid {
            let mut n_active_neighbours = 0;

            for nb in &nbs {
                let neighbour = nb + cube;
                if let Some(n) = grid.get(&neighbour) {
                    n_active_neighbours += if *n { 1 } else { 0 };
                } else {
                    write_grid.entry(neighbour).or_insert_with(|| {
                        let mut n_active = 0;
                        for nb in &nbs {
                            if let Some(n) = grid.get(&(nb + neighbour)) {
                                n_active += if *n { 1 } else { 0 };
                            }
                        }
                        n_active == 3
                    });
                }
            }

            if *cube_active {
                n_active_neighbours -= 1;
                let should = n_active_neighbours == 2 || n_active_neighbours == 3;
                *write_grid.get_mut(cube).unwrap() = should;
            } else if !cube_active && n_active_neighbours == 3 {
                *write_grid.get_mut(cube).unwrap() = true;
            }
        }

        grid = write_grid;
        // print_grid(&grid);
    }

    grid.values().filter(|b| **b).count()
}

fn part_2(input: &TParsed) -> usize {
    let nbs = get_nbs4();
    let mut grid = input
        .iter()
        .cloned()
        .map(|(pos, state)| (Vec4::from(pos), state))
        .collect::<HashMap<_, _>>();

    for _ in 0..6 {
        let mut write_grid = HashMap::from(grid.clone());

        for (cube, cube_active) in &grid {
            let mut n_active_neighbours = 0;

            for nb in &nbs {
                let neighbour = nb + cube;
                if let Some(n) = grid.get(&neighbour) {
                    n_active_neighbours += if *n { 1 } else { 0 };
                } else {
                    write_grid.entry(neighbour).or_insert_with(|| {
                        let mut n_active = 0;
                        for nb in &nbs {
                            if let Some(n) = grid.get(&(nb + neighbour)) {
                                n_active += if *n { 1 } else { 0 };
                            }
                        }
                        n_active == 3
                    });
                }
            }

            if *cube_active {
                n_active_neighbours -= 1;
                let should = n_active_neighbours == 2 || n_active_neighbours == 3;
                *write_grid.get_mut(cube).unwrap() = should;
            } else if !cube_active && n_active_neighbours == 3 {
                *write_grid.get_mut(cube).unwrap() = true;
            }
        }

        grid = write_grid;
        //print_grid4(&grid);
    }

    grid.values().filter(|b| **b).count()
}

fn get_nbs4() -> Vec<Vec4<isize>> {
    fn vec4(x: isize, y: isize, z: isize, w: isize) -> Vec4<isize> {
        Vec4 { x, y, z, w }
    }

    let mut result = vec![];

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                for w in -1..2 {
                    result.push(vec4(x, y, z, w));
                }
            }
        }
    }

    result
}

fn get_nbs() -> Vec<Vec3<isize>> {
    fn vec3(x: isize, y: isize, z: isize) -> Vec3<isize> {
        Vec3 { x, y, z }
    }

    let mut result = vec![];

    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                result.push(vec3(x, y, z));
            }
        }
    }

    result
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 112)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(&input), 848)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
.#.
..#
###";

fn parse(input: &str) -> TParsed {
    let mut result = TParsed::new();
    for (y, l) in input.lines().enumerate() {
        for (x, s) in l.chars().enumerate() {
            result.push((Vec3::new(x as isize, y as isize, 0), s == '#'));
        }
    }

    result
}

fn _print_grid(grid: &HashMap<Vec3<isize>, bool>) {
    let mut min_z = isize::MAX;
    let mut max_z = 0;
    let mut min = isize::MAX;
    let mut max = 0;

    for (pos, _) in grid {
        max = max.max(pos.y.max(pos.x));
        min = min.min(pos.y.min(pos.x));

        max_z = max_z.max(pos.z);
        min_z = min_z.min(pos.z);
    }

    for z in min_z..max_z + 1 {
        println!("\n{}", z);
        for y in min..max + 1 {
            let mut line = vec![];
            for x in min..max + 1 {
                line.push(if *grid.get(&Vec3::new(x, y, z)).unwrap_or(&false) {
                    '#'
                } else {
                    '.'
                });
            }

            for c in line {
                print!("{}", c);
            }
            println!("");
        }
    }
}

fn _print_grid4(grid: &HashMap<Vec4<isize>, bool>) {
    let mut min_w = isize::MAX;
    let mut max_w = 0;
    let mut min_z = isize::MAX;
    let mut max_z = 0;
    let mut min = isize::MAX;
    let mut max = 0;

    for (pos, _) in grid {
        max = max.max(pos.y.max(pos.x));
        min = min.min(pos.y.min(pos.x));

        max_z = max_z.max(pos.z);
        min_z = min_z.min(pos.z);

        max_w = max_w.max(pos.w);
        min_w = min_w.min(pos.w);
    }

    for w in min_w..max_w + 1 {
        for z in min_z..max_z + 1 {
            println!("\n Z={} W={}", z, w);
            for y in min..max + 1 {
                let mut line = vec![];
                for x in min..max + 1 {
                    line.push(if *grid.get(&Vec4::new(x, y, z, w)).unwrap_or(&false) {
                        '#'
                    } else {
                        '.'
                    });
                }

                for c in line {
                    print!("{}", c);
                }
                println!("");
            }
        }
    }
}
