// part_two.rs

use crate::common::Form;

pub fn sum_of_answers(forms: &[Form]) -> Option<usize> {
    Some(
        forms
            .iter()
            .fold(0, |acc, form| form.group_answer_yes + acc),
    )
}
