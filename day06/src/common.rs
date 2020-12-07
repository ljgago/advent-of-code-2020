// common.rs

use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Form {
    pub people: usize,
    pub answer_yes: usize,
    pub group_answer_yes: usize,
}

impl FromStr for Form {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let people: usize = s.split("\n").collect::<Vec<&str>>().len();

        //let mut answers_hash = HashSet::new();
        let mut answers_hash = HashMap::new();
        let answers = s.replace("\n", "");

        for answer in answers.chars() {
            if !answers_hash.contains_key(&answer) {
                answers_hash.insert(answer, 1);
            } else {
                // to part two
                // change the value of HashMap
                if let Some(value) = answers_hash.get_mut(&answer) {
                    *value = *value + 1;
                }
            }
        }
        let answer_yes: usize = answers_hash.len();

        // to part two
        let group_answer_yes: usize = if people == 1 {
            answer_yes
        } else {
            answers_hash.iter().fold(
                0,
                |acc, (_, value)| if *value == people { acc + 1 } else { acc },
            )
        };

        Ok(Form {
            people: people,
            answer_yes: answer_yes,
            group_answer_yes: group_answer_yes,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_form() {
        let form = Form::from_str("abc").unwrap();
        assert_eq!(1, form.people);
        assert_eq!(3, form.answer_yes);
        assert_eq!(3, form.group_answer_yes);

        let form = Form::from_str("a\nb\nc").unwrap();
        assert_eq!(3, form.people);
        assert_eq!(3, form.answer_yes);
        assert_eq!(0, form.group_answer_yes);

        let form = Form::from_str("ab\nac").unwrap();
        assert_eq!(2, form.people);
        assert_eq!(3, form.answer_yes);
        assert_eq!(1, form.group_answer_yes);

        let form = Form::from_str("a\na\na\na").unwrap();
        assert_eq!(4, form.people);
        assert_eq!(1, form.answer_yes);
        assert_eq!(1, form.group_answer_yes);

        let form = Form::from_str("b").unwrap();
        assert_eq!(1, form.people);
        assert_eq!(1, form.answer_yes);
        assert_eq!(1, form.group_answer_yes);

        let form = Form::from_str("vs\nc\nc\nc").unwrap();
        assert_eq!(4, form.people);
        assert_eq!(3, form.answer_yes);
        assert_eq!(0, form.group_answer_yes);

        let form = Form::from_str("sr\nrs\nrs\nsr\nrs").unwrap();
        assert_eq!(5, form.people);
        assert_eq!(2, form.answer_yes);
        assert_eq!(2, form.group_answer_yes);
    }
}
