pub fn day(input: String) {
    let input_num = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    println!("Day 1 Result1: {:?}", part_1(&input_num));
    println!("Day 1 Result2: {:?}", part_2(&input_num));
}

fn part_1(input_num: &Vec<i32>) -> i32 {
    for (ix, x) in input_num.iter().enumerate() {
        for (iy, y) in input_num.iter().enumerate() {
            if ix != iy && x + y == 2020 {
                return x * y;
            };
        }
    }

    panic!("Unable to find answer")
}

fn part_2(input_num: &Vec<i32>) -> i32 {
    for (ix, x) in input_num.iter().enumerate() {
        for (iy, y) in input_num.iter().enumerate() {
            for (iz, z) in input_num.iter().enumerate() {
                if ix != iy && ix != iz && iy != iz && x + y + z == 2020 {
                    return x * y * z;
                };
            }
        }
    }

    panic!("Unable to find answer")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(part_1(&vec![1721, 979, 366, 299, 675, 1456]) == 514579)
    }

    #[test]
    fn test_example_2() {
        assert!(part_2(&vec![1721, 979, 366, 299, 675, 1456]) == 241861950)
    }
}
