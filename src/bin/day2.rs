use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(&data)));
    println!("Part 2: {}", part2(parse_input(&data)));
}

fn parse_input(input: &str) -> Vec<(i32, i32, char, &str)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let parsed_input: Vec<&str> = l.split(',').collect();
            (
                parsed_input[0].parse::<i32>().unwrap(),
                parsed_input[1].parse::<i32>().unwrap(),
                parsed_input[2].chars().next().unwrap(),
                parsed_input[3],
            )
        })
        .collect()
}

/*
--- Day 2: Password Philosophy ---

Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc

Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?

--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
 */
fn part1(inputs: Vec<(i32, i32, char, &str)>) -> i32 {
    inputs
        .iter()
        .filter(|&(lb, ub, char, pass)| {
            let mut char_freq: HashMap<char, i32> = HashMap::new();
            pass.chars()
                .for_each(|c| *char_freq.entry(c).or_insert(0) += 1);

            is_valid_initial(char_freq, *char, *lb, *ub)
        })
        .count() as i32
}

fn is_valid_initial(
    char_freq: HashMap<char, i32>,
    character: char,
    lower_bound: i32,
    upper_bound: i32,
) -> bool {
    if !char_freq.contains_key(&character) {
        return false;
    }

    if char_freq[&character] >= lower_bound && char_freq[&character] <= upper_bound {
        return true;
    }

    return false;
}

/*
--- Part Two ---

While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

    1-3 a: abcde is valid: position 1 contains a and position 3 does not.
    1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
    2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.

How many passwords are valid according to the new interpretation of the policies?
 */
fn part2(inputs: Vec<(i32, i32, char, &str)>) -> i32 {
    inputs
        .iter()
        .filter(|(lb, ub, char, pass)| is_valid_fixed(pass, *char, *lb as usize, *ub as usize))
        .count() as i32
}

fn is_valid_fixed(pass: &str, character: char, lower_bound: usize, upper_bound: usize) -> bool {
    let pass_vec: Vec<char> = pass.chars().collect();

    let mut contains_lower = false;
    if pass_vec[(lower_bound) - 1] == character {
        contains_lower = true
    }

    let mut contains_upper = false;
    if pass_vec[(upper_bound) - 1] == character {
        contains_upper = true
    }

    if contains_lower && contains_upper {
        return false;
    }

    if !contains_lower && !contains_upper {
        return false;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        let input = "1,3,a,abcde
1,3,b,cdefg
2,9,c,ccccccccc";

        assert_eq!(part1(parse_input(input)), 2);
    }

    #[test]
    fn test_part_2_example() {
        let input = "1,3,a,abcde
1,3,b,cdefg
2,9,c,ccccccccc";

        assert_eq!(part2(parse_input(input)), 1);
    }
}
