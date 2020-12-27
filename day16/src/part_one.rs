// part_one.rs

use std::collections::HashMap;

use crate::common::Document;

pub fn sum_error_rate(document: &Document) -> Option<usize> {
    let mut map_rules: HashMap<usize, String> = HashMap::new();
    for rule in document.rules.iter() {
        for num in rule.range1.clone() {
            map_rules.insert(num, rule.name.clone());
        }
        for num in rule.range2.clone() {
            map_rules.insert(num, rule.name.clone());
        }
    }

    let mut error_rate: Vec<usize> = Vec::new();
    for nearby_ticket in document.nearby_tickets.iter() {
        for item in nearby_ticket.iter() {
            if !map_rules.contains_key(item) {
                error_rate.push(*item);
            }
        }
    }

    Some(error_rate.iter().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::Rule;

    #[test]
    fn test_error_rate() {
        let document = Document {
            rules: vec![
                Rule {
                    name: "class".to_owned(),
                    range1: 1..=3,
                    range2: 5..=7,
                },
                Rule {
                    name: "row".to_owned(),
                    range1: 6..=11,
                    range2: 33..=44,
                },
                Rule {
                    name: "seat".to_owned(),
                    range1: 13..=40,
                    range2: 45..=50,
                },
            ],
            your_ticket: vec![7, 1, 14],
            nearby_tickets: vec![
                vec![7,3,47],
                vec![40,4,50],
                vec![55,2,20],
                vec![38,6,12],
            ],
        };
        assert_eq!(Some(71), sum_error_rate(&document));
    }
}
