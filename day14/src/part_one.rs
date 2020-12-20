// part_one.rs

use std::collections::HashMap;

use crate::common::Program;

fn apply_mask(mask: String, num: usize) -> Option<usize> {
    let mask_on: usize = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
    let mask_off: usize = usize::from_str_radix(&mask.replace("X", "1"), 2).unwrap();

    Some((num | mask_on) & mask_off)
}

pub fn run_and_sum(full_program: &[Program]) -> Option<usize> {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    for program in full_program.iter() {
        for m in program.mem.iter() {
            memory.insert(m.0, apply_mask(program.mask.clone(), m.1)?);
        }
    }

    let result: usize = memory.iter().map(|(_, value)| value).sum();

    Some(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        let full_program: Vec<Program> = vec![Program {
            mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_owned(),
            mem: vec![(8, 11), (7, 101), (8, 0)],
        }];
        assert_eq!(Some(165), run_and_sum(&full_program));
    }
}
