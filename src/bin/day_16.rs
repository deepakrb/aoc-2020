use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day_16.txt").expect("Unable to read file");

    println!("Part 1: {}", part_1(parse_input(data.as_str())));
    println!(
        "Part 2: {}",
        part_2(parse_input(data.as_str()), "departure")
    );
}

struct Input {
    limits: Vec<(String, Vec<(i32, i32)>)>,
    m_ticket: Vec<i32>,
    o_tickets: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> Input {
    let values: Vec<&str> = input.split("\n\n").collect();
    let limits = values[0]
        .lines()
        .map(|l| {
            let spl: Vec<&str> = l.split(":").collect();
            (
                spl[0].trim().to_string(),
                spl[1]
                    .split("or")
                    .map(|c| {
                        let v = c
                            .trim()
                            .split("-")
                            .map(|v| v.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        (v[0], v[1])
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<(String, Vec<(i32, i32)>)>>();

    let m_ticket = values[1].lines().collect::<Vec<&str>>()[1]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let o_tickets = values[2].lines().collect::<Vec<&str>>()[1..]
        .iter()
        .map(|l| {
            l.split(",")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    Input {
        limits,
        m_ticket,
        o_tickets,
    }
}

/*
--- Day 16: Ticket Translation ---

As you're walking to yet another connecting flight, you realize that one of the legs of your re-routed trip coming up is on a high-speed train. However, the train ticket you were given is in a language you don't understand. You should probably figure out what it says before you get to the train station after the next flight.

Unfortunately, you can't actually read the words on the ticket. You can, however, read the numbers, and so you figure out the fields these tickets must have and the valid ranges for values in those fields.

You collect the rules for ticket fields, the numbers on your ticket, and the numbers on other nearby tickets for the same train service (via the airport security cameras) together into a single document you can reference (your puzzle input).

The rules for ticket fields specify a list of fields that exist somewhere on the ticket and the valid ranges of values for each field. For example, a rule like class: 1-3 or 5-7 means that one of the fields in every ticket is named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such that 3 and 5 are both valid in this field, but 4 is not).

Each ticket is represented by a single line of comma-separated values. The values are the numbers on the ticket in the order they appear; every ticket has the same format. For example, consider this ticket:

.--------------------------------------------------------.
| ????: 101    ?????: 102   ??????????: 103     ???: 104 |
|                                                        |
| ??: 301  ??: 302             ???????: 303      ??????? |
| ??: 401  ??: 402           ???? ????: 403    ????????? |
'--------------------------------------------------------'

Here, ? represents text in a language you don't understand. This ticket might be represented as 101,102,103,104,301,302,303,401,402,403; of course, the actual train tickets you're looking at are much more complicated. In any case, you've extracted just the numbers in such a way that the first number is always the same specific field, the second number is always a different specific field, and so on - you just don't know what each position actually means!

Start by determining which tickets are completely invalid; these are tickets that contain values which aren't valid for any field. Ignore your ticket for now.

For example, suppose you have the following notes:

class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12

It doesn't matter which position corresponds to which field; you can identify invalid nearby tickets by considering only whether tickets contain values that are not valid for any field. In this example, the values on the first nearby ticket are all valid for at least one field. This is not true of the other three nearby tickets: the values 4, 55, and 12 are are not valid for any field. Adding together all of the invalid values produces your ticket scanning error rate: 4 + 55 + 12 = 71.

Consider the validity of the nearby tickets you scanned. What is your ticket scanning error rate?
 */
fn part_1(inp: Input) -> i32 {
    vec![inp.m_ticket.clone()]
        .into_iter()
        .chain(inp.o_tickets.clone().into_iter())
        .map(|v| {
            v.iter()
                .filter(|val| {
                    !inp.limits
                        .iter()
                        .map(|(_, checks)| {
                            checks
                                .iter()
                                .map(|&(lb, ub)| *val.clone() >= lb && *val.clone() <= ub)
                                .any(|x| x == true)
                        })
                        .any(|x| x == true)
                })
                .map(|v| *v)
                .collect::<Vec<i32>>()
        })
        .flatten()
        .sum()
}

/*
--- Part Two ---

Now that you've identified which tickets contain invalid values, discard those tickets entirely. Use the remaining valid tickets to determine which field is which.

Using the valid ranges for each field, determine what order the fields appear on the tickets. The order is consistent between all tickets: if seat is the third field, it is the third field on every ticket, including your ticket.

For example, suppose you have the following notes:

class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9

Based on the nearby tickets in the above example, the first position must be row, the second position must be class, and the third position must be seat; you can conclude that in your ticket, class is 12, row is 11, and seat is 13.

Once you work out which field is which, look for the six fields on your ticket that start with the word departure. What do you get if you multiply those six values together?
 */
fn part_2(inp: Input, prefix: &str) -> i64 {
    // We reduce the inputs into a hashmap of possible values per field index i.e.
    // { 1: ["arrival location", "departure time"], 2: ["departure time"]. each position contains
    // multiple possible fields.
    let possible_values = vec![inp.m_ticket.clone()]
        .into_iter()
        .chain(inp.o_tickets.clone().into_iter())
        .map(|v| get_possible_fields(v, &inp.limits))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, HashSet<String>>, vals| {
                vals.iter().enumerate().for_each(|(i, hs)| {
                    if let Some(v) = acc.get_mut(&i) {
                        *v = v.clone().intersection(hs).map(|v| v.clone()).collect();
                    } else {
                        acc.insert(i, hs.clone());
                    }
                });
                acc
            },
        );

    product_of_field_prefix(prefix, inp.m_ticket, get_field_at_index(&possible_values))
}

fn get_possible_fields(
    ticket: Vec<i32>,
    limits: &Vec<(String, Vec<(i32, i32)>)>,
) -> Vec<HashSet<String>> {
    ticket
        .iter()
        .map(|&val| {
            limits
                .iter()
                .fold(HashSet::new(), |mut acc, (pos, checks)| {
                    let passes = checks
                        .iter()
                        .map(|&(lb, ub)| val.clone() >= lb && val.clone() <= ub)
                        .any(|x| x == true);
                    if passes {
                        acc.insert(pos.clone());
                    }
                    acc
                })
        })
        .filter(|v| v.len() != 0)
        .collect()
}

// get_field_at_index builds a mapping of index to field. Note, there is a possibility this doesn't converge
// It loops over the possible value set, if it finds a concrete field, i.e.  a field where only one
// possible value can fit it can remove that field as a possibility from all other indexes.
fn get_field_at_index(poss_fields: &HashMap<usize, HashSet<String>>) -> HashMap<usize, String> {
    let mut inp = poss_fields.clone();
    let mut hm: HashMap<usize, String> = HashMap::new();

    loop {
        let mut found_single_value = false;

        for (i, key_vals) in inp.clone().into_iter() {
            if key_vals.len() != 1 {
                continue;
            }

            let val = key_vals.iter().collect::<Vec<&String>>()[0];

            // remove this value from all other keys, its definitely at this index
            for (idx, mut vals) in inp.clone().into_iter() {
                vals.remove(&val.clone());
                inp.insert(idx, vals);
            }

            hm.insert(i, val.to_string());
            found_single_value = true;
        }

        if !found_single_value {
            return hm;
        }
    }
}

// product_of_field_prefix multiplies the fields value for keys with a prefix
fn product_of_field_prefix(prefix: &str, ticket: Vec<i32>, fields: HashMap<usize, String>) -> i64 {
    let mut acc: i64 = 1;
    for (key, value) in fields {
        if value.starts_with(prefix) {
            acc = acc * (*ticket.get(key).unwrap() as i64)
        }
    }
    acc
}

mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(part_1(parse_input(input)), 71);
    }
}
