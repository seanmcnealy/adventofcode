use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|&c| char::to_digit(char::from(c), 10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        fn evaluate(grid: Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
            let mut level = grid[x][y];
            if level != 0 {
                0
            } else {
                let mut visited: Vec<(usize, usize)> = vec![(x, y)];
                while !visited.is_empty() {
                    let n = visited.first().unwrap().clone();
                    let n_level = grid[n.0][n.1];
                    if n_level == level + 1 {
                        level = n_level;
                    }
                    if n_level == 9 {
                        return visited.len() as u8;
                    }

                    let next = vec![
                        (n.0 as isize - 1, n.1 as isize),
                        (n.0 as isize + 1, n.1 as isize),
                        (n.0 as isize, n.1 as isize - 1),
                        (n.0 as isize, n.1 as isize + 1),
                    ]
                    .iter()
                    .filter(|m| {
                        if m.0 < 0
                            || m.0 as usize >= grid.len()
                            || m.1 < 0
                            || m.1 as usize >= grid[0].len()
                            || visited.contains(&(m.0 as usize, m.1 as usize))
                        {
                            false
                        } else {
                            grid[m.0 as usize][m.1 as usize] == n_level + 1
                        }
                    })
                    .map(|x| (x.0 as usize, x.1 as usize))
                    .collect::<Vec<(usize, usize)>>();
                    visited.extend(next);
                    visited = visited[1..].to_vec();
                }
                0
            }
        }

        Ok((0..grid.len())
            .map(|x| {
                (0..grid[0].len())
                    .map(|y| evaluate(grid.clone(), x, y) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>())
    }
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| {
                line.as_bytes()
                    .iter()
                    .map(|&c| char::to_digit(char::from(c), 10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        fn evaluate(grid: Vec<Vec<u8>>, x: usize, y: usize) -> u8 {
            let mut level = grid[x][y];
            if level != 0 {
                0
            } else {
                let mut visited: Vec<(usize, usize)> = vec![(x, y)];
                while !visited.is_empty() {
                    let n = visited.first().unwrap().clone();
                    let n_level = grid[n.0][n.1];
                    if n_level == level + 1 {
                        level = n_level;
                    }
                    if n_level == 9 {
                        return visited.len() as u8;
                    }

                    let next = vec![
                        (n.0 as isize - 1, n.1 as isize),
                        (n.0 as isize + 1, n.1 as isize),
                        (n.0 as isize, n.1 as isize - 1),
                        (n.0 as isize, n.1 as isize + 1),
                    ]
                    .iter()
                    .filter(|m| {
                        if m.0 < 0
                            || m.0 as usize >= grid.len()
                            || m.1 < 0
                            || m.1 as usize >= grid[0].len()
                        /*|| visited.contains(&(m.0 as usize, m.1 as usize))*/
                        {
                            false
                        } else {
                            grid[m.0 as usize][m.1 as usize] == n_level + 1
                        }
                    })
                    .map(|x| (x.0 as usize, x.1 as usize))
                    .collect::<Vec<(usize, usize)>>();
                    visited.extend(next);
                    visited = visited[1..].to_vec();
                }
                0
            }
        }

        Ok((0..grid.len())
            .map(|x| {
                (0..grid[0].len())
                    .map(|y| evaluate(grid.clone(), x, y) as usize)
                    .sum::<usize>()
            })
            .sum::<usize>())
    }
    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
