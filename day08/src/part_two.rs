// part_two.rs

use std::collections::HashSet;

use crate::common::Instruction;

pub fn search_fix_operation(instructions: &[Instruction]) -> Option<(i32, i32)> {
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.operation.as_str() == "jmp" {
            let inst = Instruction {
                operation: "nop".to_owned(),
                argument: instruction.argument,
            };
            let mut instructions_vec: Vec<Instruction> = instructions.to_vec();
            instructions_vec[index] = inst.clone();

            if let Some(acc) = run_program(&instructions_vec) {
                // instruction line = index + 1
                return Some((1 + index as i32, acc))
            }

        }

        if instruction.operation.as_str() == "nop" && instruction.argument != 0 {
            let inst = Instruction {
                operation: "nop".to_owned(),
                argument: instruction.argument,
            };
            let mut instructions_vec: Vec<Instruction> = instructions.to_vec();
            instructions_vec[index] = inst.clone();

            if let Some(acc) = run_program(&instructions_vec) {
                // instruction line = index + 1
                return Some((1 + index as i32, acc))
            }
        }
    }

    None
}

fn run_program(instructions: &[Instruction]) -> Option<i32> {
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
    let mut acc_actual: i32 = 0;
    let mut ip_set: HashSet<i32> = HashSet::new();

    loop {

        if !ip_set.contains(&ip) {
            ip_set.insert(ip);
        } else {
            return None;
        }

        if ip as usize == instructions.len() {
            return Some(acc_actual)
        }
        // println!("(ip: {}, acc: {})", ip, acc_actual);

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
    fn test_infinite_loop() {
        let instructions_bad: Vec<Instruction> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "jmp -4", // <-- bad operation
            "acc +6"
        ].iter()
         .flat_map(|&x| x.parse())
         .collect();
        assert_eq!(None, run_program(&instructions_bad));

        let instructions_ok: Vec<Instruction> = vec![
            "nop +0",
            "acc +1",
            "jmp +4",
            "acc +3",
            "jmp -3",
            "acc -99",
            "acc +1",
            "nop -4", // <-- changed from `jmp` to `nop`
            "acc +6"
        ].iter()
         .flat_map(|&x| x.parse())
         .collect();
        assert_eq!(Some(8), run_program(&instructions_ok));
    }

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
            "jmp -4", // <-- line 8
            "acc +6"
        ].iter()
         .flat_map(|&x| x.parse())
         .collect();

        assert_eq!(Some((8, 8)), search_fix_operation(&instructions));
    }
}
