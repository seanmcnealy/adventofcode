use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::Mutex;

const DAY: &str = "19";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"[wubrg]+").unwrap();
    }
    fn parse_line(line: String) -> Vec<String> {
        REGEX
            .find_iter(line.as_str())
            .map(|c| String::from(c.as_str()))
            .collect::<Vec<String>>()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<String>>>();

        let mut a = lines[0].to_vec();
        let towels = a
            .iter()
            .filter(|x| x.len() == 1 || x.contains("g"))
            .map(|x| x.clone())
            .collect();

        let b: Vec<String> = lines
            .iter()
            .map(|x| {
                if x.len() == 1 {
                    Some(x[0].clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        fn check(a: String, b: &Vec<String>) -> bool {
            if a.is_empty() {
                true
            } else {
                let mut found = false;
                for x in b {
                    if a.eq(x) {
                        found = true;
                        break;
                    } else if a.starts_with(x) {
                        found = check(
                            String::from_utf8(a.as_bytes()[x.len()..].to_vec()).unwrap(),
                            b,
                        );
                        if found {
                            break;
                        }
                    }
                }
                found
            }
        }

        Ok(b.iter()
            .map(|goal| {
                println!("{}", goal);
                if check(goal.clone(), &towels) {
                    1
                } else {
                    0
                }
            })
            .sum())
    }
    assert_eq!(6, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    lazy_static! {
        static ref memo: Mutex<HashMap<String, usize>> = Mutex::new(HashMap::new());
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        memo.lock().unwrap().clear();
        let lines = reader
            .lines()
            .flatten()
            .map(parse_line)
            .collect::<Vec<Vec<String>>>();

        let mut a = lines[0].to_vec();
        let towels = a.iter().map(|x| x.clone()).collect::<HashSet<String>>();

        let max_size = towels.iter().map(|s| s.len()).max().unwrap();

        let b: Vec<String> = lines
            .iter()
            .map(|x| {
                if x.len() == 1 {
                    Some(x[0].clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        fn check(max_size: usize, a: String, b: &HashSet<String>) -> usize {
            if memo.lock().unwrap().contains_key(&a) {
                *memo.lock().unwrap().get(&a).unwrap()
            } else {
                let answer = (1..=max_size)
                    .map(|i| {
                        if i == a.len() {
                            if b.contains(&a) {
                                1
                            } else {
                                0
                            }
                        } else if i > a.len() {
                            0
                        } else if b
                            .contains(&String::from_utf8(a.as_bytes()[0..i].to_vec()).unwrap())
                        {
                            check(
                                max_size,
                                String::from_utf8(a.as_bytes()[i..].to_vec()).unwrap(),
                                b,
                            )
                        } else {
                            0
                        }
                    })
                    .sum();
                memo.lock().unwrap().insert(a.clone(), answer);
                answer
            }
        }

        Ok(b.iter()
            .map(|goal| {
                println!("{}", goal);
                check(max_size, goal.clone(), &towels)
            })
            .sum())
    }
    assert_eq!(16, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
