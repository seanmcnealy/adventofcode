use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
const TEST2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\d+").unwrap();
    }
    fn parse_line(line: String) -> Vec<u64> {
        REGEX
            .find_iter(line.as_str())
            .map(|c| c.as_str().parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<String, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<u64>>>();

        let mut a = lines[0][0];
        let mut b = lines[1][0];
        let mut c = lines[2][0];

        let program = lines[4].clone();

        let mut pc: usize = 0;

        let mut output_buffer: Vec<u64> = vec![];

        loop {
            let inst = program[pc];
            let literal = program[pc + 1];

            let combo = match literal {
                0 | 1 | 2 | 3 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => 0,
            };

            match inst {
                0 => {
                    // adv - division
                    a = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2
                }
                1 => {
                    // bxl - bitwise xor
                    b = b ^ literal;
                    pc += 2;
                }
                2 => {
                    // bst - store combo in b
                    b = combo & 0x7;
                    pc += 2;
                }
                3 => {
                    // jnz - jump not zero
                    pc = if a != 0 { literal as usize } else { pc + 2 }
                }
                4 => {
                    // bxc - bitwise xor
                    b = b ^ c;
                    pc += 2;
                }
                5 => {
                    // out - output
                    output_buffer.push(combo & 0x7);
                    pc += 2;
                }
                6 => {
                    // bdv - divide
                    b = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2;
                }
                7 => {
                    // cdv - divide
                    c = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2;
                }
                _ => {}
            }

            if pc >= program.len() {
                break;
            }
        }

        Ok(output_buffer
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0",
        part1(BufReader::new(TEST.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R, initial_a: u64) -> Result<String, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<u64>>>();

        let mut a = initial_a;
        let mut b: u64 = lines[1][0] as u64;
        let mut c: u64 = lines[2][0] as u64;

        let program = lines[4].clone();

        let mut pc: usize = 0;

        let mut output_buffer: Vec<u64> = vec![];

        loop {
            let inst = program[pc];
            let literal: u64 = program[pc + 1];

            let combo: u64 = match literal {
                0 | 1 | 2 | 3 => literal,
                4 => a,
                5 => b,
                6 => c,
                _ => 0,
            };

            match inst {
                0 => {
                    // adv - division
                    a = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2
                }
                1 => {
                    // bxl - bitwise xor
                    b = b ^ literal;
                    pc += 2;
                }
                2 => {
                    // bst - store combo in b
                    b = combo & 0x7;
                    pc += 2;
                }
                3 => {
                    // jnz - jump not zero
                    pc = if a != 0 { literal as usize } else { pc + 2 }
                }
                4 => {
                    // bxc - bitwise xor
                    b = b ^ c;
                    pc += 2;
                }
                5 => {
                    // out - output
                    output_buffer.push(combo & 0x7);
                    pc += 2;
                }
                6 => {
                    // bdv - divide
                    b = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2;
                }
                7 => {
                    // cdv - divide
                    c = a / (2i32.pow(combo as u32)) as u64;
                    pc += 2;
                }
                _ => {}
            }

            if pc >= program.len() {
                break;
            }
        }

        Ok(output_buffer
            .iter()
            .map(|&i| i.to_string())
            .collect::<Vec<String>>()
            .join(","))
    }
    // 3,0 jumps to beginning if a != 0
    // so a starts a 0!
    // 5,4 outputs a (which is why the final number is 0)
    // 0,3 divides a by 2^3 (right shift 3)
    // so take the program, right shift it backwards
    let mut answer = 0;
    for i in vec![0, 3, 4, 5, 3, 0] {
        answer |= i;
        answer <<= 3;
    }
    // target is 117440
    assert_eq!(
        "0,3,5,4,3,0",
        part2(BufReader::new(TEST2.as_bytes()), answer)?
    );

    // 3,0 jumps to beginning if a != 0
    // 5,5 outputs B (so B starts at 0)
    // 4,4 B = xor B and C // B == C on last iteration?
    // 1,5 B = B xor 5
    // 0,3 A >>= 3
    // 7,5 C = A >> B // C takes 3 bits from A, shifted up to 7 bits? zero on last iteration
    // 1,3 B = xor B and 3 // reverse of 2 of A's bits
    // 2,4 B = A & 0x7

    let mut answer: u64 = 0;
    for i in vec![6u64, 5, 6, 2, 0, 6, 2, 4, 4, 4, 2, 5, 7, 1, 5, 5] {
        answer <<= 3;
        answer |= i;
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, answer)?);

    println!("Answer = {}", answer);
    println!("Should be = {}", "2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0");
    println!("   Result = {}", result);

    Ok(())
}
