use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "3";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
        static ref RE2: Regex = Regex::new(r"\((\d+),(\d+)\)").unwrap();
    }
    fn parse_line(line: String) -> Vec<(i32, i32)> {
        RE.find_iter(line.as_str())
            .map(|cap| {
                let m = cap.as_str();
                RE2.captures(m).map(|cap| {
                    let (_, [x, y]) = cap.extract::<2>();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
            })
            .flatten()
            .collect::<Vec<(i32, i32)>>()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader.lines().flatten().map(|line| parse_line(line));

        Ok(answer
            .map(|l| l.iter().map(|(x, y)| x * y).sum::<i32>())
            .sum())
    }
    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn parse_line2(line: String) -> Vec<(i32, i32)> {
        let mut on = true;
        RE.find_iter(line.as_str())
            .map(|cap| {
                let m = cap.as_str();
                if m == "do()" {
                    on = true
                }
                if m == "don't()" {
                    on = false
                }
                if !on {
                    None
                } else {
                    RE2.captures(m).map(|cap| {
                        let (_, [x, y]) = cap.extract::<2>();
                        (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                    })
                }
            })
            .flatten()
            .collect::<Vec<(i32, i32)>>()
    }

    fn part2<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader.lines().flatten().map(|line| parse_line2(line));

        Ok(answer
            .map(|l| l.iter().map(|(x, y)| x * y).sum::<i32>())
            .sum())
    }
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
