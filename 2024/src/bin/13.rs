use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

fn main() -> Result<(), Error> {
    static mut best: i64 = i64::MAX;
    lazy_static! {
        // Button A: X+94, Y+34
        static ref REGEX_A: Regex = Regex::new(r"Button A: X([+-]?\d+), Y([+-]\d+)").unwrap();
        // Button B: X+22, Y+67
        static ref REGEX_B: Regex = Regex::new(r"Button B: X([+-]?\d+), Y([+-]\d+)").unwrap();
        //Prize: X=8400, Y=5400
        static ref REGEX_PRIZE: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    }
    fn parse_line(line: String) -> Option<(i32, i32)> {
        REGEX_A
            .captures(line.as_str())
            .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
            .or(REGEX_B
                .captures(line.as_str())
                .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap())))
            .or(REGEX_PRIZE
                .captures(line.as_str())
                .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap())))
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        unsafe fn step(
            a_value: (i32, i32),
            b_value: (i32, i32),
            a_count: i32,
            b_count: i32,
            cost: i64,
            state: (i32, i32),
            target: (i32, i32),
        ) -> Option<i64> {
            if a_count > 100 || b_count > 100 || cost as i64 > best {
                None
            } else if state == target {
                let x = &mut best;
                *x = cost;
                Some(cost)
            } else if state.0 > target.0 || state.1 > target.1 {
                None
            } else if b_count > 0 {
                step(
                    a_value,
                    b_value,
                    a_count,
                    b_count + 1,
                    cost + 1,
                    (state.0 + b_value.0, state.1 + b_value.1),
                    target,
                )
            } else {
                vec![
                    step(
                        a_value,
                        b_value,
                        a_count + 1,
                        b_count,
                        cost + 3,
                        (state.0 + a_value.0, state.1 + a_value.1),
                        target,
                    ),
                    step(
                        a_value,
                        b_value,
                        a_count,
                        b_count + 1,
                        cost + 1,
                        (state.0 + b_value.0, state.1 + b_value.1),
                        target,
                    ),
                ]
                .iter()
                .flatten()
                .map(|&x| x)
                .min()
            }
        }

        Ok(lines
            .chunks(3)
            .map(|chunk| unsafe {
                let a = chunk[0];
                let b = chunk[1];
                let p = chunk[2];

                let x = &mut best;
                *x = i64::MAX;

                step(a, b, 0, 0, 0, (0, 0), p)
            })
            .flatten()
            .sum::<i64>())
    }
    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(i32, i32)>>();

        Ok(lines
            .chunks(3)
            .map(|chunk| {
                let a = chunk[0];
                let b = chunk[1];
                let p = chunk[2];

                let target = (p.0 as u64 + 10000000000000, p.1 as u64 + 10000000000000);

                let x = (target.0 as i64 * b.1 as i64 - target.1 as i64 * b.0 as i64)
                    / (a.0 as i64 * b.1 as i64 - a.1 as i64 * b.0 as i64);
                let y = (target.0 as i64 * -(a.1 as i64) + target.1 as i64 * a.0 as i64)
                    / (a.0 as i64 * b.1 as i64 - a.1 as i64 * b.0 as i64);

                if a.0 as i64 * x + b.0 as i64 * y == target.0 as i64
                    && a.1 as i64 * x + b.1 as i64 * y == target.1 as i64
                {
                    Some(3 * x + y)
                } else {
                    None
                }
            })
            .flatten()
            .sum::<i64>())
    }
    assert_eq!(875318608908, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
