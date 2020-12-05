use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(&data)));
    println!("Part 2: {}", part2(parse_input(&data)));
}

fn parse_input(input: &str) -> Vec<&str> {
    return input.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
}

/*
--- Day 5: Binary Boarding ---

You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.

The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.

So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.

As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?
 */
fn part1(inputs: Vec<&str>) -> i32 {
    return get_ids(inputs).iter().max().unwrap().clone();
}

/*
--- Part Two ---

Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

What is the ID of your seat?
 */
fn part2(inputs: Vec<&str>) -> i32 {
    let found_ids: HashSet<i32> = HashSet::from_iter(get_ids(inputs).iter().cloned());

    let all_possible_ids: HashSet<i32> = (*found_ids.iter().min().unwrap()..*found_ids.iter().max().unwrap()).collect();
    *all_possible_ids.difference(&found_ids).next().unwrap() as i32
}

fn get_ids(inputs: Vec<&str>) -> Vec<i32> {
    inputs.iter()
        .map(|input| {
            let (row, column) = get_row_value(input.parse().unwrap());
            return (row * 8) + column
        }).collect()
}

fn get_row_value(operation: String) -> (i32, i32) {
    let mut row_floor = 0;
    let mut row_limit = 128;
    let mut column_floor = 0;
    let mut column_limit = 8;

    operation.chars().for_each(|c| {
        let new_row = ((row_limit - row_floor) / 2) + row_floor;
        if c == 'F' { row_limit =  new_row; return; }
        if c == 'B' { row_floor = new_row; return; }

        let new_column = ((column_limit - column_floor) / 2) + column_floor;
        if c == 'L' { column_limit = new_column; return; }
        if c == 'R' { column_floor = new_column; return; }
    });

    (row_floor, column_floor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row_column() {
        assert_eq!(get_row_value("BFFFBBFRRR".to_string()), (70, 7));
        assert_eq!(get_row_value("FFFBBBFRRR".to_string()), (14, 7));
        assert_eq!(get_row_value("BBFFBBFRLL".to_string()), (102, 4));
    }

    #[test]
    fn test_part_1() {
        let input = "BFFFBBFRRR
        FFFBBBFRRR
        BBFFBBFRLL";

        assert_eq!(part1(parse_input(input)), 820);
    }
}
