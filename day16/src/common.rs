// common.rs

use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    pub name: String,
    pub range1: RangeInclusive<usize>,
    pub range2: RangeInclusive<usize>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Document {
    pub rules: Vec<Rule>,
    pub your_ticket: Vec<usize>,
    pub nearby_tickets: Vec<Vec<usize>>,
}

fn parse_custom(string: &str) -> Option<Document> {
    let s: Vec<&str> = string
        .trim()
        .split("\n\n")
        .collect();

    let rules: Vec<Rule> = s[0]
        .lines()
        .map(|str_rule| {
            let vec_rule: Vec<&str> = str_rule.trim().split(": ").collect();
            let name: String = vec_rule[0].trim().to_owned();

            let vec_range: Vec<&str> = vec_rule[1].trim().split(" or ").collect();
            let vec_range1: Vec<&str> = vec_range[0].trim().split("-").collect();
            let vec_range2: Vec<&str> = vec_range[1].trim().split("-").collect();

            let range1: RangeInclusive<usize> = vec_range1[0].parse::<usize>().unwrap()..=vec_range1[1].parse::<usize>().unwrap();
            let range2: RangeInclusive<usize> = vec_range2[0].parse::<usize>().unwrap()..=vec_range2[1].parse::<usize>().unwrap();

            Rule {
                name: name,
                range1: range1,
                range2: range2,
            }
        })
        .collect();

    let vec_your_ticket: Vec<&str> = s[1]
        .trim()
        .lines()
        .collect();
    let your_ticket: Vec<usize> = vec_your_ticket[1]
        .trim()
        .split(",")
        .map(|ticket| ticket.parse().unwrap())
        .collect();

    let vec_nearby_tickets: Vec<&str> = s[2]
        .trim()
        .splitn(2, "\n")
        .collect();
    let nearby_tickets: Vec<Vec<usize>> = vec_nearby_tickets[1]
        .trim()
        .lines()
        .map(|str_nearby_tikets| {
            let vec_tickets: Vec<usize> = str_nearby_tikets
                .trim()
                .split(",")
                .map(|tiket| tiket.parse().unwrap())
                .collect();
            vec_tickets
        })
        .collect();

    Some(Document {
        rules: rules,
        your_ticket: your_ticket,
        nearby_tickets: nearby_tickets,
    })

}

pub fn read_custom(filename: &str) -> Option<Document> {
    parse_custom(&std::fs::read_to_string(filename).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct_document() {
        let data =
            r#"class: 1-3 or 5-7
            row: 6-11 or 33-44
            seat: 13-40 or 45-50

            your ticket:
            7,1,14

            nearby tickets:
            7,3,47
            40,4,50
            55,2,20
            38,6,12"#;

        assert_eq!(Some(Document{
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
            ]

        }), parse_custom(&data));
    }
}
