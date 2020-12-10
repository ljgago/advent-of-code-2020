// part_one.rs

use std::collections::HashSet;

use crate::common::Instruction;

pub fn get_acc_infinite_loop(instructions: &[Instruction]) -> Option<i32> {
    // Pure functions
    let acc_inst = |argument: i32, ip: i32, acc: i32| {
        (ip + 1, acc + argument)
    };

    let jmp_inst = |argument: i32, ip: i32, acc: i32| {
        (ip + argument, acc)
    };

    let nop_inst = |ip: i32, acc: i32| {
        (ip + 1, acc)
    };

    let mut ip: i32 = 0;
    let mut acc_before: i32 = 0;
    let mut acc_actual: i32 = 0;
    let mut ip_set: HashSet<i32> = HashSet::new();

    loop {

        if !ip_set.contains(&ip) {
            ip_set.insert(ip);
        } else {
            return Some(acc_before);
        }

        // println!("(ip: {}, acc: {})", ip, acc_actual);

        acc_before = acc_actual;

        let (ip_, acc_) = match instructions[ip as usize].operation.as_str() {
            "acc" => acc_inst(instructions[ip as usize].argument, ip, acc_actual),
            "jmp" => jmp_inst(instructions[ip as usize].argument, ip, acc_actual),
            _ => nop_inst(ip, acc_actual),
        };

        ip = ip_;
        acc_actual = acc_;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_acc_infinite_loop() {
        let instructions: Vec<Instruction> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4",
            "acc +6"
        ].iter()
         .flat_map(|&x| x.parse())
         .collect();

        assert_eq!(Some(5), get_acc_infinite_loop(&instructions));
    }
}
