use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "125 17";

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
        let mut line = lines
            .first()
            .unwrap()
            .iter()
            .map(|&l| (l, 1))
            .collect::<Vec<(u64, u64)>>();

        fn sim(n: u64, c: u64) -> Vec<(u64, u64)> {
            let s = n.to_string();
            if n == 0 {
                vec![(1, c)]
            } else if s.len() % 2 == 0 {
                vec![
                    (s[0..(s.len() / 2)].parse::<u64>().unwrap(), c),
                    (s[(s.len() / 2)..].parse::<u64>().unwrap(), c),
                ]
            } else {
                vec![(n * 2024, c)]
            }
        }

        let mut next_line: Vec<(u64, u64)> = vec![];
        for _ in 0..25 {
            next_line = line
                .iter()
                .map(|n| sim(n.0, n.1))
                .flatten()
                .collect::<Vec<(u64, u64)>>();
            line = next_line
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<u64, u64>, n| {
                    *acc.entry(n.0).or_insert(0) += n.1;
                    acc
                })
                .iter()
                .map(|(&n, &c)| (n, c))
                .collect::<Vec<(u64, u64)>>();
        }

        Ok(line.iter().map(|(x, c)| c).sum())
    }
    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

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
        let mut line = lines
            .first()
            .unwrap()
            .iter()
            .map(|&l| (l, 1))
            .collect::<Vec<(u64, u64)>>();

        fn sim(n: u64, c: u64) -> Vec<(u64, u64)> {
            let s = n.to_string();
            if n == 0 {
                vec![(1, c)]
            } else if s.len() % 2 == 0 {
                vec![
                    (s[0..(s.len() / 2)].parse::<u64>().unwrap(), c),
                    (s[(s.len() / 2)..].parse::<u64>().unwrap(), c),
                ]
            } else {
                vec![(n * 2024, c)]
            }
        }

        let mut next_line: Vec<(u64, u64)> = vec![];
        for _ in 0..75 {
            next_line = line
                .iter()
                .map(|n| sim(n.0, n.1))
                .flatten()
                .collect::<Vec<(u64, u64)>>();
            line = next_line
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<u64, u64>, n| {
                    *acc.entry(n.0).or_insert(0) += n.1;
                    acc
                })
                .iter()
                .map(|(&n, &c)| (n, c))
                .collect::<Vec<(u64, u64)>>();
        }

        Ok(line.iter().map(|(x, c)| c).sum())
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
