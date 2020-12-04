pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(passes: &Vec<Pass>) -> usize {
    passes.iter().filter(|i| i.is_correct_format()).count()
}

fn part_2(passes: &Vec<Pass>) -> usize {
    passes.iter().filter(|i| i.is_correct_format2()).count()
}

fn parse(input: &str) -> Vec<Pass> {
    input.lines().map(|line| Pass::from(line)).collect()
}

#[derive(Debug)]
struct Pass {
    low_lim: usize,
    high_lim: usize,
    char: char,
    password: String,
}

impl Pass {
    fn is_correct_format(&self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.char).count();
        count >= self.low_lim && count <= self.high_lim
    }

    fn is_correct_format2(&self) -> bool {
        let l = self.password.chars().nth(self.low_lim - 1).unwrap_or('@');
        let h = self.password.chars().nth(self.high_lim - 1).unwrap_or('@');
        (l == self.char) != (h == self.char)
    }
}

impl From<&str> for Pass {
    fn from(val: &str) -> Self {
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d*)-(\d*) (.): (\w*)").unwrap();
        }

        let captures = RE.captures(val).unwrap();

        Pass {
            low_lim: captures[1].parse::<usize>().unwrap(),
            high_lim: captures[2].parse::<usize>().unwrap(),
            char: captures[3].chars().next().unwrap(),
            password: captures[4].to_owned(),
        }
    }
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert!(part_1(&input) == 2)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert!(part_2(&input) == 1)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
