use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use ndarray::{s, Array, Ix2};
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "5";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)\|(\d+)$").unwrap();
        static ref RE2: Regex = Regex::new(r"\d+").unwrap();
    }
    fn parse_line(
        acc: (Array<bool, Ix2>, Vec<Vec<u8>>),
        line: String,
    ) -> (Array<bool, Ix2>, Vec<Vec<u8>>) {
        let (mut acc1, mut acc2) = acc;
        RE.captures(line.as_str())
            .map(|cap| {
                let m1 = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let m2 = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
                acc1[[m2, m1]] = true;
            })
            .or_else(|| {
                let x = RE2
                    .find_iter(line.as_str())
                    .map(|c| c.as_str().parse::<u8>().unwrap())
                    .collect::<Vec<u8>>();
                if x.len() > 0 {
                    acc2.push(x);
                }
                None
            });
        (acc1, acc2)
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<u32, Error> {
        let (grid, lines) = reader.lines().flatten().fold(
            (Array::from_elem((100, 100), false), vec![]),
            |acc, line| parse_line(acc, line),
        );

        let answer = lines
            .iter()
            .map(|l: &Vec<u8>| {
                let mut row = Array::from_elem(100, false);
                for &i in l {
                    if row[usize::from(i)] {
                        return 0;
                    }
                    let next_row = grid.slice(s![usize::from(i), ..]);
                    row |= &next_row;
                }
                return u32::from(l[l.len() / 2]);
            })
            .sum::<u32>();

        Ok(answer)
    }
    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u32, Error> {
        let (grid, lines) = reader.lines().flatten().fold(
            (Array::from_elem((100, 100), false), vec![]),
            |acc, line| parse_line(acc, line),
        );

        let answer = lines
            .iter()
            .map(|l: &Vec<u8>| {
                let mut fixed = false;
                let mut error_row = Array::from_elem(100, false);
                let mut new_row: Vec<u8> = vec![];
                for &i in l {
                    let next_row = grid.slice(s![usize::from(i), ..]);
                    if error_row[usize::from(i)] {
                        let rules = grid.slice(s![.., usize::from(i)]);
                        let insert_location = new_row
                            .iter()
                            .take_while(|&&j| !rules[usize::from(j)])
                            .collect::<Vec<&u8>>()
                            .len();
                        new_row.insert(insert_location, i);
                        fixed = true;
                    } else {
                        new_row.push(i);
                    }

                    error_row |= &next_row;
                }
                return if fixed {
                    u32::from(new_row[new_row.len() / 2])
                } else {
                    0
                };
            })
            .sum::<u32>();

        Ok(answer)
    }
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
