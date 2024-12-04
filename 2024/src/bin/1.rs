use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "1";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<(), Error> {
    fn parse_line(line: String) -> Option<(i32, i32)> {
        let parser: Regex = Regex::new(r"^(\d+)\s+(\d+)$").unwrap();
        parser
            .captures(line.as_str())
            .map(|cap| {
                cap.get(1)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .and_then(|x| cap.get(2).unwrap().as_str().parse::<i32>().map(|y| (x, y)))
                    .map(|x| Some(x))
                    .unwrap_or(None)
            })
            .flatten()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line))
            .flatten();
        let (mut answer2, mut answer3) =
            answer.fold((Vec::new(), Vec::new()), |(mut v1, mut v2), l| {
                match l {
                    (x, y) => {
                        v1.push(x);
                        v2.push(y)
                    }
                }
                (v1, v2)
            });
        answer2.sort();
        answer3.sort();

        Ok(answer2
            .iter()
            .zip(answer3.iter())
            .map(|(x, y)| (x - y).abs())
            .sum())
    }
    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i32, Error> {
        let answer = reader
            .lines()
            .flatten()
            .map(|line| parse_line(line))
            .flatten();
        let (answer2, map) = answer.fold(
            (Vec::new(), HashMap::<i32, i32>::new()),
            |(mut v, mut map), l| {
                match l {
                    (x, y) => {
                        v.push(x);
                        *map.entry(y).or_insert(0) += 1;
                    }
                }
                (v, map)
            },
        );

        Ok(answer2.iter().map(|x| x * *map.get(x).unwrap_or(&0)).sum())
    }
    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    Ok(())
}
