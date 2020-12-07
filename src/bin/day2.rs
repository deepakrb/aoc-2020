use std::fs;
// NOTE: Day 1 and 2 are incorrectly formatted and only contain the last answer, they need to be updated
// to include the working for Part 1

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
fn main() {
    let data = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");

    let inputs: Vec<&str> = data
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut valid_passes: Vec<_> = Vec::new();

    for input in inputs.iter() {
        let parsed_input: Vec<_> = input.split(|c| c == ',').collect();
        let (lower_bound, upper_bound, character, pass) = (
            parsed_input[0].parse::<i32>().unwrap(), 
            parsed_input[1].parse::<i32>().unwrap(), 
            parsed_input[2].chars().next().unwrap(), 
            parsed_input[3]
        );
        
        if is_valid(&pass, character, lower_bound, upper_bound) {
            valid_passes.push(pass)
        }
    }

    println!("Part 2: {}", valid_passes.iter().count());
}

fn is_valid(pass: &str, character: char, lower_bound: i32, upper_bound: i32) -> bool {
    let pass_vec: Vec<char> = pass.chars().collect();

    let mut contains_lower = false;
    if pass_vec[(lower_bound) as usize - 1] == character {
        contains_lower = true
    }

    let mut contains_upper = false;
    if pass_vec[(upper_bound) as usize - 1] == character {
        contains_upper = true
    }

    if contains_lower && contains_upper {
        return false
    }

    if !contains_lower && !contains_upper {
        return false
    }

    return true
}
