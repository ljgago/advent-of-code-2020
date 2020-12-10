// common.rs

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub operation: String,
    pub argument: i32,
}

impl FromStr for Instruction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instruction: Vec<&str> = s.split(" ").collect::<Vec<&str>>();

        let operation: String = instruction[0].to_owned();
        let argument: i32 = instruction[1].parse().unwrap();

        Ok(Instruction {
            operation: operation,
            argument: argument,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_struct_instruction() {
        assert_eq!(
            Ok(Instruction {
                operation: "nop".to_owned(),
                argument: 0
            }),
            Instruction::from_str("nop +0")
        );

        assert_eq!(
            Ok(Instruction {
                operation: "jmp".to_owned(),
                argument: 4
            }),
            Instruction::from_str("jmp +4")
        );

        assert_eq!(
            Ok(Instruction {
                operation: "acc".to_owned(),
                argument: -10
            }),
            Instruction::from_str("acc -10")
        );
    }
}
