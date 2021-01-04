// part_one.rs

fn calc(expression: &str) -> usize {
    let operation = |a: usize, b: usize, opt: char| -> usize {
        match opt {
            '+' => a + b,
            '*' => a * b,
            _ => 0,
        }
    };

    let mut a: usize = 0;
    let mut operator: char = ' ';
    let mut parenthesis = false;
    let mut count_parenthesis: i32 = 0;
    let mut new_expression: Vec<char> = Vec::new();
    for c in expression.chars() {
        if c == '(' && parenthesis == false {
            parenthesis = true;
        } else if parenthesis {
            if c == '(' {
                count_parenthesis += 1;
            }

            if c == ')' {
                if count_parenthesis == 0 {
                    if a == 0 {
                        a = calc(&new_expression.iter().collect::<String>());
                    } else {
                        let b = calc(&new_expression.iter().collect::<String>());
                        a = operation(a, b, operator);
                    }
                    new_expression.clear();
                    parenthesis = false;
                    continue;
                }
                if count_parenthesis > 0 {
                    count_parenthesis -= 1;
                }
            }
            new_expression.push(c);
        } else {
            match c {
                '+' => operator = '+',
                '*' => operator = '*',
                '(' => (),
                ')' => (),
                _ => {
                    if a == 0 {
                        a = c.to_digit(10).unwrap() as usize;
                    } else {
                        let b = c.to_digit(10).unwrap() as usize;
                        a = operation(a, b, operator);
                    }
                }
            }
        }
    }
    return a;
}

pub fn sum_operations(expressions: &[String]) -> Option<usize> {
    let sum: usize = expressions
        .iter()
        .map(|expression| {
            let expression = expression.replace(" ", "");
            calc(&expression)
        })
        .sum();
    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_operations() {
        let expressions = vec!["1 + 2 * 3 + 4 * 5 + 6".to_owned()];
        assert_eq!(Some(71), sum_operations(&expressions));

        let expressions = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_owned()];
        assert_eq!(Some(51), sum_operations(&expressions));

        let expressions = vec!["2 * 3 + (4 * 5)".to_owned()];
        assert_eq!(Some(26), sum_operations(&expressions));

        let expressions = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()];
        assert_eq!(Some(437), sum_operations(&expressions));

        let expressions = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()];
        assert_eq!(Some(12240), sum_operations(&expressions));

        let expressions = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()];
        assert_eq!(Some(13632), sum_operations(&expressions));
    }
}
