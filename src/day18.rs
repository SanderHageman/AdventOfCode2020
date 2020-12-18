type TParsed = Vec<TParsedSub>;
type TParsedSub = Vec<Expr>;

pub fn day(input: String) -> (i64, i64) {
    let parsed_input = parse(&input);
    (part_1(&parsed_input), part_2(&parsed_input))
}

fn part_1(input: &TParsed) -> i64 {
    input.iter().map(|line| get_scope_value(&line)).sum()
}

fn part_2(input: &TParsed) -> i64 {
    input.iter().map(|line| get_scope_value_pt2(&line)).sum()
}

fn get_scope_value(scope: &Vec<Expr>) -> i64 {
    calculate(scope, get_scope_value)
}

fn get_scope_value_pt2(scope: &Vec<Expr>) -> i64 {
    // > 3 means it could be more than A + B so we must check
    if scope.len() > 3 {
        let mut mod_scope = vec![];
        let mut carry_opt = None;

        for expr in scope {
            if let Some(carry) = carry_opt {
                let add_expr = vec![carry, Expr::Add, expr.clone()];
                mod_scope.push(Expr::N(get_scope_value_pt2(&add_expr)));
                carry_opt = None;
            } else {
                match expr {
                    Expr::Add => carry_opt = Some(mod_scope.pop().unwrap()),
                    _ => mod_scope.push(expr.clone()),
                }
            }
        }
        calculate(&mod_scope, get_scope_value_pt2)
    } else {
        calculate(&scope, get_scope_value_pt2)
    }
}

fn calculate<F>(scope: &Vec<Expr>, fn_recurse: F) -> i64
where
    F: Fn(&Vec<Expr>) -> i64,
{
    let mut result = 0;
    let mut modifier = None;

    let mut apply_modifier = |n: i64, modifier: &Option<Expr>| match modifier {
        Some(Expr::Add) => result += n,
        Some(Expr::Mult) => result *= n,
        Some(_) => panic!("Uncovered modifier {:?}", modifier),
        None => result = n,
    };

    for expr in scope {
        match expr {
            Expr::N(n) => apply_modifier(*n, &modifier),
            Expr::Add => modifier = Some(Expr::Add),
            Expr::Mult => modifier = Some(Expr::Mult),
            Expr::Scope(other_scope) => {
                let scoped_result = fn_recurse(other_scope);
                apply_modifier(scoped_result, &modifier);
            }
            _ => panic!("Uncovered expression {:?}", expr),
        }
    }

    result
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    N(i64),
    Add,
    Mult,
    Scope(Vec<Expr>),
    StartScope,
    EndScope,
}

#[test]
fn test_example_1() {
    assert_eq!(part_1(&parse(&EX)), 51);
    assert_eq!(part_1(&parse(&EX1)), 26);
    assert_eq!(part_1(&parse(&EX2)), 437);
    assert_eq!(part_1(&parse(&EX3)), 12240);
    assert_eq!(part_1(&parse(&EX4)), 13632);
}

#[test]
fn test_example_2() {
    assert_eq!(part_2(&parse(&EX)), 51);
    assert_eq!(part_2(&parse(&EX1)), 46);
    assert_eq!(part_2(&parse(&EX2)), 1445);
    assert_eq!(part_2(&parse(&EX3)), 669060);
    assert_eq!(part_2(&parse(&EX4)), 23340);
}

#[cfg(test)]
const EX: &str = "1 + (2 * 3) + (4 * (5 + 6))";

#[cfg(test)]
const EX1: &str = "2 * 3 + (4 * 5)";

#[cfg(test)]
const EX2: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";

#[cfg(test)]
const EX3: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";

#[cfg(test)]
const EX4: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

fn parse(input: &str) -> TParsed {
    let mut result = TParsed::new();

    for line in input.lines() {
        let mut stack = vec![vec![]];
        let mut current_scope = vec![];

        for c in line.chars().filter(|c| !c.is_whitespace()) {
            let expr = match c {
                '(' => Expr::StartScope,
                ')' => Expr::EndScope,
                '*' => Expr::Mult,
                '+' => Expr::Add,
                _ => Expr::N(c.to_digit(10).unwrap() as i64),
            };

            match expr {
                Expr::StartScope => {
                    stack.push(current_scope);
                    current_scope = vec![];
                }
                Expr::EndScope => {
                    let scope = Expr::Scope(current_scope);
                    current_scope = stack.pop().unwrap();
                    current_scope.push(scope);
                }
                _ => current_scope.push(expr),
            }
        }

        result.push(current_scope);
    }

    result
}
