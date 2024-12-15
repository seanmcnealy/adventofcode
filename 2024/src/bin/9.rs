use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "9";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "2333133121414131402";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<u64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let line = lines.first().unwrap();

        let mut l = 0;
        let mut r = line.len() - 1;
        let mut r_rem = char::to_digit(char::from(line[r]), 10).unwrap();
        let mut i = 0;

        let mut acc: u64 = 0;

        while l < r {
            let unmoved = char::to_digit(char::from(line[l]), 10).unwrap();
            for j in 0..unmoved {
                // println!("index {} id {}", i, l/2);
                acc = acc + (i * l / 2) as u64;
                i += 1;
            }
            l += 1;

            if l < r {
                let moved = char::to_digit(char::from(line[l]), 10).unwrap();
                for j in 0..moved {
                    if l < r {
                        // println!("index {} id {} rem {}", i, r / 2, r_rem);
                        acc = acc + (i * (r / 2)) as u64;
                        i += 1;
                        r_rem -= 1;
                        while r_rem == 0 {
                            r -= 2;
                            r_rem = char::to_digit(char::from(line[r]), 10).unwrap();
                        }
                    }
                }
                l += 1;
            }
        }
        while l == r && r_rem > 0 {
            // println!("index {} id {} rem {} *", i, r/2, r_rem);
            acc = acc + (i * r / 2) as u64;
            i += 1;
            r_rem -= 1;
        }

        Ok(acc)
    }
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<u64, Error> {
        let lines = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let mut line = lines.first().unwrap().clone();
        let orig_line = lines.first().unwrap().clone();

        let mut l = 0;
        let mut r = line.len() - 1;
        let mut i = 0;

        let mut acc: u64 = 0;

        while l < r {
            let unmoved = char::to_digit(char::from(line[l]), 10).unwrap();
            let orig_left = char::to_digit(char::from(orig_line[l]), 10).unwrap();
            for j in 0..unmoved {
                // println!("index {} id {}", i, l/2);
                acc = acc + (i * l / 2) as u64;
                i += 1;
            }
            if unmoved == 0 {
                i += orig_left as usize;
            }
            l += 1;

            if l < r {
                let mut moved = char::to_digit(char::from(line[l]), 10).unwrap();
                let mut r_inner = r;
                while moved > 0 && r_inner > l {
                    let r_inner_digit = char::to_digit(char::from(line[r_inner]), 10).unwrap();
                    if r_inner_digit > 0 && r_inner_digit <= moved {
                        line[r_inner] = '0'.to_ascii_lowercase() as u8;
                        for j in 0..r_inner_digit {
                            // println!("index {} id {} right {}", i, r_inner / 2, r_inner);
                            acc = acc + (i * (r_inner / 2)) as u64;
                            i += 1;
                        }
                        moved -= r_inner_digit
                    }
                    r_inner -= 2;
                }
                l += 1;
                i += moved as usize;
            }
        }

        Ok(acc)
    }
    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
