use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "8";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let mut towers: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let t = grid[x][y];
                if t != '.' as u8 {
                    towers.entry(t).or_insert(Vec::new()).push((x, y))
                }
            }
        }

        fn pairs<T: Clone, R>(vec: &Vec<T>, f: &impl Fn(T, T) -> R) -> Vec<R> {
            if vec.is_empty() {
                vec![]
            } else {
                let x = vec.first().unwrap();
                vec[1..]
                    .iter()
                    .map(|i| f(x.clone(), i.clone()))
                    .chain(pairs(&vec[1..].to_vec(), f).into_iter().collect::<Vec<R>>())
                    .collect::<Vec<R>>()
            }
        }

        let nodes = towers
            .values()
            .map(|t| {
                pairs::<(usize, usize), Vec<(usize, usize)>>(t, &|x, y| {
                    let diff = (x.0 as isize - y.0 as isize, x.1 as isize - y.1 as isize);
                    let n1 = (x.0 as isize + diff.0, x.1 as isize + diff.1);
                    let n2 = (y.0 as isize - diff.0, y.1 as isize - diff.1);
                    vec![n1, n2]
                        .iter()
                        .filter(|n| {
                            n.0 >= 0
                                && (n.0 as usize) < grid.len()
                                && n.1 >= 0
                                && (n.1 as usize) < grid[0].len()
                        })
                        .map(|&z| (z.0 as usize, z.1 as usize))
                        .collect::<Vec<(usize, usize)>>()
                })
            })
            .flatten()
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        Ok(nodes
            .iter()
            .map(|&x| x)
            .collect::<HashSet<(usize, usize)>>()
            .iter()
            .len())
    }
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let mut towers: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                let t = grid[x][y];
                if t != '.' as u8 {
                    towers.entry(t).or_insert(Vec::new()).push((x, y))
                }
            }
        }

        fn pairs<T: Clone, R>(vec: &Vec<T>, f: &impl Fn(T, T) -> R) -> Vec<R> {
            if vec.is_empty() {
                vec![]
            } else {
                let x = vec.first().unwrap();
                vec[1..]
                    .iter()
                    .map(|i| f(x.clone(), i.clone()))
                    .chain(pairs(&vec[1..].to_vec(), f).into_iter().collect::<Vec<R>>())
                    .collect::<Vec<R>>()
            }
        }

        let nodes = towers
            .values()
            .map(|t| {
                pairs::<(usize, usize), Vec<(usize, usize)>>(t, &|x, y| {
                    let diff = (x.0 as isize - y.0 as isize, x.1 as isize - y.1 as isize);
                    (0..100)
                        .map(|i| (x.0 as isize + diff.0 * i, x.1 as isize + diff.1 * i))
                        .take_while(|n| {
                            n.0 >= 0
                                && (n.0 as usize) < grid.len()
                                && n.1 >= 0
                                && (n.1 as usize) < grid[0].len()
                        })
                        .chain(
                            (0..100)
                                .map(|i| (y.0 as isize - diff.0 * i, y.1 as isize - diff.1 * i))
                                .take_while(|n| {
                                    n.0 >= 0
                                        && (n.0 as usize) < grid.len()
                                        && n.1 >= 0
                                        && (n.1 as usize) < grid[0].len()
                                }),
                        )
                        .map(|z| (z.0 as usize, z.1 as usize))
                        .collect::<Vec<(usize, usize)>>()
                })
            })
            .flatten()
            .flatten()
            .collect::<Vec<(usize, usize)>>();

        Ok(nodes
            .iter()
            .map(|&x| x)
            .collect::<HashSet<(usize, usize)>>()
            .iter()
            .len())
    }
    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
