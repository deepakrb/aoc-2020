use itertools::Itertools;
use lazy_static::lazy_static;
use std::fs;

enum Change {
    FILL,
    ABANDON,
    NONE,
}

lazy_static! {
    static ref SURROUNDING_DELTAS: Vec<(i32, i32)> = vec![
        (0, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
        (-1, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
    ];
}

fn main() {
    let data = fs::read_to_string("inputs/day11.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(data.as_str())));
    println!("Part 2: {}", part2(parse_input(data.as_str())));
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

/*
--- Day 11: Seating System ---

Your plane lands with plenty of time to spare. The final leg of your journey is a ferry that goes directly to the tropical island where you can finally start your vacation. As you reach the waiting area to board the ferry, you realize you're so early, nobody else has even arrived yet!

By modeling the process people use to choose (or abandon) their seat in the waiting area, you're pretty sure you can predict the best place to sit. You make a quick map of the seat layout (your puzzle input).

The seat layout fits neatly on a grid. Each position is either floor (.), an empty seat (L), or an occupied seat (#). For example, the initial seat layout might look like this:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

Now, you just need to model the people who will be arriving shortly. Fortunately, people are entirely predictable and always follow a simple set of rules. All decisions are based on the number of occupied seats adjacent to a given seat (one of the eight positions immediately up, down, left, right, or diagonal from the seat). The following rules are applied to every seat simultaneously:

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.

After one round of these rules, every seat in the example layout becomes occupied:

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

After a second round, the seats with four or more occupied adjacent seats become empty again:

#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##

This process continues for three more rounds:

#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##

#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##

#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##

At this point, something interesting happens: the chaos stabilizes and further applications of these rules cause no seats to change state! Once people stop moving around, you count 37 occupied seats.

Simulate your seating area by applying the seating rules repeatedly until no seats change state. How many seats end up occupied?
 */
fn part1(inputs: Vec<Vec<char>>) -> i32 {
    let mut previous = inputs;
    loop {
        let next = simulate_round(&previous, &part1_strategy);
        if is_equal(&previous, &next) {
            return get_occupied_seat_count(previous);
        }
        previous = next;
    }
}

fn part1_strategy(inputs: Vec<Vec<char>>, seat_value: char, x: usize, y: usize) -> Change {
    if seat_value == 'L' {
        let unoccupied = SURROUNDING_DELTAS
            .iter()
            .map(|&(dx, dy)| {
                let new_y = (y as i32 + dy) as usize;
                let new_x = (x as i32 + dx) as usize;

                if inputs.get(new_y).and_then(|col| col.get(new_x)).is_none() {
                    return 'L';
                }

                inputs[new_y][new_x]
            })
            .all(|val| val == 'L' || val == '.');

        return if unoccupied {
            Change::FILL
        } else {
            Change::NONE
        };
    }

    if seat_value == '#' {
        let occupied_count: i32 = SURROUNDING_DELTAS
            .iter()
            .map(|&(dx, dy)| {
                let new_y = (y as i32 + dy) as usize;
                let new_x = (x as i32 + dx) as usize;

                if inputs.get(new_y).and_then(|col| col.get(new_x)).is_none() {
                    return 0;
                }

                return if inputs[new_y][new_x] == '#' { 1 } else { 0 };
            })
            .sum();

        return if occupied_count >= 4 {
            Change::ABANDON
        } else {
            Change::NONE
        };
    }

    Change::NONE
}

/*
--- Part Two ---

As soon as people start to arrive, you realize your mistake. People don't just care about adjacent seats - they care about the first seat they can see in each of those eight directions!

Now, instead of considering just the eight immediately adjacent seats, consider the first seat in each of those eight directions. For example, the empty seat below would see eight occupied seats:

.......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....

The leftmost empty seat below would only see one empty seat, but cannot see any of the occupied ones:

.............
.L.L.#.#.#.#.
.............

The empty seat below would see no occupied seats:

.##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.

Also, people seem to be more tolerant than you expected: it now takes five or more visible occupied seats for an occupied seat to become empty (rather than four or more from the previous rules). The other rules still apply: empty seats that see no occupied seats become occupied, seats matching no rule don't change, and floor never changes.

Given the same starting layout as above, these new rules cause the seating area to shift around as follows:

L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL

#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##

#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#

#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#

Again, at this point, people stop shifting around and the seating area reaches equilibrium. Once this occurs, you count 26 occupied seats.

Given the new visibility method and the rule change for occupied seats becoming empty, once equilibrium is reached, how many seats end up occupied?
 */
fn part2(inputs: Vec<Vec<char>>) -> i32 {
    let mut previous = inputs;
    loop {
        let next = simulate_round(&previous, &part2_strategy);
        if is_equal(&previous, &next) {
            return get_occupied_seat_count(previous);
        }
        previous = next;
    }
}

fn part2_strategy(inputs: Vec<Vec<char>>, seat_value: char, x: usize, y: usize) -> Change {
    if seat_value == 'L' {
        let unoccupied = SURROUNDING_DELTAS
            .iter()
            .map(|&(dx, dy)| {
                let res = get_nearest_seat(&inputs, x, y, dx, dy);
                if res.is_none() {
                    return 'L';
                }

                return res.unwrap();
            })
            .all(|val| val == 'L');

        return if unoccupied {
            Change::FILL
        } else {
            Change::NONE
        };
    }

    if seat_value == '#' {
        let occupied_count: i32 = SURROUNDING_DELTAS
            .iter()
            .map(|&(dx, dy)| {
                let res = get_nearest_seat(&inputs, x, y, dx, dy);
                if res.is_none() {
                    return 0;
                }

                return if res.unwrap() == '#' { 1 } else { 0 };
            })
            .sum();

        return if occupied_count >= 5 {
            Change::ABANDON
        } else {
            Change::NONE
        };
    }

    Change::NONE
}

fn get_nearest_seat(inputs: &Vec<Vec<char>>, x: usize, y: usize, dx: i32, dy: i32) -> Option<char> {
    let mut cdx = dx;
    let mut cdy = dy;

    loop {
        let new_y = (y as i32 + cdy) as usize;
        let new_x = (x as i32 + cdx) as usize;

        if inputs.get(new_y).and_then(|col| col.get(new_x)).is_none() {
            return None;
        }

        if inputs[new_y][new_x] != '.' {
            return Some(inputs[new_y][new_x]);
        }

        cdx += dx;
        cdy += dy;
    }
}

fn simulate_round(
    inputs: &Vec<Vec<char>>,
    f: &dyn Fn(Vec<Vec<char>>, char, usize, usize) -> Change,
) -> Vec<Vec<char>> {
    inputs
        .iter()
        .enumerate()
        .map(|(y, val)| {
            val.iter()
                .enumerate()
                .map(|(x, value)| match f(inputs.clone(), *value, x, y) {
                    Change::FILL => '#',
                    Change::ABANDON => 'L',
                    Change::NONE => *value,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

fn is_equal(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    let matching = a
        .iter()
        .zip(b.iter())
        .filter(|&(a, b)| a.into_iter().collect::<String>() == b.into_iter().collect::<String>())
        .count();
    matching == a.len() && matching == b.len()
}

fn get_occupied_seat_count(inputs: Vec<Vec<char>>) -> i32 {
    inputs
        .iter()
        .map(|val| val.into_iter().collect::<String>())
        .join("")
        .chars()
        .map(|c| return if c == '#' { 1 } else { 0 })
        .sum::<i32>()
}

mod tests {
    use super::*;

    #[test]
    fn test_part_1_first_round() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let output = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";

        let next_round = simulate_round(&parse_input(input), &part1_strategy);
        let output_chars = parse_input(output);

        assert_eq!(is_equal(&next_round, &output_chars), true)
    }

    #[test]
    fn test_part_1_strategy_round_closed() {
        let repeating_input_output = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##";

        let next_round = simulate_round(&parse_input(repeating_input_output), &part1_strategy);
        let output_chars = parse_input(repeating_input_output);

        assert_eq!(is_equal(&next_round, &output_chars), true)
    }

    #[test]
    fn test_part_1_example_1() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(part1(parse_input(input)), 37)
    }

    #[test]
    fn test_part_2_example_1() {
        let input = "L.LL.LL.LL
    LLLLLLL.LL
    L.L.L..L..
    LLLL.LL.LL
    L.LL.LL.LL
    L.LLLLL.LL
    ..L.L.....
    LLLLLLLLLL
    L.LLLLLL.L
    L.LLLLL.LL";

        assert_eq!(part2(parse_input(input)), 26)
    }
}
