// part_one.rs

use std::collections::HashMap;

use crate::common::Instruction;

// left-rotation = counter-clockwise -> positive rotation
// +90, +180, +270 <-> +pi/2, +pi, +3*pi/2
//
// right-rotation = clockwise -> negative rotation
// -90, -180, -270 <-> -pi/2, -pi, -3*pi/2
//
// Rotation Matrix:
// | cos(a)  -sen(a) |
// |                 |
// | sen(a)   cos(a) |
//
// | x' |   | cos(a)  -sen(a) | | x |
// |    | = |                 | |   |
// | y' |   | sen(a)   cos(a) | | y |
//
// 90       180      270
// | 0 -1|  |-1  0|  | 0  1|
// | 1  0|  | 0 -1|  |-1  0|
//
// -90      -180     -270
// | 0  1|  |-1  0|  | 0 -1|
// |-1  0|  | 0 -1|  | 1  0|
//

fn rotate(letter: char, degrees: i32) -> char {
    let line_to_letter: HashMap<(i32, i32), char> = [
        ((1, 0), 'E'),
        ((0, 1), 'N'),
        ((-1, 0), 'W'),
        ((0, -1), 'S'),
    ].iter().cloned().collect();
    let letter_to_line: HashMap<char, (i32, i32)> = [
        ('E', (1, 0)),
        ('N', (0, 1)),
        ('W', (-1, 0)),
        ('S', (0, -1)),
    ].iter().cloned().collect();

    let line = *letter_to_line.get(&letter).unwrap();
    let rotate_line = match degrees {
        90 | -270 => (-line.1, line.0),
        180 | -180 => (-line.0, -line.1),
        270 | -90 => (line.1, -line.0),
        _ => line,
    };

    *line_to_letter.get(&rotate_line).unwrap()
}

pub fn get_distance(instructions: &[Instruction]) -> Option<i32> {
    let mut ship: (i32, i32) = (0, 0);
    let mut last_direction: char = 'E';

    for instruction in instructions.iter() {
        let direction = match instruction.action {
            'F' => last_direction,
            _ => instruction.action,
        };

        ship = match direction {
            'N' => (ship.0, ship.1 + instruction.value),
            'S' => (ship.0, ship.1 - instruction.value),
            'E' => (ship.0 + instruction.value, ship.1),
            'W' => (ship.0 - instruction.value, ship.1),
            _ => ship,
        };

        last_direction = match direction {
            'L' => rotate(last_direction, instruction.value),
            'R' => rotate(last_direction, -instruction.value),
            _ => last_direction,
        };
    }
    Some(ship.0.abs() + ship.1.abs())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_get_distance() {
        let data: Vec<Instruction> = vec![
            Instruction::from_str("F10").unwrap(),
            Instruction::from_str("N3").unwrap(),
            Instruction::from_str("F7").unwrap(),
            Instruction::from_str("R90").unwrap(),
            Instruction::from_str("F11").unwrap(),
        ];
        assert_eq!(Some(25), get_distance(&data));
    }
}
