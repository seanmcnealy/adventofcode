use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use ndarray::{Array, Array2, Ix2};
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, Lines};
use std::iter::Flatten;
use std::sync::Mutex;

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\d+),(\d+)$").unwrap();
    }
    fn parse_line(line: String) -> Option<(usize, usize)> {
        REGEX.captures(line.as_str()).map(|c| {
            (
                c[1].parse::<usize>().unwrap(),
                c[2].parse::<usize>().unwrap(),
            )
        })
    }
    fn get_location(grid: &Array2<u8>, x: usize, y: usize, dx: isize, dy: isize) -> u8 {
        if x as isize + dx < 0
            || (x as isize + dx) as usize >= grid.shape()[0]
            || y as isize + dy < 0
            || (y as isize + dy) as usize >= grid.shape()[1]
        {
            '#' as u8
        } else {
            grid[[(x as isize + dx) as usize, (y as isize + dy) as usize]]
        }
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(
        reader: R,
        size_x: usize,
        size_y: usize,
        corrupted: usize,
    ) -> Result<usize, Error> {
        let falling = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        let mut grid: Array<u8, Ix2> = Array::from_elem((size_x, size_y), '.' as u8);

        falling[0..corrupted].iter().for_each(|(x, y)| {
            grid[[*y, *x]] = '#' as u8;
        });

        let start = (0, 0);
        let goal = (size_x - 1, size_y - 1);
        let mut solution = usize::MAX;

        let mut found: Vec<(usize, usize)> = vec![(start.0, start.1)];
        let mut search = VecDeque::from_iter(vec![(start.0, start.1, 0usize)]);

        while !search.is_empty() {
            let (agent_x, agent_y, cost) = search.pop_front().unwrap();
            if agent_x == goal.0 && agent_y == goal.1 && cost < solution {
                solution = cost;
            } else {
                let left = get_location(&grid, agent_x, agent_y, 0, -1);
                if left == '.' as u8 && !found.contains(&(agent_x, agent_y - 1)) {
                    found.push((agent_x, agent_y - 1));
                    search.push_back((agent_x, agent_y - 1, cost + 1));
                }
                let right = get_location(&grid, agent_x, agent_y, 0, 1);
                if right == '.' as u8 && !found.contains(&(agent_x, agent_y + 1)) {
                    found.push((agent_x, agent_y + 1));
                    search.push_back((agent_x, agent_y + 1, cost + 1));
                }
                let up = get_location(&grid, agent_x, agent_y, -1, 0);
                if up == '.' as u8 && !found.contains(&(agent_x - 1, agent_y)) {
                    found.push((agent_x - 1, agent_y));
                    search.push_back((agent_x - 1, agent_y, cost + 1));
                }
                let down = get_location(&grid, agent_x, agent_y, 1, 0);
                if down == '.' as u8 && !found.contains(&(agent_x + 1, agent_y)) {
                    found.push((agent_x + 1, agent_y));
                    search.push_back((agent_x + 1, agent_y, cost + 1));
                }
            }
        }

        Ok(solution)
    }
    assert_eq!(22, part1(BufReader::new(TEST1.as_bytes()), 7, 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 71, 1024)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(
        reader: R,
        size_x: usize,
        size_y: usize,
        corrupted: usize,
    ) -> Result<String, Error> {
        let falling = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        let mut grid: Array<u8, Ix2> = Array::from_elem((size_x, size_y), '.' as u8);

        let mut last_x = 0usize;
        let mut last_y = 0usize;
        for (i, (y, x)) in falling.iter().enumerate() {
            grid[[*x, *y]] = '#' as u8;
            if i < corrupted {
                continue;
            }

            let start = (0, 0);
            let goal = (size_x - 1, size_y - 1);
            let mut solution = usize::MAX;

            let mut found: Vec<(usize, usize)> = vec![(start.0, start.1)];
            let mut search = VecDeque::from_iter(vec![(start.0, start.1, 0usize)]);

            while !search.is_empty() {
                let (agent_x, agent_y, cost) = search.pop_front().unwrap();
                if agent_x == goal.0 && agent_y == goal.1 && cost < solution {
                    solution = cost;
                } else {
                    let left = get_location(&grid, agent_x, agent_y, 0, -1);
                    if left == '.' as u8 && !found.contains(&(agent_x, agent_y - 1)) {
                        found.push((agent_x, agent_y - 1));
                        search.push_back((agent_x, agent_y - 1, cost + 1));
                    }
                    let right = get_location(&grid, agent_x, agent_y, 0, 1);
                    if right == '.' as u8 && !found.contains(&(agent_x, agent_y + 1)) {
                        found.push((agent_x, agent_y + 1));
                        search.push_back((agent_x, agent_y + 1, cost + 1));
                    }
                    let up = get_location(&grid, agent_x, agent_y, -1, 0);
                    if up == '.' as u8 && !found.contains(&(agent_x - 1, agent_y)) {
                        found.push((agent_x - 1, agent_y));
                        search.push_back((agent_x - 1, agent_y, cost + 1));
                    }
                    let down = get_location(&grid, agent_x, agent_y, 1, 0);
                    if down == '.' as u8 && !found.contains(&(agent_x + 1, agent_y)) {
                        found.push((agent_x + 1, agent_y));
                        search.push_back((agent_x + 1, agent_y, cost + 1));
                    }
                }
            }
            if solution == usize::MAX {
                last_x = *x;
                last_y = *y;
                break;
            }
        }

        Ok(vec![last_y.to_string(), last_x.to_string()].join(","))
    }
    assert_eq!("6,1", part2(BufReader::new(TEST1.as_bytes()), 7, 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71, 71, 2548)?);
    println!("Result = {}", result);

    Ok(())
}
