// common.rs

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub action: char,
    pub value: i32,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction = s.split_at(1);

        let action: char = instruction.0.chars().next().unwrap();
        let value: i32 = instruction.1.parse().unwrap();

        Ok(Instruction {
            action: action,
            value: value,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_struct() {
        assert_eq!(
            Instruction {
                action: 'N',
                value: 10
            },
            Instruction::from_str("N10").unwrap()
        );

        assert_eq!(
            Instruction {
                action: 'R',
                value: 90
            },
            Instruction::from_str("R90").unwrap()
        );

        assert_eq!(
            Instruction {
                action: 'F',
                value: 7
            },
            Instruction::from_str("F7").unwrap()
        );
    }
}
