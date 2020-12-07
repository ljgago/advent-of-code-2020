// part_one.rs

use crate::common::Form;

pub fn sum_of_answers(forms: &[Form]) -> Option<usize> {
    Some(
        forms
            .iter()
            .fold(0, |acc, form| form.answer_yes + acc))
}
