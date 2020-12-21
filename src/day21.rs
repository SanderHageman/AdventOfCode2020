use std::collections::{HashMap, HashSet};

type TParsed = HashMap<TP1, TP2>;
type TP1 = String;
type TP2 = Vec<Vec<String>>;

pub fn day(input: String) -> (usize, String) {
    let parsed_input = parse(&input);
    let (result1, allergens_to_ingredient) = part_1(&parsed_input);
    (result1, part_2(allergens_to_ingredient))
}

fn part_1(allergens_to_ingedients: &TParsed) -> (usize, HashMap<String, String>) {
    let mut claims = HashMap::new();
    let mut all_ingredients = HashSet::new();

    for (allergen, ingredients) in allergens_to_ingedients {
        let mut common = HashMap::new();
        ingredients.iter().for_each(|ingredients| {
            for ingredient in ingredients {
                *common.entry(ingredient).or_insert(0) += 1;
            }
            all_ingredients.insert(ingredients.clone());
        });

        let common = common
            .iter()
            .filter(|(_, v)| *v == &ingredients.len())
            .map(|(k, _)| (*k).to_owned())
            .collect::<Vec<_>>();

        claims.insert(allergen.to_owned(), common);
    }

    let mut ingredients = vec![];
    all_ingredients.drain().for_each(|v| ingredients.extend(v));
    let all_ingredients = ingredients;

    loop {
        let claims_read = claims.clone();
        let mut changed = false;

        for (allergen, ingredients) in claims_read.iter().filter(|(_, v)| v.len() == 1) {
            claims
                .iter_mut()
                .filter(|(k, _)| k != &allergen)
                .for_each(|(_, v)| {
                    if let Some(i) = v.iter().position(|c| c == &ingredients[0]) {
                        v.remove(i);
                        changed = true;
                    }
                });
        }

        if !changed {
            break;
        }
    }

    let claims: HashMap<String, String> = claims
        .drain()
        .map(|(k, mut v)| (k, v.pop().unwrap()))
        .collect();
    let claimed = claims.iter().map(|(_, v)| v.to_owned()).collect::<Vec<_>>();

    let rest = all_ingredients
        .iter()
        .filter(|i| !claimed.contains(i))
        .count();

    (rest, claims)
}

fn part_2(allergens_to_ingredient: HashMap<String, String>) -> String {
    let mut sorted_allergens = allergens_to_ingredient
        .iter()
        .map(|(k, _)| k.to_owned())
        .collect::<Vec<_>>();
    sorted_allergens.sort_unstable();

    let mut result = String::new();
    for allergen in &sorted_allergens {
        result.push_str(allergens_to_ingredient.get(allergen).unwrap());
        result.push(',');
    }

    result.pop();
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
    assert_eq!(part_1(&input).0, 5)
}

#[test]
fn test_example_2() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_2(part_1(&input).1), "mxmxvkd,sqjhc,fvjkl")
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

fn parse(input: &str) -> TParsed {
    use regex::Regex;
    lazy_static! {
        static ref ALLERGENS: Regex = Regex::new(r"contains .*\)$").unwrap();
    }

    let mut result = TParsed::new();

    for line in input.lines() {
        let allergens_cap = ALLERGENS.captures(line).unwrap()[0].to_owned();
        let line = line
            .strip_suffix(&allergens_cap)
            .unwrap()
            .strip_suffix(" (")
            .unwrap();

        let ingredients = line
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        let allergens_cap = allergens_cap
            .strip_prefix("contains ")
            .unwrap()
            .strip_suffix(')')
            .unwrap();

        let allergens = allergens_cap
            .split_whitespace()
            .map(|s| s.strip_suffix(',').unwrap_or(s).to_owned())
            .collect::<Vec<_>>();

        for allergen in allergens {
            result
                .entry(allergen)
                .or_insert(vec![])
                .push(ingredients.clone());
        }
    }

    result
}
