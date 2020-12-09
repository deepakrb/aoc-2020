use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct StringErr(String);

fn main() {
    let data = fs::read_to_string("inputs/day9.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(data.as_str()), 25));
    println!("Part 2: {}", part2(parse_input(data.as_str()), 25).unwrap());
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i64>().unwrap())
        .collect()
}

fn get_first_invalid_number(inputs: &Vec<i64>, preamble_length: usize) -> Result<i64, StringErr> {
    for (i, input) in inputs.iter().enumerate() {
        if i <= preamble_length {
            continue;
        }

        let preamble = inputs
            .split_at(i)
            .0
            .iter()
            .rev()
            .take(preamble_length)
            .collect::<Vec<&i64>>();

        let combinations = get_combinations(preamble);

        if !combinations.contains(input) {
            return Ok(*input);
        }
    }

    Err(StringErr("no_result".to_owned()))
}

fn get_combinations(preamble: Vec<&i64>) -> HashSet<i64> {
    preamble
        .iter()
        .map(|&x| preamble.iter().map(|&y| x + y).collect::<HashSet<i64>>())
        .fold(HashSet::new(), |mut acc, set| {
            acc.extend(set.iter());
            acc
        })
}

/*
--- Day 9: Encoding Error ---

With your neighbor happily enjoying their video game, you turn your attention to an open data port on the little screen in the seat in front of you.

Though the port is non-standard, you manage to connect it to your computer through the clever use of several paperclips. Upon connection, the port outputs a series of numbers (your puzzle input).

The data appears to be encrypted with the eXchange-Masking Addition System (XMAS) which, conveniently for you, is an old cypher with an important weakness.

XMAS starts by transmitting a preamble of 25 numbers. After that, each number you receive should be the sum of any two of the 25 immediately previous numbers. The two numbers will have different values, and there might be more than one such pair.

For example, suppose your preamble consists of the numbers 1 through 25 in a random order. To be valid, the next number must be the sum of two of those numbers:

    26 would be a valid next number, as it could be 1 plus 25 (or many other pairs, like 2 and 24).
    49 would be a valid next number, as it is the sum of 24 and 25.
    100 would not be valid; no two of the previous 25 numbers sum to 100.
    50 would also not be valid; although 25 appears in the previous 25 numbers, the two numbers in the pair must be different.

Suppose the 26th number is 45, and the first number (no longer an option, as it is more than 25 numbers ago) was 20. Now, for the next number to be valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that add up to it:

    26 would still be a valid next number, as 1 and 25 are still within the previous 25 numbers.
    65 would not be valid, as no two of the available numbers sum to it.
    64 and 66 would both be valid, as they are the result of 19+45 and 21+45 respectively.

Here is a larger example which only considers the previous 5 numbers (and has a preamble of length 5):

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this example, after the 5-number preamble, almost every number is the sum of two of the previous 5 numbers; the only number that does not follow this rule is 127.

The first step of attacking the weakness in the XMAS data is to find the first number in the list (after the preamble) which is not the sum of two of the 25 numbers before it. What is the first number that does not have this property?
 */
fn part1(inputs: Vec<i64>, preamble_length: usize) -> i64 {
    get_first_invalid_number(&inputs, preamble_length).unwrap()
}

/*
--- Part Two ---

The final step in breaking the XMAS encryption relies on the invalid number you just found: you must find a contiguous set of at least two numbers in your list which sum to the invalid number from step 1.

Again consider the above example:

35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576

In this list, adding up all of the numbers from 15 through 40 produces the invalid number from step 1, 127. (Of course, the contiguous set of numbers in your actual list might be much longer.)

To find the encryption weakness, add together the smallest and largest number in this contiguous range; in this example, these are 15 and 47, producing 62.

What is the encryption weakness in your XMAS-encrypted list of numbers?
 */
fn part2(inputs: Vec<i64>, preamble_length: usize) -> Result<i64, StringErr> {
    let invalid_number = get_first_invalid_number(&inputs, preamble_length).unwrap();

    // uses a head (idx_head) and tail (idx_tail) pointer to iterate through
    // the inputs list.
    for (idx_head, input) in inputs.iter().enumerate() {
        let mut idx_tail = idx_head + 1;
        let mut considered_numbers: Vec<i64> = vec![*input];

        loop {
            let current_number = &inputs[idx_tail];
            let rolling_sum: i64 = considered_numbers.iter().sum();

            // success
            if rolling_sum + current_number == invalid_number {
                return Ok(considered_numbers.iter().min().unwrap()
                    + considered_numbers.iter().max().unwrap());
            }

            // moves the idx_head forward
            if rolling_sum + current_number > invalid_number {
                break;
            }

            // moves the idx_tail forward
            considered_numbers.push(*current_number);
            idx_tail += 1;
        }
    }

    Err(StringErr("no_result".to_owned()))
}

mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(part1(parse_input(input), 5), 127)
    }

    #[test]
    fn test_example_2() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(part2(parse_input(input), 5).unwrap(), 62)
    }
}
