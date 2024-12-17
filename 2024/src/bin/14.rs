use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

fn main() -> Result<(), Error> {
    lazy_static! {
        // p=0,4 v=3,-3
        static ref REGEX: Regex = Regex::new(r"^p=(\d+),(\d+) v=(-?\d+),(-?\d+)$").unwrap();
    }
    fn parse_line(line: String) -> Option<(i32, i32, i32, i32)> {
        REGEX.captures(line.as_str()).map(|c| {
            (
                c[1].parse::<i32>().unwrap(),
                c[2].parse::<i32>().unwrap(),
                c[3].parse::<i32>().unwrap(),
                c[4].parse::<i32>().unwrap(),
            )
        })
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R, size_x: usize, size_y: usize) -> Result<usize, Error> {
        let mut agents = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(i32, i32, i32, i32)>>();

        let moved_agents = agents
            .iter()
            .map(|(x, y, v_x, v_y)| {
                let x = (x + (100 * v_x)).rem_euclid(size_x as i32);
                let y = (y + (100 * v_y)).rem_euclid(size_y as i32);
                (x, y)
            })
            .collect::<Vec<(i32, i32)>>();

        let q1 = moved_agents
            .iter()
            .filter(|(x, y)| *x < (size_x as i32 / 2) && *y < (size_y as i32 / 2))
            .count();
        let q2 = moved_agents
            .iter()
            .filter(|(x, y)| *x < (size_x as i32 / 2) && *y > (size_y as i32 / 2))
            .count();
        let q3 = moved_agents
            .iter()
            .filter(|(x, y)| *x > (size_x as i32 / 2) && *y < (size_y as i32 / 2))
            .count();
        let q4 = moved_agents
            .iter()
            .filter(|(x, y)| *x > (size_x as i32 / 2) && *y > (size_y as i32 / 2))
            .count();

        Ok(q1 * q2 * q3 * q4)
    }
    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R, size_x: usize, size_y: usize) -> Result<usize, Error> {
        let agents = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(i32, i32, i32, i32)>>();

        fn run_agents(
            agents: &Vec<(i32, i32, i32, i32)>,
            size_x: usize,
            size_y: usize,
            time: i32,
        ) -> Vec<(i32, i32)> {
            agents
                .iter()
                .map(|(x, y, v_x, v_y)| {
                    let x = (x + (time * v_x)).rem_euclid(size_x as i32);
                    let y = (y + (time * v_y)).rem_euclid(size_y as i32);
                    (x, y)
                })
                .collect::<Vec<(i32, i32)>>()
        }

        for i in 0..10000 {
            let moved_agents = run_agents(&agents, size_x, size_y, i);

            if *moved_agents
                .iter()
                .fold(HashMap::new(), |mut acc: HashMap<(i32, i32), i32>, n| {
                    *acc.entry(*n).or_insert(0) += 1;
                    acc
                })
                .values()
                .max()
                .unwrap()
                == 1
            {
                println!("iteration {}", i);
                for i in 0..size_x {
                    println!(
                        "{}",
                        String::from_utf8(
                            (0..size_y)
                                .map(|j| if moved_agents.contains(&(i as i32, j as i32)) {
                                    '*' as u8
                                } else {
                                    ' ' as u8
                                })
                                .collect::<Vec<u8>>()
                        )
                        .unwrap()
                    )
                }
            }
        }
        Ok(0)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);

    let result = time_snippet!(part2(input_file, 101, 103)?);
    println!("Result = {}", result);

    Ok(())
}
