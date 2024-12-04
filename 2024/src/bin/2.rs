use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;

const DAY: &str = "2";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    fn parse_line(line: String) -> Vec<i32> {
        RE.find_iter(line.as_str())
            .map(|cap| {
                cap.as_str().parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line));
        let answer2 =
            answer.fold(Vec::new(), |mut v1, l| {
                v1.push(l);
                v1
            });

        fn safe(l : &Vec<i32>) -> i32 {
            let mut last = 2*l[0] - l[1];
            let dir = l[1] - l[0];
            if l.iter().all(|&e| {
                let d = e - last;
                if d.abs() > 3 || d == 0 {return false}
                if d / d.abs() != dir / dir.abs() { return false }
                last = e;
                return true
            }) {1} else {0}
        }

        Ok(answer2
            .iter()
            .map(|l| safe(l))
            .sum())
    }
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line));
        let answer2 =
            answer.fold(Vec::new(), |mut v1, l| {
                v1.push(l);
                v1
            });

        fn safe1(l : &Vec<i32>) -> i32 {
            let mut last = 2*l[0] - l[1];
            let dir = l[1] - l[0];
            if l.iter().all(|&e| {
                let d = e - last;
                if d.abs() > 3 || d == 0 {return false}
                if d / d.abs() != dir / dir.abs() { return false }
                last = e;
                return true
            }) {1} else {0}
        }

        fn safe(l : &Vec<i32>) -> i32 {
            if safe1(&l[1..].to_vec()) == 1 {return 1}
            if safe1(&[l[0]].iter().cloned().chain(l[2..].iter().cloned()).collect()) == 1 {return 1}
            let mut last = 2*l[0] - l[1];
            let dir = l[1] - l[0];
            if dir == 0 {return 0};
            if l.iter().map(|&e| {
                let d = e - last;
                if d.abs() > 3 || d == 0 {1}
                else if d / d.abs() != dir / dir.abs() { 1 }
                else {
                    last = e;
                    0
                }
            }).sum::<i32>() < 2 {1} else {0}
        }

        Ok(answer2
            .iter()
            .map(|l| safe(l))
            .sum())
    }
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    Ok(())
}
