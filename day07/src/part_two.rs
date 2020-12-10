// part_two.rs

use std::collections::HashMap;

pub fn all_individual_bags(root: &str, bags_map: &HashMap<String, Vec<(String, usize)>>) -> Option<usize> {

    let sum: usize = bags_map
        .get(root)?
        .iter()
        .fold(0, |acc, (bag, quantity)| {
            if *quantity != 0 {
                acc + quantity + quantity * all_individual_bags(bag, bags_map).unwrap()
            } else {
                acc
            }
        });

    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::*;

    #[test]
    fn test_add() {
        let rules = vec![
            "shiny gold bags contain 2 dark red bags.".to_owned(),
            "dark red bags contain 2 dark orange bags.".to_owned(),
            "dark orange bags contain 2 dark yellow bags.".to_owned(),
            "dark yellow bags contain 2 dark green bags.".to_owned(),
            "dark green bags contain 2 dark blue bags.".to_owned(),
            "dark blue bags contain 2 dark violet bags.".to_owned(),
            "dark violet bags contain no other bags.".to_owned()
        ];
        let bags_map = hash_map_of_bags(&rules);

        assert_eq!(Some(126), all_individual_bags("shiny gold", &bags_map));
    }
}
