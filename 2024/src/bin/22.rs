use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "22";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
1
10
100
2024
";
const TEST2: &str = "\
1
2
3
2024
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^\d+$").unwrap();
    }
    fn parse_line(line: String) -> Option<usize> {
        REGEX
            .captures(line.as_str())
            .map(|c| c[0].parse::<usize>().unwrap())
    }
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let start = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<usize>>();

        // test 1
        /*(0..10).fold(123usize, |acc, _|{
            let step1 = ((acc << 6) ^ acc ) % 16777216;
            let step2 = ((step1 >> 5) ^ step1) % 16777216;
            let step3 = ((step2 << 11) ^ step2) % 16777216;
            println!("{}", step3);
            step3
        });*/

        Ok(start
            .iter()
            .map(|&s| {
                (0..2000).fold(s, |acc, _| {
                    let step1 = ((acc << 6) ^ acc) % 16777216;
                    let step2 = ((step1 >> 5) ^ step1) % 16777216;
                    let step3 = ((step2 << 11) ^ step2) % 16777216;
                    step3
                })
            })
            .sum())
    }
    assert_eq!(37327623, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        let start = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<usize>>();

        let mut patterns: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();

        start.iter().for_each(|&s| {
            let mut inner_patterns: HashMap<(isize, isize, isize, isize), usize> = HashMap::new();
            (0..2000).fold((0usize, 0usize, 0usize, s), |(s0, s1, s2, s3), _| {
                let step1 = ((s3 << 6) ^ s3) % 16777216;
                let step2 = ((step1 >> 5) ^ step1) % 16777216;
                let step3 = ((step2 << 11) ^ step2) % 16777216;
                let pat = (
                    s1 as isize % 10 - s0 as isize % 10,
                    s2 as isize % 10 - s1 as isize % 10,
                    s3 as isize % 10 - s2 as isize % 10,
                    step3 as isize % 10 - s3 as isize % 10,
                );
                if !inner_patterns.contains_key(&pat) {
                    inner_patterns.insert(pat, step3 % 10);
                }

                (s1, s2, s3, step3)
            });
            for (p, n) in inner_patterns.iter() {
                patterns
                    .entry(*p)
                    .and_modify(|v| *v += n)
                    .or_insert(n.clone());
            }
        });

        Ok(*patterns.values().max().unwrap())
    }
    assert_eq!(23, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
