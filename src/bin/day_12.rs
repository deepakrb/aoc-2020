use lazy_static::lazy_static;
use std::fs;

lazy_static! {
    static ref DIRECTIONS: Vec<(i32, i32)> = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];
}

fn main() {
    let data = fs::read_to_string("inputs/day_12.txt").expect("Unable to read file");

    println!("Part 1: {}", part_1(parse_input(data.as_str())));
    println!("Part 2: {}", part_2(parse_input(data.as_str())));
}

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let c = l.chars().collect::<Vec<char>>();
            (
                c[0],
                c[1..].iter().collect::<String>().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

/*
--- Day 12: Rain Risk ---

Your ferry made decent progress toward the island, but the storm came in faster than anyone expected. The ferry needs to take evasive actions!

Unfortunately, the ship's navigation computer seems to be malfunctioning; rather than giving a route directly to safety, it produced extremely circuitous instructions. When the captain uses the PA system to ask if anyone can help, you quickly volunteer.

The navigation instructions (your puzzle input) consists of a sequence of single-character actions paired with integer input values. After staring at them for a few minutes, you work out what they probably mean:

    Action N means to move north by the given value.
    Action S means to move south by the given value.
    Action E means to move east by the given value.
    Action W means to move west by the given value.
    Action L means to turn left the given number of degrees.
    Action R means to turn right the given number of degrees.
    Action F means to move forward by the given value in the direction the ship is currently facing.

The ship starts by facing east. Only the L and R actions change the direction the ship is facing. (That is, if the ship is facing east and the next instruction is N10, the ship would move north 10 units, but would still move east if the following action were F.)

For example:

F10
N3
F7
R90
F11

These instructions would be handled as follows:

    F10 would move the ship 10 units east (because the ship starts by facing east) to east 10, north 0.
    N3 would move the ship 3 units north to east 10, north 3.
    F7 would move the ship another 7 units east (because the ship is still facing east) to east 17, north 3.
    R90 would cause the ship to turn right by 90 degrees and face south; it remains at east 17, north 3.
    F11 would move the ship 11 units south to east 17, south 8.

At the end of these instructions, the ship's Manhattan distance (sum of the absolute values of its east/west position and its north/south position) from its starting position is 17 + 8 = 25.

Figure out where the navigation instructions lead. What is the Manhattan distance between that location and the ship's starting position?
 */
fn part_1(instructions: Vec<(char, i32)>) -> i32 {
    let mut coord = (0, 0);
    let mut direction_idx: i32 = 0;
    instructions.iter().for_each(|(ins, val)| {
        let (dx, dy) = DIRECTIONS[direction_idx as usize];
        match ins {
            'N' => coord = (coord.0, coord.1 + val),
            'S' => coord = (coord.0, coord.1 - val),
            'E' => coord = (coord.0 + val, coord.1),
            'W' => coord = (coord.0 - val, coord.1),
            'F' => coord = (coord.0 + (dx * val), coord.1 + (dy * val)),
            'L' => direction_idx = (4 + (direction_idx - (val / 90))) % 4,
            'R' => direction_idx = (direction_idx + (val / 90)) % 4,
            _ => panic!("unknown instruction"),
        }
    });
    coord.0.abs() + coord.1.abs()
}

/*
--- Part Two ---

Before you can give the destination to the captain, you realize that the actual action meanings were printed on the back of the instructions the whole time.

Almost all of the actions indicate how to move a waypoint which is relative to the ship's position:

    Action N means to move the waypoint north by the given value.
    Action S means to move the waypoint south by the given value.
    Action E means to move the waypoint east by the given value.
    Action W means to move the waypoint west by the given value.
    Action L means to rotate the waypoint around the ship left (counter-clockwise) the given number of degrees.
    Action R means to rotate the waypoint around the ship right (clockwise) the given number of degrees.
    Action F means to move forward to the waypoint a number of times equal to the given value.

The waypoint starts 10 units east and 1 unit north relative to the ship. The waypoint is relative to the ship; that is, if the ship moves, the waypoint moves with it.

For example, using the same instructions as above:

    F10 moves the ship to the waypoint 10 times (a total of 100 units east and 10 units north), leaving the ship at east 100, north 10. The waypoint stays 10 units east and 1 unit north of the ship.
    N3 moves the waypoint 3 units north to 10 units east and 4 units north of the ship. The ship remains at east 100, north 10.
    F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28 units north), leaving the ship at east 170, north 38. The waypoint stays 10 units east and 4 units north of the ship.
    R90 rotates the waypoint around the ship clockwise 90 degrees, moving it to 4 units east and 10 units south of the ship. The ship remains at east 170, north 38.
    F11 moves the ship to the waypoint 11 times (a total of 44 units east and 110 units south), leaving the ship at east 214, south 72. The waypoint stays 4 units east and 10 units south of the ship.

After these operations, the ship's Manhattan distance from its starting position is 214 + 72 = 286.

Figure out where the navigation instructions actually lead. What is the Manhattan distance between that location and the ship's starting position?
 */
fn part_2(instructions: Vec<(char, i32)>) -> i32 {
    let mut wp = (10, 1);
    let mut coord = (0, 0);
    instructions.iter().for_each(|(ins, val)| match ins {
        'N' => wp = (wp.0, wp.1 + val),
        'S' => wp = (wp.0, wp.1 - val),
        'E' => wp = (wp.0 + val, wp.1),
        'W' => wp = (wp.0 - val, wp.1),
        'F' => coord = (coord.0 + (wp.0 * val), coord.1 + (wp.1 * val)),
        'L' => wp = rotate_waypoint(wp, -val / 90),
        'R' => wp = rotate_waypoint(wp, val / 90),
        _ => panic!("unknown instruction"),
    });
    coord.0.abs() + coord.1.abs()
}

fn rotate_waypoint(wp: (i32, i32), turn: i32) -> (i32, i32) {
    let mut tmp_wp = wp.clone();
    for _ in 0..turn.abs() {
        match turn / turn.abs() {
            1 => tmp_wp = (tmp_wp.1, tmp_wp.0 * -1),
            -1 => tmp_wp = (tmp_wp.1 * -1, tmp_wp.0),
            _ => panic!("invalid"),
        }
    }
    tmp_wp
}

mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "F10
N3
F7
R90
F11";

        assert_eq!(part_1(parse_input(input)), 25)
    }

    #[test]
    fn test_part_2() {
        let input = "F10
N3
F7
R90
F11
L180";

        assert_eq!(part_2(parse_input(input)), 286)
    }
}
