use std::collections::HashMap;
use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day_15.txt").expect("Unable to read file");

    println!("Part 1: {}", part_1(parse_input(data.as_str())));
    println!("Part 2: {}", part_2(parse_input(data.as_str())));
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<i32>().unwrap())
        .collect()
}

/*
--- Day 15: Rambunctious Recitation ---

You catch the airport shuttle and try to book a new flight to your vacation island. Due to the storm, all direct flights have been cancelled, but a route is available to get around the storm. You take it.

While you wait for your flight, you decide to check in with the Elves back at the North Pole. They're playing a memory game and are ever so excited to explain the rules!

In this game, the players take turns saying numbers. They begin by taking turns reading from a list of starting numbers (your puzzle input). Then, each turn consists of considering the most recently spoken number:

    If that was the first time the number has been spoken, the current player says 0.
    Otherwise, the number had been spoken before; the current player announces how many turns apart the number is from when it was previously spoken.

So, after the starting numbers, each turn results in that player speaking aloud either 0 (if the last number is new) or an age (if the last number is a repeat).

For example, suppose the starting numbers are 0,3,6:

    Turn 1: The 1st number spoken is a starting number, 0.
    Turn 2: The 2nd number spoken is a starting number, 3.
    Turn 3: The 3rd number spoken is a starting number, 6.
    Turn 4: Now, consider the last number spoken, 6. Since that was the first time the number had been spoken, the 4th number spoken is 0.
    Turn 5: Next, again consider the last number spoken, 0. Since it had been spoken before, the next number to speak is the difference between the turn number when it was last spoken (the previous turn, 4) and the turn number of the time it was most recently spoken before then (turn 1). Thus, the 5th number spoken is 4 - 1, 3.
    Turn 6: The last number spoken, 3 had also been spoken before, most recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
    Turn 7: Since 3 was just spoken twice in a row, and the last two turns are 1 turn apart, the 7th number spoken is 1.
    Turn 8: Since 1 is new, the 8th number spoken is 0.
    Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is the difference between them, 4.
    Turn 10: 4 is new, so the 10th number spoken is 0.

(The game ends when the Elves get sick of playing or dinner is ready, whichever comes first.)

Their question for you is: what will be the 2020th number spoken? In the example above, the 2020th number spoken will be 436.

Here are a few more examples:

    Given the starting numbers 1,3,2, the 2020th number spoken is 1.
    Given the starting numbers 2,1,3, the 2020th number spoken is 10.
    Given the starting numbers 1,2,3, the 2020th number spoken is 27.
    Given the starting numbers 2,3,1, the 2020th number spoken is 78.
    Given the starting numbers 3,2,1, the 2020th number spoken is 438.
    Given the starting numbers 3,1,2, the 2020th number spoken is 1836.

Given your starting numbers, what will be the 2020th number spoken?
 */
fn part_1(inputs: Vec<i32>) -> i32 {
    get_number(inputs, 2020)
}

/*
--- Part Two ---

Impressed, the Elves issue you a challenge: determine the 30000000th number spoken. For example, given the same starting numbers as above:

    Given 0,3,6, the 30000000th number spoken is 175594.
    Given 1,3,2, the 30000000th number spoken is 2578.
    Given 2,1,3, the 30000000th number spoken is 3544142.
    Given 1,2,3, the 30000000th number spoken is 261214.
    Given 2,3,1, the 30000000th number spoken is 6895259.
    Given 3,2,1, the 30000000th number spoken is 18.
    Given 3,1,2, the 30000000th number spoken is 362.

Given your starting numbers, what will be the 30000000th number spoken?
 */
fn part_2(inputs: Vec<i32>) -> i32 {
    get_number(inputs, 30000000)
}

fn get_number(inputs: Vec<i32>, goal: usize) -> i32 {
    let (last_number, mut hm) =
        inputs
            .iter()
            .enumerate()
            .fold((0, HashMap::new()), |(_, mut map), (i, &v)| {
                map.insert(v, vec![i]);
                (v, map)
            });

    (inputs.len()..goal).fold(last_number, |mut ln, i| {
        let def: Vec<usize> = vec![];
        let val = hm.get(&ln).unwrap_or(&def);
        if val.len() < 2 {
            ln = 0;
        } else {
            ln = (val[val.len() - 1] - val[val.len() - 2]) as i32
        }

        &hm.entry(ln).or_insert_with(Vec::new).push(i as usize);
        ln
    })
}

mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(parse_input("1,3,2")), 1);
        assert_eq!(part_1(parse_input("2,1,3")), 10);
        assert_eq!(part_1(parse_input("1,2,3")), 27);
        assert_eq!(part_1(parse_input("2,3,1")), 78);
        assert_eq!(part_1(parse_input("3,2,1")), 438);
        assert_eq!(part_1(parse_input("3,1,2")), 1836);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(parse_input("0,3,6")), 175594);
        assert_eq!(part_2(parse_input("1,3,2")), 2578);
        assert_eq!(part_2(parse_input("2,1,3")), 3544142);
        assert_eq!(part_2(parse_input("1,2,3")), 261214);
        assert_eq!(part_2(parse_input("2,3,1")), 6895259);
        assert_eq!(part_2(parse_input("3,2,1")), 18);
        assert_eq!(part_2(parse_input("3,1,2")), 362);
    }
}
