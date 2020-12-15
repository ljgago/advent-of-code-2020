// part_two.rs

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

fn rotate(line: (i32, i32), degrees: i32) -> (i32, i32) {
    match degrees {
        90 | -270 => (-line.1, line.0),
        180 | -180 => (-line.0, -line.1),
        270 | -90 => (line.1, -line.0),
        _ => line,
    }
}

pub fn get_distance(instructions: &[Instruction]) -> Option<i32> {
    let mut ship: (i32, i32) = (0, 0);
    let mut waypoint: (i32, i32) = (10, 1);

    for instruction in instructions.iter() {
        ship = match instruction.action {
            'F' => (
                ship.0 + waypoint.0 * instruction.value,
                ship.1 + waypoint.1 * instruction.value,
            ),
            _ => ship,
        };

        waypoint = match instruction.action {
            'N' => (waypoint.0, waypoint.1 + instruction.value),
            'S' => (waypoint.0, waypoint.1 - instruction.value),
            'E' => (waypoint.0 + instruction.value, waypoint.1),
            'W' => (waypoint.0 - instruction.value, waypoint.1),
            'R' => rotate(waypoint, -instruction.value),
            'L' => rotate(waypoint, instruction.value),
            _ => waypoint,
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
        assert_eq!(Some(286), get_distance(&data));
    }
}
