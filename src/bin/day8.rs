use std::collections::HashSet;
use std::fs;

fn main() {
    let data = fs::read_to_string("inputs/day8.txt").expect("Unable to read file");

    println!("Part 1: {}", part1(parse_input(data.as_str())));
}

struct Parser {
    ip: i32,
    acc: i32,

    ops: Vec<(String, i32)>,
    run_ops: HashSet<i32>,
}

impl Parser {
    fn new(ops: Vec<(String, i32)>) -> Self {
        Self {
            ip: 0,
            acc: 0,
            ops,
            run_ops: Default::default(),
        }
    }

    fn run(&mut self) -> &mut Parser {
        loop {
            let (op, value) = &self.ops[self.ip as usize];
            if self.run_ops.contains(&self.ip) {
                return self;
            }

            &self.run_ops.insert(self.ip);

            match op.as_str() {
                "nop" => {
                    self.ip += 1;
                }
                "acc" => {
                    self.acc += value;
                    self.ip += 1;
                }
                "jmp" => {
                    self.ip += value;
                }
                _ => {
                    panic!("unknown op")
                }
            }

            if self.ip > (self.ops.len() as i32) - 1 {
                return self;
            }
        }
    }

    fn get_acc(&mut self) -> i32 {
        self.acc
    }
}

fn parse_input(input: &str) -> Vec<(String, i32)> {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|&l| !l.is_empty())
        .map(|l| {
            let instruction = l.split(" ").map(String::from).collect::<Vec<String>>();
            (
                String::from(&instruction[0]),
                instruction[1].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

/*
--- Day 8: Handheld Halting ---

Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.

Their handheld game console won't turn on! They ask if you can take a look.

You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.

The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).

    acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
    jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
    nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.

For example, consider the following program:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6

These instructions are visited in this order:

nop +0  | 1
acc +1  | 2, 8(!)
jmp +4  | 3
acc +3  | 6
jmp -3  | 7
acc -99 |
acc +1  | 4
jmp -4  | 5
acc +6  |

First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.

This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.

Immediately before the program would run an instruction a second time, the value in the accumulator is 5.

Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
 */
fn part1(operations: Vec<(String, i32)>) -> i32 {
    Parser::new(operations).run().get_acc()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(part1(parse_input(input)), 5)
    }
}