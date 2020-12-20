// part_two.rs

use std::collections::HashMap;

use crate::common::Program;

fn apply_mask(mask: String, num: usize) -> Option<String> {
    let mask_on: usize = usize::from_str_radix(&mask.replace("X", "0"), 2).unwrap();

    let mask_chars = mask.chars();
    let mut result_chars: Vec<char> = format!("{:036b}", num | mask_on).chars().collect();
    for (i, c) in mask_chars.enumerate() {
        if c == 'X' {
            result_chars[i] = 'X';
        }
    }

    Some(result_chars.iter().collect::<String>())
}

// Generate all address
fn address_mask_generator(mask: &str) -> Option<Vec<usize>> {
    let count = mask
        .chars()
        .fold(0, |acc, digit| match digit {
            'X' => acc + 1,
            _ => acc,
        });

    let address_vec: Vec<usize> = (0..(1 << count))
        .map(|x| format!("{:01$b}", x, count).chars().collect::<Vec<char>>())
        .map(|x| {
            let mut address: Vec<char> = mask.clone().chars().collect();
            for digit in x {
                for (i, c) in address.iter().enumerate() {
                    if *c == 'X' {
                        address[i] = digit;
                        break;
                    }
                }
            }
            usize::from_str_radix(&address.iter().collect::<String>(), 2).unwrap()
        })
        .collect();

    Some(address_vec)
}

pub fn run_and_sum(full_program: &[Program]) -> Option<usize> {
    let mut memory: Vec<(String, usize)> = Vec::new();
    for program in full_program.iter() {
        for m in program.mem.iter() {
            let address = apply_mask(program.mask.clone(), m.0)?;
            memory.push((address, m.1));
        }
    }

    let mut memory_map: HashMap<usize, usize> = HashMap::new();
    for (mask_address, value) in memory.iter() {
        let address_vec: Vec<usize> = address_mask_generator(&mask_address)?;
        for address in address_vec.iter() {
            memory_map.insert(*address, *value);
        }
    }

    let sum: usize = memory_map.iter().fold(0, |acc, (_, value)| acc + value);

    Some(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_and_sum() {
        let full_program: Vec<Program> = vec![
            Program {
                mask: "000000000000000000000000000000X1001X".to_owned(),
                mem: vec![(42, 100)],
            },
            Program {
                mask: "00000000000000000000000000000000X0XX".to_owned(),
                mem: vec![(26, 1)],
            },
        ];
        assert_eq!(Some(208), run_and_sum(&full_program));

        let full_program: Vec<Program> = vec![
            Program {
                mask: "00000000000000000000000000000000XX10".to_owned(),
                mem: vec![(2, 5), (5, 2)],
            },
            Program {
                mask: "000000000000000000000000000000001X0X".to_owned(),
                mem: vec![(3, 1), (1, 3)],
            },
        ];
        assert_eq!(Some(30), run_and_sum(&full_program));
    }

    #[test]
    fn test_address_mask_generator() {
        assert_eq!(
            Some(vec![
                0b10010,
                0b10011,
                0b10110,
                0b10111,
            ]),
            address_mask_generator("0000010X1X")
        );

        assert_eq!(
            Some(vec![
                0b010010,
                0b010011,
                0b010110,
                0b010111,
                0b110010,
                0b110011,
                0b110110,
                0b110111,
            ]),
            address_mask_generator("0000X10X1X")
        );
    }
}
