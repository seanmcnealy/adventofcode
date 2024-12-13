use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use ndarray::{s, Array, Ix2};
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "6";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R, start: (isize, isize)) -> Result<usize, Error> {
        let mut grid = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let mut agent: (isize, isize) = start;
        enum DIR {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        }
        let mut agent_dir = DIR::UP;

        while agent.0 >= 0
            && (agent.0 as usize) < grid.len()
            && agent.1 >= 0
            && (agent.1 as usize) < grid[0].len()
        {
            grid[agent.0 as usize][agent.1 as usize] = 'X' as u8;
            match agent_dir {
                DIR::UP => {
                    if agent.0 - 1 >= 0
                        && grid[agent.0 as usize - 1][agent.1 as usize] != ('#' as u8)
                    {
                        agent = (agent.0 - 1, agent.1);
                    } else if agent.0 - 1 >= 0 {
                        agent_dir = DIR::RIGHT;
                    } else {
                        agent = (agent.0 - 1, agent.1);
                    }
                }
                DIR::DOWN => {
                    if (agent.0 as usize + 1) < grid.len()
                        && grid[agent.0 as usize + 1][agent.1 as usize] != '#' as u8
                    {
                        agent = (agent.0 + 1, agent.1);
                    } else if agent.0 as usize + 1 < grid.len() {
                        agent_dir = DIR::LEFT;
                    } else {
                        agent = (agent.0 + 1, agent.1);
                    }
                }
                DIR::LEFT => {
                    if agent.1 - 1 >= 0 && grid[agent.0 as usize][agent.1 as usize - 1] != '#' as u8
                    {
                        agent = (agent.0, agent.1 - 1);
                    } else if agent.1 - 1 >= 0 {
                        agent_dir = DIR::UP;
                    } else {
                        agent = (agent.0, agent.1 - 1);
                    }
                }
                DIR::RIGHT => {
                    if (agent.1 as usize + 1) < grid[0].len()
                        && grid[agent.0 as usize][agent.1 as usize + 1] != '#' as u8
                    {
                        agent = (agent.0, agent.1 + 1);
                    } else if agent.1 as usize + 1 < grid[0].len() {
                        agent_dir = DIR::DOWN;
                    } else {
                        agent = (agent.0, agent.1 + 1);
                    }
                }
            }
        }

        Ok(grid
            .iter()
            .map(|l| l.iter().filter(|&&c| c == 'X' as u8).count())
            .sum())
    }
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()), (6, 4))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, (69, 91))?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R, start: (isize, isize)) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();

        #[derive(PartialEq)]
        enum DIR {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        }

        fn sim(grid: &Vec<Vec<u8>>, start: (isize, isize), ob: (usize, usize)) -> bool {
            let mut agent: (isize, isize) = start;
            let mut agent_dir = DIR::UP;

            let mut turns: Vec<(isize, isize, DIR)> = vec![];
            let mut cycle = false;

            while !cycle
                && agent.0 >= 0
                && (agent.0 as usize) < grid.len()
                && agent.1 >= 0
                && (agent.1 as usize) < grid[0].len()
            {
                match agent_dir {
                    DIR::UP => {
                        if agent.0 - 1 >= 0
                            && grid[agent.0 as usize - 1][agent.1 as usize] != ('#' as u8)
                            && (agent.0 as usize - 1, agent.1 as usize) != ob
                        {
                            agent = (agent.0 - 1, agent.1);
                        } else if agent.0 - 1 >= 0 {
                            if turns.contains(&(agent.0, agent.1, DIR::UP)) {
                                cycle = true
                            }
                            agent_dir = DIR::RIGHT;
                            turns.push((agent.0, agent.1, DIR::UP));
                        } else {
                            agent = (agent.0 - 1, agent.1);
                        }
                    }
                    DIR::DOWN => {
                        if (agent.0 as usize + 1) < grid.len()
                            && grid[agent.0 as usize + 1][agent.1 as usize] != '#' as u8
                            && (agent.0 as usize + 1, agent.1 as usize) != ob
                        {
                            agent = (agent.0 + 1, agent.1);
                        } else if agent.0 as usize + 1 < grid.len() {
                            if turns.contains(&(agent.0, agent.1, DIR::DOWN)) {
                                cycle = true
                            }
                            agent_dir = DIR::LEFT;
                            turns.push((agent.0, agent.1, DIR::DOWN));
                        } else {
                            agent = (agent.0 + 1, agent.1);
                        }
                    }
                    DIR::LEFT => {
                        if agent.1 - 1 >= 0
                            && grid[agent.0 as usize][agent.1 as usize - 1] != '#' as u8
                            && (agent.0 as usize, agent.1 as usize - 1) != ob
                        {
                            agent = (agent.0, agent.1 - 1);
                        } else if agent.1 - 1 >= 0 {
                            if turns.contains(&(agent.0, agent.1, DIR::LEFT)) {
                                cycle = true
                            }
                            agent_dir = DIR::UP;
                            turns.push((agent.0, agent.1, DIR::LEFT));
                        } else {
                            agent = (agent.0, agent.1 - 1);
                        }
                    }
                    DIR::RIGHT => {
                        if (agent.1 as usize + 1) < grid[0].len()
                            && grid[agent.0 as usize][agent.1 as usize + 1] != '#' as u8
                            && (agent.0 as usize, agent.1 as usize + 1) != ob
                        {
                            agent = (agent.0, agent.1 + 1);
                        } else if agent.1 as usize + 1 < grid[0].len() {
                            if turns.contains(&(agent.0, agent.1, DIR::RIGHT)) {
                                cycle = true
                            }
                            agent_dir = DIR::DOWN;
                            turns.push((agent.0, agent.1, DIR::RIGHT));
                        } else {
                            agent = (agent.0, agent.1 + 1);
                        }
                    }
                }
            }

            cycle
        }

        let mut found = 0;
        for i in 0..130 {
            for j in 0..130 {
                if sim(&grid, start, (i, j)) {
                    found += 1;
                }
            }
        }

        Ok(found)
    }
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()), (6, 4))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, (69, 91))?);
    println!("Result = {}", result);

    Ok(())
}
