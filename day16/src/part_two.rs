// part_two.rs

use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::{Document, Rule};

// Steps:
// 1. Get the valid tickets
// 2. Generate a row of rules for each column item from valid tickets,
//    and apply the bitwise_and operation for get one row like result
// 3. Generate a matrix with these rows
// 4. Find rows with one only 1 and set 0 in the rest of columns
// 5. Find the fields corresponding to each column
// 6. Multiply the fields of your tickets

// Compute the bit-wise AND of two vectors.
fn bitwise_and(x1: &[u8], x2: &[u8]) -> Option<Vec<u8>> {
    if x1.len() != x2.len() {
        return None;
    }

    let mut result: Vec<u8> = Vec::new();

    for i in 0..x1.len() {
        let value: u8 = x1[i] & x2[i];
        result.push(value);
    }

    Some(result)
}

// class: 0-1 or 4-19
// row: 0-5 or 8-19
// seat: 0-13 or 16-19

// nearby tickets:
// 3,9,18
// 15,1,5
// 5,14,9

// generate a list de valids categories by columns
// index: i=0
//    | class | row | seat |
// 3  |   0   |  1  |   1  |
// 15 |   1   |  1  |   0  |
// 5  |   1   |  1  |   1  |

// index: i=1
//    | class | row | seat |
// 9  |   1   |  1  |   1  |
// 1  |   1   |  1  |   1  |
// 14 |   1   |  1  |   0  |

// index: i=2
//    | class | row | seat |
// 18 |   1   |  1  |   1  |
// 5  |   1   |  1  |   1  |
// 9  |   1   |  1  |   1  |

// apply bitwise_and operation
// index | class | row | seat |
// i=0   |   0   |  1  |   0  |
// i=1   |   1   |  1  |   0  |
// i=2   |   1   |  1  |   1  |

fn generate_matrix(tickets: &[Vec<usize>], rules: &[Rule]) -> Option<Vec<Vec<u8>>> {
    let mut vec_fields: Vec<Vec<u8>> = Vec::new();
    let mut row: Vec<u8> = Vec::new();
    let mut row_result: Vec<u8> = vec![1; tickets[0].len()];

    for i in 0..tickets[0].len() {
        for j in 0..tickets.len() {
            for rule in rules.iter() {
                if rule.range1.contains(&tickets[j][i]) || rule.range2.contains(&tickets[j][i]) {
                    row.push(1);
                } else {
                    row.push(0);
                }
            }
            row_result = bitwise_and(&row_result, &row)?;
            row.clear();
        }
        vec_fields.push(row_result);
        row_result = vec![1; tickets[0].len()];
    }

    Some(vec_fields)
}

fn found_fields(matrix: &mut [Vec<u8>], set_index: HashSet<usize>) -> Option<Vec<Vec<u8>>> {
    let mut point: Option<(usize, usize)> = None;
    let mut set_index: HashSet<usize> = set_index;
    //let mut matrix = matrix.to_vec();

    for (j, row) in matrix.iter().enumerate() {
        if row.iter().sum::<u8>() == 1 && !set_index.contains(&j) {
            let found = row.iter().enumerate().find(|(_, value)| **value == 1);

            if let Some((i, _)) = found {
                point = Some((i, j));
                set_index.insert(j);
                break;
            }
        }
    }

    if point == None {
        return Some(matrix.to_vec());
    }

    for j in 0..matrix.len() {
        if j != point?.1 {
            matrix[j][point?.0] = 0;
        }
    }

    return found_fields(matrix, set_index);
}

fn match_fields(matrix: &[Vec<u8>]) -> Option<Vec<(usize, usize)>> {
    // (i, j) -> i = field, j = index of ticket
    let list: Vec<(usize, usize)> = matrix
        .iter()
        .enumerate()
        .map(|(j, row)| {
            let (i, _) = row
                .iter()
                .enumerate()
                .find(|(_, value)| **value == 1)
                .unwrap();
            (i, j)
        })
        .collect();

    Some(list)
}

