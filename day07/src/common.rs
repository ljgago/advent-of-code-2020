// common.rs

use std::collections::HashMap;

pub fn hash_map_of_bags(rules: &[String]) -> HashMap<String, Vec<(String, usize)>> {
    let mut bags_map: HashMap<String, Vec<(String, usize)>> = HashMap::new();

    for rule in rules.iter() {
        let line_rule: Vec<String> = rule
            .replace("bags contain", "|")
            .as_str()
            .replace("bags,", "|")
            .as_str()
            .replace("bag,", "|")
            .as_str()
            .replace("bags.", "")
            .as_str()
            .replace("bag.", "")
            .as_str()
            .trim()
            .split("|")
            .map(|x| x.trim().to_owned())
            .collect();

        bags_map.insert(
            line_rule.clone()[0].to_owned(),
            line_rule.clone()[1..].iter().fold(vec![], |mut acc, bag| {
                if bag == "no other" {
                    acc.push((bag.to_owned(), 0));
                } else {
                    let bag_into: Vec<&str> = bag.splitn(2, " ").collect();
                    acc.push((
                        bag_into[1].to_owned(),
                        bag_into[0].parse::<usize>().unwrap(),
                    ));
                }
                acc
            }),
        );
    }
    bags_map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_map_of_bags() {
        let rules = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_owned(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_owned(),
            "bright white bags contain 1 shiny gold bag.".to_owned(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_owned(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_owned(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_owned(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_owned(),
            "faded blue bags contain no other bags.".to_owned(),
            "dotted black bags contain no other bags.".to_owned(),
        ];
        let bags_map = hash_map_of_bags(&rules);

        assert_eq!(
            Some(&vec![
                ("bright white".to_owned(), 1),
                ("muted yellow".to_owned(), 2)
            ]),
            bags_map.get("light red")
        );

        assert_eq!(
            Some(&vec![
                ("bright white".to_owned(), 3),
                ("muted yellow".to_owned(), 4)
            ]),
            bags_map.get("dark orange")
        );

        assert_eq!(
            Some(&vec![
                ("faded blue".to_owned(), 5),
                ("dotted black".to_owned(), 6)
            ]),
            bags_map.get("vibrant plum")
        );

        assert_eq!(
            Some(&vec![("no other".to_owned(), 0)]),
            bags_map.get("faded blue")
        );
    }
}
