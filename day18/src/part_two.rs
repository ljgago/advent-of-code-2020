// part_two.rs

// Created a Reverse Polish Notation (RPN) with the new rules
fn generate_rpn_token(expression: &str) -> Option<Vec<String>> {
    let mut stack: Vec<char> = Vec::new();
    let mut rpn: Vec<char> = Vec::new();
    let expression = expression.replace(" ", "");

    for c in expression.chars() {
        // println!("rpn: {:?} - stack: {:?} --> {}", rpn, stack, c);
        match c {
            '+' => stack.push('+'),
            '*' => {
                if stack.is_empty() {
                } else if stack[stack.len() - 1] == '+' {
                    loop {
                        if stack.is_empty() {
                            break;
                        }
                        let opt: char = stack.pop().unwrap();
                        if opt != '(' {
                            rpn.push(opt);
                        } else {
                            stack.push(opt);
                            break;
                        }
                    }
                }
                stack.push('*');
            }
            '(' => stack.push('('),
            ')' => loop {
                if stack.is_empty() {
                    break;
                }
                let opt: char = stack.pop().unwrap();
                if opt != '(' {
                    rpn.push(opt);
                } else {
                    break;
                }
            },
            _ => rpn.push(c),
        }
    }

    loop {
        if stack.is_empty() {
            break;
        }
        let opt: char = stack.pop().unwrap();
        if opt != '(' {
            rpn.push(opt);
        }
    }

    let tokens: Vec<String> = rpn.iter().map(|x| x.to_string()).collect();

    Some(tokens)
}

fn calc_tokens(tokens: &[String]) -> Option<usize> {
    let mut tokens: Vec<String> = tokens.to_vec();
    let mut new_tokens: Vec<String> = tokens.clone();
    let mut done = false;

    let result: usize = loop {
        tokens = new_tokens.clone();
        new_tokens.clear();
        for i in 0..tokens.len() {
            if (i + 2) < tokens.len() {
                match tokens[i + 2].as_str() {
                    "+" => {
                        let result = tokens[i].parse::<usize>().unwrap()
                            + tokens[i + 1].parse::<usize>().unwrap();
                        new_tokens.push(result.to_string());
                        if (i + 3) < tokens.len() {
                            let (_, right) = tokens.split_at(i + 3);
                            new_tokens = [new_tokens, right.to_vec()].concat();
                        }
                        break;
                    }
                    "*" => {
                        let result = tokens[i].parse::<usize>().unwrap()
                            * tokens[i + 1].parse::<usize>().unwrap();
                        new_tokens.push(result.to_string());
                        if (i + 3) < tokens.len() {
                            let (_, right) = tokens.split_at(i + 3);
                            new_tokens = [new_tokens, right.to_vec()].concat();
                        }
                        break;
                    }
                    _ => new_tokens.push(tokens[i].clone()),
                }
            } else {
                done = true;
                break;
            }
        }
        if done {
            break tokens[0].parse::<usize>().unwrap();
        }
    };

    Some(result)
}

pub fn sum_operations(expressions: &[String]) -> Option<usize> {
    let sum: usize = expressions
        .iter()
        .map(|expression| {
            let tokens = generate_rpn_token(&expression).unwrap();
            calc_tokens(&tokens).unwrap()
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_rpn_tokens() {
        let expressions = "1 + 2 * 3 + 4 * 5 + 6";
        let rpn: Vec<String> = vec!['1', '2', '+', '3', '4', '+', '*', '5', '6', '+', '*']
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Some(rpn), generate_rpn_token(expressions));

        let expressions = "1 + (2 * 3) + (4 * (5 + 6))";
        let rpn: Vec<String> = vec!['1', '2', '3', '*', '4', '5', '6', '+', '*', '+', '+']
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Some(rpn), generate_rpn_token(expressions));

        let expressions = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let rpn: Vec<String> = vec![
            '2', '4', '+', '9', '*', '6', '9', '+', '8', '6', '+', '*', '6', '+', '*', '2', '4',
            '+', '+', '2', '*',
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(Some(rpn), generate_rpn_token(expressions));
    }

    #[test]
    fn test_calc_tokens() {
        let rpn: Vec<String> = vec!['1', '2', '+', '3', '4', '+', '*', '5', '6', '+', '*']
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Some(231), calc_tokens(&rpn));

        let rpn: Vec<String> = vec!['1', '2', '3', '*', '4', '5', '6', '+', '*', '+', '+']
            .iter()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Some(51), calc_tokens(&rpn));

        let rpn: Vec<String> = vec![
            '2', '4', '+', '9', '*', '6', '9', '+', '8', '6', '+', '*', '6', '+', '*', '2', '4',
            '+', '+', '2', '*',
        ]
        .iter()
        .map(|x| x.to_string())
        .collect();
        assert_eq!(Some(23340), calc_tokens(&rpn));
    }

    #[test]
    fn test_sum_operations() {
        let expressions = vec!["1 + 2 * 3 + 4 * 5 + 6".to_owned()];
        assert_eq!(Some(231), sum_operations(&expressions));

        let expressions = vec!["1 + (2 * 3) + (4 * (5 + 6))".to_owned()];
        assert_eq!(Some(51), sum_operations(&expressions));

        let expressions = vec!["2 * 3 + (4 * 5)".to_owned()];
        assert_eq!(Some(46), sum_operations(&expressions));

        let expressions = vec!["5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()];
        assert_eq!(Some(1445), sum_operations(&expressions));

        let expressions = vec!["5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_owned()];
        assert_eq!(Some(669060), sum_operations(&expressions));

        let expressions = vec!["((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_owned()];
        assert_eq!(Some(23340), sum_operations(&expressions));
    }
}