pub fn mul_found_fields(document: &Document) -> Option<usize> {
    // 1. Get the valid tockets
    let mut valid_nearby_tickets: Vec<Vec<usize>> = Vec::new();
    for j in 0..document.nearby_tickets.len() {
        let mut found = false;
        for i in 0..document.nearby_tickets[0].len() {
            found = false;
            for rule in document.rules.iter() {
                if rule.range1.contains(&document.nearby_tickets[j][i])
                    || rule.range2.contains(&document.nearby_tickets[j][i])
                {
                    found = true;
                    break;
                }
            }
            if found == false {
                break;
            }
        }
        if found {
            valid_nearby_tickets.push(document.nearby_tickets[j].clone());
        }
    }

    // 2. Generate a row of rules for each column item from valid tickets,
    //    and apply the bitwise_and operation for get one row like result
    // 3. Generate a matrix with these rows
    let mut matrix: Vec<Vec<u8>> = generate_matrix(&valid_nearby_tickets, &document.rules)?;

    // 4. Find rows with one only 1 and set 0 in the rest of columns
    let set_index: HashSet<usize> = HashSet::new();
    matrix = found_fields(&mut matrix, set_index)?;

    // 5. Find the fields corresponding to each column
    let match_list = match_fields(&matrix)?;

    // 6. Multiply the fields of your tickets
    let mut map: HashMap<String, usize> = HashMap::new();
    for (i, j) in match_list.iter() {
        map.insert(document.rules[*i].name.clone(), document.your_ticket[*j]);
    }

    let mut result: usize = 1;
    for (name, value) in map.iter() {
        match name.as_str() {
            "departure location" | "departure station" | "departure platform"
            | "departure track" | "departure date" | "departure time" => result *= value,
            _ => (),
        }
    }
    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::Rule;

    #[test]
    fn test_bitwise_and() {
        let vec1: Vec<u8> = vec![0, 1, 1];
        let vec2: Vec<u8> = vec![1, 1, 0];
        assert_eq!(Some(vec![0, 1, 0]), bitwise_and(&vec1, &vec2));

        let vec1: Vec<u8> = vec![0, 0, 1];
        let vec2: Vec<u8> = vec![1, 1, 0];
        assert_eq!(Some(vec![0, 0, 0]), bitwise_and(&vec1, &vec2));
    }

    #[test]
    fn test_generate_matrix() {
        let document = Document {
            rules: vec![
                Rule {
                    name: "class".to_owned(),
                    range1: 0..=1,
                    range2: 4..=19,
                },
                Rule {
                    name: "row".to_owned(),
                    range1: 0..=5,
                    range2: 8..=19,
                },
                Rule {
                    name: "seat".to_owned(),
                    range1: 0..=13,
                    range2: 16..=19,
                },
            ],
            your_ticket: vec![11, 12, 13],
            nearby_tickets: vec![
                vec![3, 9, 18],
                vec![15, 1, 5],
                vec![5, 14, 9],
            ],
        };
        assert_eq!(Some(vec![
            vec![0, 1, 0],
            vec![1, 1, 0],
            vec![1, 1, 1],
        ]), generate_matrix(&document.nearby_tickets, &document.rules));
    }

    #[test]
    fn test_found_fields() {
        let mut matrix: Vec<Vec<u8>> = vec![
            vec![0, 1, 0],
            vec![1, 1, 0],
            vec![1, 1, 1],
        ];
        let set_index: HashSet<usize> = HashSet::new();

        assert_eq!(Some(vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ]), found_fields(&mut matrix, set_index));
    }

    #[test]
    fn test_match_fields() {
        let matrix = vec![
            vec![0, 1, 0],
            vec![1, 0, 0],
            vec![0, 0, 1],
        ];
        assert_eq!(Some(vec![
            (1, 0),
            (0, 1),
            (2, 2),
        ]), match_fields(&matrix));
    }
}
