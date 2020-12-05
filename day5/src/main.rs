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

fn part1(inputs: Vec<&str>) -> i32 {
    return get_ids(inputs).iter().max().unwrap().clone();
}

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
