type TParsed = Vec<TParsedSub>;
type TParsedSub = u64;

pub fn day(input: String) -> (u64, u64) {
    let parsed_input = parse(&input);
    let p1 = part_1(&parsed_input, 25);
    (p1, part_2(&parsed_input, p1))
}

fn part_1(input: &TParsed, preamble: usize) -> u64 {
    'outer: for wnd in input.windows(preamble + 1) {
        let target = wnd[preamble];
        for cmb in comb(&wnd[..preamble], 2) {
            if cmb[0] + cmb[1] == target {
                continue 'outer;
            }
        }
        return target;
    }
    0
}

fn part_2(input: &TParsed, p1: u64) -> u64 {
    for i in 2..25 {
        for wnd in input.windows(i) {
            if wnd.iter().sum::<u64>() == p1 {
                let lo = wnd.iter().min().unwrap();
                let hi = wnd.iter().max().unwrap();
                return lo + hi;
            }
        }
    }
    0
}

fn parse(input: &str) -> TParsed {
    input.lines().map(|ln| ln.parse::<u64>().unwrap()).collect()
}

//https://rosettacode.org/wiki/Combinations#Rust
fn comb<T>(slice: &[T], k: usize) -> Vec<Vec<T>>
where
    T: Copy,
{
    // If k == 1, return a vector containing a vector for each element of the slice.
    if k == 1 {
        return slice.iter().map(|x| vec![*x]).collect::<Vec<Vec<T>>>();
    }
    // If k is exactly the slice length, return the slice inside a vector.
    if k == slice.len() {
        return vec![slice.to_vec()];
    }

    // Make a vector from the first element + all combinations of k - 1 elements of the rest of the slice.
    let mut result = comb(&slice[1..], k - 1)
        .into_iter()
        .map(|x| [&slice[..1], x.as_slice()].concat())
        .collect::<Vec<Vec<T>>>();

    // Extend this last vector with the all the combinations of k elements after from index 1 onward.
    result.extend(comb(&slice[1..], k));
    // Return final vector.
    return result;
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input, 5), 127)
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EXAMPLE_INPUT), 127), 62);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
