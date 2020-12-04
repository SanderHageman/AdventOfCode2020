use std::collections::HashMap;

type TParsedVal<'a> = HashMap<&'a str, &'a str>;
type TParsed<'a> = Vec<TParsedVal<'a>>;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> i64 {
    input.iter().filter(|map| is_valid_1(map)).count() as i64
}

fn part_2(input: &TParsed) -> i64 {
    input.iter().filter(|map| is_valid_2(map)).count() as i64
}

fn is_valid_1(map: &TParsedVal) -> bool {
    map.len() >= 8 || (map.len() == 7 && !map.contains_key("cid"))
}

fn is_valid_2(map: &TParsedVal) -> bool {
    if !is_valid_1(map) {
        return false;
    }

    fn in_range(t: i32, lo: i32, hi: i32) -> bool {
        t >= lo && t <= hi
    }

    fn validate_years(map: &TParsedVal) -> bool {
        let byr = map.get(&"byr").unwrap().parse::<i32>().unwrap_or_default();
        let iyr = map.get(&"iyr").unwrap().parse::<i32>().unwrap_or_default();
        let eyr = map.get(&"eyr").unwrap().parse::<i32>().unwrap_or_default();
        in_range(byr, 1920, 2002) && in_range(iyr, 2010, 2020) && in_range(eyr, 2020, 2030)
    }

    fn validate_height(map: &TParsedVal) -> bool {
        let hgt = map.get(&"hgt").unwrap();
        let correct_unit = hgt.ends_with("cm");
        let hgt = hgt
            .trim_end_matches(char::is_alphabetic)
            .parse::<i32>()
            .unwrap_or_default();

        let lim = if correct_unit { (150, 193) } else { (59, 76) };

        in_range(hgt, lim.0, lim.1)
    }

    fn validate_hair_color(map: &TParsedVal) -> bool {
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#([a-fA-F0-9]{6})$").unwrap();
        }
        RE.is_match(map.get(&"hcl").unwrap())
    }

    fn validate_eye_color(map: &TParsedVal) -> bool {
        match map.get(&"ecl").unwrap() {
            &"amb" | &"blu" | &"brn" | &"gry" | &"grn" | &"hzl" | &"oth" => true,
            _ => false,
        }
    }

    fn validate_passport_id(map: &TParsedVal) -> bool {
        use regex::Regex;
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([0-9]{9})$").unwrap();
        }
        RE.is_match(map.get(&"pid").unwrap())
    }

    validate_years(&map)
        && validate_height(&map)
        && validate_hair_color(&map)
        && validate_eye_color(&map)
        && validate_passport_id(&map)
}

fn parse(input: &str) -> TParsed {
    let mut result = TParsed::new();
    let mut passport = TParsedVal::new();

    for line in input.lines() {
        // empty line means next passport
        if line.is_empty() {
            result.push(passport);
            passport = TParsedVal::new();
            continue;
        }

        for entry in line.split_whitespace() {
            let mut a = entry.split(':');
            passport.insert(&a.next().unwrap(), &a.next().unwrap());
        }
    }

    result.push(passport);
    result
}

#[test]
fn show_parse() {
    println!("{:?}", parse(EXAMPLE_INPUT));
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 2)
}

#[test]
fn test_example_2_invalid() {
    let input = parse(EXAMPLE_INPUT_2_INVALID);
    assert_eq!(part_2(&input), 0)
}

#[test]
fn test_example_2_valid() {
    let input = parse(EXAMPLE_INPUT_2_VALID);
    assert_eq!(part_2(&input), 4)
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

#[cfg(test)]
const EXAMPLE_INPUT_2_INVALID: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

#[cfg(test)]
const EXAMPLE_INPUT_2_VALID: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
