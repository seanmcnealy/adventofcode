use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<i32, Error> {
        let mut grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        fn solve_group(grid_ref: &Vec<Vec<u8>>, x: usize, y: usize) -> (Vec<Vec<u8>>, i32) {
            let mut grid = grid_ref.clone();
            let name = grid[x][y];
            let mut found = vec![(x, y)];
            let mut search = vec![(x, y)];

            let mut perimiter: i32 = 4;
            let mut area: i32 = 1;

            while !search.is_empty() {
                let n = search.pop().unwrap();
                vec![
                    (n.0 as isize - 1, n.1 as isize),
                    (n.0 as isize + 1, n.1 as isize),
                    (n.0 as isize, n.1 as isize - 1),
                    (n.0 as isize, n.1 as isize + 1),
                ]
                .iter()
                .filter(|(x, y)| {
                    *x >= 0
                        && (*x as usize) < grid.len()
                        && *y >= 0
                        && (*y as usize) < grid[0].len()
                })
                .map(|(x, y)| (*x as usize, *y as usize))
                .for_each(|(x, y)| {
                    if grid[x][y] == name && !found.contains(&(x, y)) {
                        perimiter += 4 - 2 * vec![
                            (x as isize - 1, y as isize),
                            (x as isize + 1, y as isize),
                            (x as isize, y as isize - 1),
                            (x as isize, y as isize + 1),
                        ]
                        .iter()
                        .filter(|(x, y)| found.contains(&(*x as usize, *y as usize)))
                        .count() as i32;
                        area += 1;
                        search.push((x, y));
                        found.push((x, y));
                    }
                })
            }

            found.iter().for_each(|(x, y)| grid[*x][*y] = '.' as u8);

            (grid, perimiter * area)
        }

        let mut acc = 0;
        loop {
            let mut x = 0;
            let mut y = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] != '.' as u8 {
                        x = i;
                        y = j;
                    }
                }
            }
            if x == 0 && y == 0 {
                break;
            }
            let (g, a) = solve_group(&grid, x, y);
            grid = g;
            acc += a;
        }
        Ok(acc)
    }
    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<i32, Error> {
        let mut grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        fn solve_group(grid_ref: &Vec<Vec<u8>>, x: usize, y: usize) -> (Vec<Vec<u8>>, i32) {
            let mut grid = grid_ref.clone();
            let name = grid[x][y];
            let mut found = vec![(x, y)];
            let mut search = vec![(x, y)];

            while !search.is_empty() {
                let n = search.pop().unwrap();
                vec![
                    (n.0 as isize - 1, n.1 as isize),
                    (n.0 as isize + 1, n.1 as isize),
                    (n.0 as isize, n.1 as isize - 1),
                    (n.0 as isize, n.1 as isize + 1),
                ]
                .iter()
                .filter(|(x, y)| {
                    *x >= 0
                        && (*x as usize) < grid.len()
                        && *y >= 0
                        && (*y as usize) < grid[0].len()
                })
                .map(|(x, y)| (*x as usize, *y as usize))
                .for_each(|(x, y)| {
                    if grid[x][y] == name && !found.contains(&(x, y)) {
                        search.push((x, y));
                        found.push((x, y));
                    }
                })
            }

            found.iter().for_each(|(x, y)| grid[*x][*y] = '.' as u8);

            fn get_name(grid_ref: &Vec<Vec<u8>>, x: usize, y: usize, i: isize, j: isize) -> u8 {
                if x as isize + i >= 0
                    && ((x as isize + i) as usize) < grid_ref.len()
                    && y as isize + j >= 0
                    && ((y as isize + j) as usize) < grid_ref[0].len()
                {
                    grid_ref[(x as isize + i) as usize][(y as isize + j) as usize]
                } else {
                    '.' as u8
                }
            }

            fn corners(grid_ref: &Vec<Vec<u8>>, name: u8, x: usize, y: usize) -> i32 {
                let mut m = 0;
                if (get_name(grid_ref, x, y, -1, 0) != name
                    && get_name(grid_ref, x, y, 0, -1) != name)
                    || (get_name(grid_ref, x, y, -1, 0) == name
                        && get_name(grid_ref, x, y, 0, -1) == name
                        && get_name(grid_ref, x, y, -1, -1) != name)
                {
                    m += 1
                }
                if (get_name(grid_ref, x, y, -1, 0) != name
                    && get_name(grid_ref, x, y, 0, 1) != name)
                    || (get_name(grid_ref, x, y, -1, 0) == name
                        && get_name(grid_ref, x, y, 0, 1) == name
                        && get_name(grid_ref, x, y, -1, 1) != name)
                {
                    m += 1
                }
                if (get_name(grid_ref, x, y, 1, 0) != name
                    && get_name(grid_ref, x, y, 0, -1) != name)
                    || (get_name(grid_ref, x, y, 1, 0) == name
                        && get_name(grid_ref, x, y, 0, -1) == name
                        && get_name(grid_ref, x, y, 1, -1) != name)
                {
                    m += 1
                }
                if (get_name(grid_ref, x, y, 1, 0) != name
                    && get_name(grid_ref, x, y, 0, 1) != name)
                    || (get_name(grid_ref, x, y, 1, 0) == name
                        && get_name(grid_ref, x, y, 0, 1) == name
                        && get_name(grid_ref, x, y, 1, 1) != name)
                {
                    m += 1
                }
                m
            }

            (
                grid,
                found
                    .iter()
                    .map(|(x, y)| corners(grid_ref, name, *x, *y))
                    .sum::<i32>()
                    * found.len() as i32,
            )
        }

        let mut acc = 0;
        loop {
            let mut x = 0;
            let mut y = 0;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] != '.' as u8 {
                        x = i;
                        y = j;
                    }
                }
            }
            if x == 0 && y == 0 {
                break;
            }
            let (g, a) = solve_group(&grid, x, y);
            grid = g;
            acc += a;
        }
        Ok(acc)
    }
    assert_eq!(1206, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
