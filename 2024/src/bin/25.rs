use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::slice::Chunks;

const DAY: &str = "25";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

fn main() -> Result<(), Error> {
    fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
        let len = v[0].len();
        let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(|s| s.as_bytes().to_vec())
            .filter(|l| l.len() > 0)
            .collect::<Vec<Vec<u8>>>();

        let locks: Vec<Vec<usize>> = lines
            .chunks(7)
            .filter(|grid| grid[0].iter().all(|c| *c == '#' as u8))
            .map(|grid| {
                transpose(grid.to_vec())
                    .iter()
                    .map(|col| col.iter().filter(|c| **c == ('#' as u8)).count() - 1)
                    .collect::<Vec<usize>>()
            })
            .collect();
        let keys: Vec<Vec<usize>> = lines
            .chunks(7)
            .filter(|grid| grid[0].iter().any(|c| *c != '#' as u8))
            .map(|grid| {
                transpose(grid.to_vec())
                    .iter()
                    .map(|col| col.iter().filter(|c| **c == ('#' as u8)).count() - 1)
                    .collect::<Vec<usize>>()
            })
            .collect();

        // locks.iter().for_each(|l| {
        //     println!("{:?}", l);
        // });
        // println!("");
        // keys.iter().for_each(|l| {
        //     println!("{:?}", l);
        // });

        Ok(keys
            .iter()
            .flat_map(|key| {
                locks
                    .iter()
                    .filter(|lock| key.iter().zip(lock.iter()).all(|(k, l)| k + l < 6))
            })
            .count())
    }
    assert_eq!(3, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");

    Ok(())
}
