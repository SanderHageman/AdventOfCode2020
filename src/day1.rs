pub fn day(input: String) {
    let mut result1 = 0;
    let mut result2 = 0;

    let inputNum = input
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    'outer1: for (ix, x) in inputNum.iter().enumerate() {
        for (iy, y) in inputNum.iter().enumerate() {
            if ix != iy && x + y == 2020 {
                result1 = x * y;
                break 'outer1;
            };
        }
    }

    'outer2: for (ix, x) in inputNum.iter().enumerate() {
        for (iy, y) in inputNum.iter().enumerate() {
            for (iz, z) in inputNum.iter().enumerate() {
                if ix != iy && ix != iz && iy != iz && x + y + z == 2020 {
                    result2 = x * y * z;
                    break 'outer2;
                };
            }
        }
    }

    println!("Day 1 Result1: {:?}", result1);
    println!("Day 1 Result2: {:?}", result2);
}
