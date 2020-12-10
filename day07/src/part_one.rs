// part_one.rs

use std::collections::HashMap;

pub fn check_contain_color_bags(root: &str, target: &str, bags_hash: &HashMap<String, Vec<(String, usize)>>) -> bool {
    if *root == *target {
        return true;
    }
    if root == "no other" {
        return false;
    }

    let found: bool = bags_hash.get(root)
        .unwrap()
        .iter()
        .map(|(name, _)| {
            check_contain_color_bags(&name.as_str(), target, bags_hash)
        })
        .any(|x| x == true);

    return found;
}

pub fn count_color_bags(target: &str, bags_hash: &HashMap<String, Vec<(String, usize)>>) -> Option<usize> {
    Some(bags_hash
        .iter()
        .map(|(bag, _)| {
            if bag == target {
                false
            } else {
                check_contain_color_bags(bag, target, bags_hash)
            }
        })
        .filter(|&x| x)
        .count()
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::*;

    #[test]
    fn test_check_contain_color_bags() {
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

        assert_eq!(true, check_contain_color_bags("light red", "shiny gold", &bags_map));
        assert_eq!(true, check_contain_color_bags("shiny gold", "shiny gold", &bags_map));
        assert_eq!(true, check_contain_color_bags("muted yellow", "shiny gold", &bags_map));
        assert_eq!(false, check_contain_color_bags("faded blue", "shiny gold", &bags_map));
    }

    #[test]
    fn test_count_color_bags() {
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

        assert_eq!(Some(4), count_color_bags("shiny gold", &bags_map));
    }
}
