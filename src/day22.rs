use itertools::zip;
use std::collections::HashSet;
type TParsed = (TParsedSub, TParsedSub);
type TParsedSub = Vec<u8>;

pub fn day(input: String) -> (usize, usize) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1((p1, p2): &TParsed) -> usize {
    game(p1, p2, false).1
}

fn part_2((p1, p2): &TParsed) -> usize {
    game(p1, p2, true).1
}

fn game(p1: &[u8], p2: &[u8], recursive: bool) -> (bool, usize) {
    let mut deck_p1 = Vec::from(p1);
    let mut deck_p2 = Vec::from(p2);

    let mut seen = HashSet::new();

    fn get_score(deck: &Vec<u8>) -> usize {
        deck.iter()
            .rev()
            .enumerate()
            .map(|(i, &card)| (i + 1) * (card as usize))
            .sum()
    }

    loop {
        let deck_p1_read = deck_p1.clone();
        let deck_p2_read = deck_p2.clone();

        for (card_p1, card_p2) in zip(&deck_p1_read, &deck_p2_read) {
            if recursive && !seen.insert((deck_p1.clone(), deck_p2.clone())) {
                return (true, get_score(&deck_p1));
            }

            assert_eq!(&deck_p1.remove(0), card_p1);
            assert_eq!(&deck_p2.remove(0), card_p2);

            let mut is_winner_p1 = card_p1 > card_p2;

            if recursive
                && (*card_p1 as usize) <= deck_p1.len()
                && (*card_p2 as usize) <= deck_p2.len()
            {
                is_winner_p1 = game(
                    &deck_p1[..(*card_p1 as usize)],
                    &deck_p2[..(*card_p2 as usize)],
                    true,
                )
                .0;
            }

            if is_winner_p1 {
                deck_p1.push(*card_p1);
                deck_p1.push(*card_p2);
            } else {
                deck_p2.push(*card_p2);
                deck_p2.push(*card_p1);
            }
        }

        if deck_p1.is_empty() || deck_p2.is_empty() {
            break;
        }
    }

    // one of them is empty ¯\_(ツ)_/¯
    let score = get_score(&deck_p1) + get_score(&deck_p2);
    (deck_p2.is_empty(), score)
}

fn parse(input: &str) -> TParsed {
    let mut p1 = vec![];
    let mut p2 = vec![];

    let mut current_vec = &mut p1;
    for line in input.lines() {
        if line.is_empty() || line.starts_with("Player 1:") {
            continue;
        } else if line.starts_with("Player 2:") {
            current_vec = &mut p2;
            continue;
        }

        current_vec.push(line.parse().unwrap());
    }

    (p1, p2)
}

#[test]
fn show_parse() {
    let input = parse(EXAMPLE_INPUT);
    println!("{:?}", input);
}

#[test]
fn test_example_1() {
    let input = parse(EXAMPLE_INPUT);
    assert_eq!(part_1(&input), 306)
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(EXAMPLE_INPUT)), 291);
    assert_eq!(part_2(&parse(EXAMPLE_INPUT_INFINITE)), 105);
}

#[cfg(test)]
const EXAMPLE_INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

#[cfg(test)]
const EXAMPLE_INPUT_INFINITE: &str = "\
Player 1:
43
19

Player 2:
2
29
14";
