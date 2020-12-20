// common.rs

use std::convert::Infallible;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub mask: String,
    pub mem: Vec<(usize, usize)>,
}

impl FromStr for Program {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split("mem[").map(|x| x.trim()).collect();

        let mask: String = data[0].to_owned();
        let mem: Vec<(usize, usize)> = data
            .clone()
            .drain(1..)
            .map(|mem_str| {
                let m: Vec<&str> = mem_str.split("] = ").map(|x| x.trim()).collect();
                (m[0].parse().unwrap(), m[1].parse().unwrap())
            })
            .collect();

        Ok(Program {
            mask: mask,
            mem: mem,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_program_struct() {
        assert_eq!(
            Program {
                mask: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_owned(),
                mem: vec![(8, 11), (7, 101), (8, 0)],
            },
            Program::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0").unwrap()
        );


        assert_eq!(
            Program {
                mask: "0111110X0X0011X1010X1X110101X1XX0001".to_owned(),
                mem: vec![(123, 234), (4564, 90), (33, 22)],
            },
            Program::from_str("0111110X0X0011X1010X1X110101X1XX0001\nmem[123] = 234\nmem[4564] = 90\nmem[33] = 22").unwrap()
        );
    }
}
