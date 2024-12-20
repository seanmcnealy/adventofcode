use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, Lines};
use std::iter::Flatten;
use std::sync::Mutex;

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<(), Error> {
    fn locate<T: PartialEq>(grid: &Vec<Vec<T>>, s: T) -> (usize, usize) {
        let mut agent = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == s {
                    agent = (i, j);
                    break;
                }
            }
        }
        agent
    }
    fn get_location(grid: &Vec<Vec<u8>>, x: usize, y: usize, dx: isize, dy: isize) -> u8 {
        if x as isize + dx < 0
            || (x as isize + dx) as usize >= grid.len()
            || y as isize + dy < 0
            || (y as isize + dy) as usize >= grid[0].len()
        {
            '#' as u8
        } else {
            grid[(x as isize + dx) as usize][(y as isize + dy) as usize]
        }
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let start = locate(&grid, 'S' as u8);
        let goal = locate(&grid, 'E' as u8);

        let mut found: HashSet<(usize, usize)> = HashSet::new();
        found.insert((start.0, start.1));
        let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
        costs.insert((start.0, start.1), 0);
        let mut search = VecDeque::from_iter(vec![(start.0, start.1, vec![(start.0, start.1)])]);
        let mut solution: Vec<(usize, usize)> = vec![];

        while !search.is_empty() {
            let (agent_x, agent_y, path) = search.pop_front().unwrap();
            if agent_x == goal.0 && agent_y == goal.1 {
                // found.insert((agent_x, agent_y));
                // costs.insert((agent_x, agent_y), path.iter().count());
                solution = path;
                break;
            } else {
                if get_location(&grid, agent_x, agent_y, -1, 0) != '#' as u8
                    && !found.contains(&(agent_x - 1, agent_y))
                {
                    found.insert((agent_x - 1, agent_y));
                    costs.insert((agent_x - 1, agent_y), path.iter().count());
                    search.push_back((
                        agent_x - 1,
                        agent_y,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x - 1, agent_y)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 1, 0) != '#' as u8
                    && !found.contains(&(agent_x + 1, agent_y))
                {
                    found.insert((agent_x + 1, agent_y));
                    costs.insert((agent_x + 1, agent_y), path.iter().count());
                    search.push_back((
                        agent_x + 1,
                        agent_y,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x + 1, agent_y)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 0, -1) != '#' as u8
                    && !found.contains(&(agent_x, agent_y - 1))
                {
                    found.insert((agent_x, agent_y - 1));
                    costs.insert((agent_x, agent_y - 1), path.iter().count());
                    search.push_back((
                        agent_x,
                        agent_y - 1,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x, agent_y - 1)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 0, 1) != '#' as u8
                    && !found.contains(&(agent_x, agent_y + 1))
                {
                    found.insert((agent_x, agent_y + 1));
                    costs.insert((agent_x, agent_y + 1), path.iter().count());
                    search.push_back((
                        agent_x,
                        agent_y + 1,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x, agent_y + 1)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
            }
        }

        Ok(solution
            .iter()
            .map(|(x, y)| {
                let mut i: usize = 0;
                if get_location(&grid, *x, *y, -2, 0) != '#' as u8
                    && solution.contains(&(*x - 2, *y))
                {
                    let c1 = costs[&(*x, *y)] as isize;
                    let c2 = costs[&(*x - 2, *y)] as isize;

                    if c2 - c1 - 2 >= 100 {
                        i += 1;
                        // println!("{},{} {}", *x, *y, c2 - c1 - 2);
                    }
                }
                if get_location(&grid, *x, *y, 2, 0) != '#' as u8
                    && solution.contains(&(*x + 2, *y))
                {
                    let c1 = costs[&(*x, *y)] as isize;
                    let c2 = costs[&(*x + 2, *y)] as isize;

                    if c2 - c1 - 2 >= 100 {
                        i += 1;
                        // println!("{},{} {}", *x, *y, c2 - c1 - 2);
                    }
                }
                if get_location(&grid, *x, *y, 0, -2) != '#' as u8
                    && solution.contains(&(*x, *y - 2))
                {
                    let c1 = costs[&(*x, *y)] as isize;
                    let c2 = costs[&(*x, *y - 2)] as isize;

                    if c2 - c1 - 2 >= 100 {
                        i += 1;
                        // println!("{},{} {}", *x, *y, c2 - c1 - 2);
                    }
                }
                if get_location(&grid, *x, *y, 0, 2) != '#' as u8
                    && solution.contains(&(*x, *y + 2))
                {
                    let c1 = costs[&(*x, *y)] as isize;
                    let c2 = costs[&(*x, *y + 2)] as isize;

                    if c2 - c1 - 2 >= 100 {
                        i += 1;
                        // println!("{},{} {}", *x, *y, c2 - c1 - 2);
                    }
                }
                i
            })
            .sum())
    }
    assert_eq!(0, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R, cap: isize) -> Result<usize, Error> {
        let grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let start = locate(&grid, 'S' as u8);
        let goal = locate(&grid, 'E' as u8);

        let mut found: HashSet<(usize, usize)> = HashSet::new();
        found.insert((start.0, start.1));
        let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
        costs.insert((start.0, start.1), 0);
        let mut search = VecDeque::from_iter(vec![(start.0, start.1, vec![(start.0, start.1)])]);
        let mut solution: Vec<(usize, usize)> = vec![];

        while !search.is_empty() {
            let (agent_x, agent_y, path) = search.pop_front().unwrap();
            if agent_x == goal.0 && agent_y == goal.1 {
                // found.insert((agent_x, agent_y));
                // costs.insert((agent_x, agent_y), path.iter().count());
                solution = path;
                break;
            } else {
                if get_location(&grid, agent_x, agent_y, -1, 0) != '#' as u8
                    && !found.contains(&(agent_x - 1, agent_y))
                {
                    found.insert((agent_x - 1, agent_y));
                    costs.insert((agent_x - 1, agent_y), path.iter().count());
                    search.push_back((
                        agent_x - 1,
                        agent_y,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x - 1, agent_y)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 1, 0) != '#' as u8
                    && !found.contains(&(agent_x + 1, agent_y))
                {
                    found.insert((agent_x + 1, agent_y));
                    costs.insert((agent_x + 1, agent_y), path.iter().count());
                    search.push_back((
                        agent_x + 1,
                        agent_y,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x + 1, agent_y)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 0, -1) != '#' as u8
                    && !found.contains(&(agent_x, agent_y - 1))
                {
                    found.insert((agent_x, agent_y - 1));
                    costs.insert((agent_x, agent_y - 1), path.iter().count());
                    search.push_back((
                        agent_x,
                        agent_y - 1,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x, agent_y - 1)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
                if get_location(&grid, agent_x, agent_y, 0, 1) != '#' as u8
                    && !found.contains(&(agent_x, agent_y + 1))
                {
                    found.insert((agent_x, agent_y + 1));
                    costs.insert((agent_x, agent_y + 1), path.iter().count());
                    search.push_back((
                        agent_x,
                        agent_y + 1,
                        path.clone()
                            .iter()
                            .chain(vec![(agent_x, agent_y + 1)].iter())
                            .map(|&x| x)
                            .collect::<Vec<(usize, usize)>>(),
                    ))
                }
            }
        }

        Ok(solution
            .iter()
            .map(|(x1, y1)| {
                solution
                    .iter()
                    .filter(|(x2, y2)| {
                        (*x1 as isize - *x2 as isize).abs() + (*y1 as isize - *y2 as isize).abs()
                            <= 20
                            && costs[&(*x1, *y1)] as isize
                                - costs[&(*x2, *y2)] as isize
                                - ((*x1 as isize - *x2 as isize).abs()
                                    + (*y1 as isize - *y2 as isize).abs())
                                >= cap
                    })
                    .count()
            })
            .sum())
    }
    assert_eq!(285, part2(BufReader::new(TEST1.as_bytes()), 50)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 100)?);
    println!("Result = {}", result);

    Ok(())
}
