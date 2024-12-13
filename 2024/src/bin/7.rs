use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "7";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    }
    fn parse_line(line: String) -> Vec<u64> {
        NUMBER_REGEX
            .find_iter(line.as_str())
            .map(|c| c.as_str().parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<u64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<u64>>>();

        fn next<'a>(answer: u64, current: u64, remaining: Vec<u64>) -> Vec<u64> {
            if current == answer && remaining.is_empty() {
                vec![current]
            } else if remaining.is_empty() || current > answer {
                vec![]
            } else {
                next(answer, current + remaining[0], remaining[1..].to_vec())
                    .iter()
                    .chain(next(answer, current * remaining[0], remaining[1..].to_vec()).iter())
                    .map(|&c| c)
                    .collect::<Vec<u64>>()
            }
        }

        let answer = lines
            .iter()
            .filter(|l: &&Vec<u64>| !l.is_empty() && !next(l[0], 0, l[1..].to_vec()).is_empty());

        Ok(answer.map(|l| l[0]).sum())
    }
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<u64>>>();

        fn next<'a>(answer: u64, current: u64, remaining: Vec<u64>) -> Vec<u64> {
            if current == answer && remaining.is_empty() {
                vec![current]
            } else if remaining.is_empty() || current > answer {
                vec![]
            } else {
                next(
                    answer,
                    format!("{}{}", current, remaining[0])
                        .parse::<u64>()
                        .unwrap(),
                    remaining[1..].to_vec(),
                )
                .iter()
                .chain(
                    next(answer, current + remaining[0], remaining[1..].to_vec())
                        .iter()
                        .chain(
                            next(answer, current * remaining[0], remaining[1..].to_vec()).iter(),
                        ),
                )
                .map(|&c| c)
                .collect::<Vec<u64>>()
            }
        }

        let answer = lines
            .iter()
            .filter(|l: &&Vec<u64>| !l.is_empty() && !next(l[0], 0, l[1..].to_vec()).is_empty());

        Ok(answer.map(|l| l[0]).sum())
    }
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
