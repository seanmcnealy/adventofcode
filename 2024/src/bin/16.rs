use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, Lines};
use std::iter::Flatten;
use std::sync::Mutex;

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
const TEST2: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
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

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let mut grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        #[derive(PartialEq, Eq, Hash, Copy, Clone)]
        enum DIR {
            UP,
            DOWN,
            LEFT,
            RIGHT,
        }
        fn turn_r(x: &DIR) -> DIR {
            match x {
                DIR::UP => DIR::RIGHT,
                DIR::RIGHT => DIR::DOWN,
                DIR::DOWN => DIR::LEFT,
                DIR::LEFT => DIR::UP,
            }
        }
        fn turn_l(x: &DIR) -> DIR {
            match x {
                DIR::UP => DIR::LEFT,
                DIR::RIGHT => DIR::UP,
                DIR::DOWN => DIR::RIGHT,
                DIR::LEFT => DIR::DOWN,
            }
        }
        let start = locate(&grid, 'S' as u8);
        let goal = locate(&grid, 'E' as u8);
        let mut solution = usize::MAX;

        let mut found: HashMap<(usize, usize, DIR), usize> =
            HashMap::from_iter(vec![((start.0, start.1, DIR::RIGHT), 0)]);
        let mut search = VecDeque::from_iter(vec![(start.0, start.1, DIR::RIGHT, 0usize)]);

        while !search.is_empty() {
            let (agent_x, agent_y, agent_dir, cost) = search.pop_front().unwrap();
            if agent_x == goal.0 && agent_y == goal.1 && cost < solution {
                solution = cost;
            } else {
                let forward = (
                    (agent_x as isize
                        + match agent_dir {
                            DIR::UP => -1,
                            DIR::DOWN => 1,
                            _ => 0isize,
                        }) as usize,
                    (agent_y as isize
                        + match agent_dir {
                            DIR::LEFT => -1,
                            DIR::RIGHT => 1,
                            _ => 0,
                        }) as usize,
                    agent_dir.clone(),
                );
                if (grid[forward.0][forward.1] == '.' as u8
                    || grid[forward.0][forward.1] == 'E' as u8)
                    && *found.get(&forward).unwrap_or(&usize::MAX) > cost + 1
                {
                    found.insert(forward, cost + 1);
                    search.push_back((forward.0, forward.1, forward.2, cost + 1));
                }
                let left = (agent_x, agent_y, turn_l(&agent_dir));
                if *found.get(&left).unwrap_or(&usize::MAX) > cost + 1000 {
                    found.insert(left, cost + 1000);
                    search.push_back((left.0, left.1, left.2, cost + 1000));
                }
                let right = (agent_x, agent_y, turn_r(&agent_dir));
                if *found.get(&right).unwrap_or(&usize::MAX) > cost + 1000 {
                    found.insert(right, cost + 1000);
                    search.push_back((right.0, right.1, right.2, cost + 1000));
                }
            }
        }

        Ok(solution)
    }
    assert_eq!(7036, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    #[derive(PartialEq, Eq, Hash, Copy, Clone)]
    enum DIR {
        UP,
        DOWN,
        LEFT,
        RIGHT,
    }

    println!("=== Part 2 ===");
    lazy_static! {
        static ref found2: Mutex<HashMap<(usize, usize, DIR), (usize, Vec<(usize, usize, DIR)>)>> =
            Mutex::new(HashMap::new());
    }
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        found2.lock().unwrap().clear();
        let mut grid = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        fn turn_r(x: &DIR) -> DIR {
            match x {
                DIR::UP => DIR::RIGHT,
                DIR::RIGHT => DIR::DOWN,
                DIR::DOWN => DIR::LEFT,
                DIR::LEFT => DIR::UP,
            }
        }
        fn turn_l(x: &DIR) -> DIR {
            match x {
                DIR::UP => DIR::LEFT,
                DIR::RIGHT => DIR::UP,
                DIR::DOWN => DIR::RIGHT,
                DIR::LEFT => DIR::DOWN,
            }
        }
        let start = locate(&grid, 'S' as u8);
        let goal = locate(&grid, 'E' as u8);
        let mut solution = usize::MAX;

        let mut found: HashMap<(usize, usize, DIR), usize> =
            HashMap::from_iter(vec![((start.0, start.1, DIR::RIGHT), 0)]);
        let mut search = VecDeque::from_iter(vec![(start.0, start.1, DIR::RIGHT, 0usize)]);

        while !search.is_empty() {
            let (agent_x, agent_y, agent_dir, cost) = search.pop_front().unwrap();
            if agent_x == goal.0 && agent_y == goal.1 && cost < solution {
                solution = cost;
            } else {
                let forward = (
                    (agent_x as isize
                        + match agent_dir {
                            DIR::UP => -1,
                            DIR::DOWN => 1,
                            _ => 0isize,
                        }) as usize,
                    (agent_y as isize
                        + match agent_dir {
                            DIR::LEFT => -1,
                            DIR::RIGHT => 1,
                            _ => 0,
                        }) as usize,
                    agent_dir.clone(),
                );
                if (grid[forward.0][forward.1] == '.' as u8
                    || grid[forward.0][forward.1] == 'E' as u8)
                    && *found.get(&forward).unwrap_or(&usize::MAX) > cost + 1
                {
                    found.insert(forward, cost + 1);
                    search.push_back((forward.0, forward.1, forward.2, cost + 1));
                }
                let left = (agent_x, agent_y, turn_l(&agent_dir));
                if *found.get(&left).unwrap_or(&usize::MAX) > cost + 1000 {
                    found.insert(left, cost + 1000);
                    search.push_back((left.0, left.1, left.2, cost + 1000));
                }
                let right = (agent_x, agent_y, turn_r(&agent_dir));
                if *found.get(&right).unwrap_or(&usize::MAX) > cost + 1000 {
                    found.insert(right, cost + 1000);
                    search.push_back((right.0, right.1, right.2, cost + 1000));
                }
            }
        }

        fn recursive_search(
            grid: &Vec<Vec<u8>>,
            goal: (usize, usize),
            solution: usize,
            path: &Vec<(usize, usize, DIR)>,
            agent: (usize, usize, DIR, usize),
        ) -> Vec<(usize, usize, DIR)> {
            let agent_x = agent.0;
            let agent_y = agent.1;
            let agent_dir = agent.2;
            let cost = agent.3;

            let found = {
                found2
                    .lock()
                    .unwrap()
                    .get(&(agent_x, agent_y, agent_dir))
                    .get_or_insert(&(usize::MAX, vec![]))
                    .clone()
            };
            let (found_cost, known_paths) = found;

            if cost > solution || found_cost < cost {
                vec![]
            } else if agent_x == goal.0 && agent_y == goal.1 && cost == solution {
                known_paths
                    .iter()
                    .chain(path.iter())
                    .map(|&x| x)
                    .collect::<Vec<(usize, usize, DIR)>>()
            } else if cost == found_cost {
                let new_paths = known_paths
                    .iter()
                    .chain(path.iter())
                    .map(|&x| x)
                    .collect::<Vec<(usize, usize, DIR)>>();
                found2
                    .lock()
                    .unwrap()
                    .insert((agent_x, agent_y, agent_dir), (cost, new_paths));
                vec![]
            } else {
                found2
                    .lock()
                    .unwrap()
                    .insert((agent_x, agent_y, agent_dir), (cost, path.clone()));

                let forward = (
                    (agent_x as isize
                        + match agent_dir {
                            DIR::UP => -1,
                            DIR::DOWN => 1,
                            _ => 0isize,
                        }) as usize,
                    (agent_y as isize
                        + match agent_dir {
                            DIR::LEFT => -1,
                            DIR::RIGHT => 1,
                            _ => 0,
                        }) as usize,
                    agent_dir.clone(),
                    cost + 1,
                );

                let left = (
                    (agent_x as isize
                        + match turn_l(&agent_dir) {
                            DIR::UP => -1,
                            DIR::DOWN => 1,
                            _ => 0isize,
                        }) as usize,
                    (agent_y as isize
                        + match turn_l(&agent_dir) {
                            DIR::LEFT => -1,
                            DIR::RIGHT => 1,
                            _ => 0,
                        }) as usize,
                    turn_l(&agent_dir).clone(),
                    cost + 1001,
                );

                let right = (
                    (agent_x as isize
                        + match turn_r(&agent_dir) {
                            DIR::UP => -1,
                            DIR::DOWN => 1,
                            _ => 0isize,
                        }) as usize,
                    (agent_y as isize
                        + match turn_r(&agent_dir) {
                            DIR::LEFT => -1,
                            DIR::RIGHT => 1,
                            _ => 0,
                        }) as usize,
                    turn_r(&agent_dir).clone(),
                    cost + 1001,
                );

                if grid[forward.0][forward.1] == '.' as u8
                    || grid[forward.0][forward.1] == 'E' as u8
                {
                    let mut forward_path = path.clone();
                    forward_path.push((forward.0, forward.1, forward.2));
                    recursive_search(grid, goal, solution, &forward_path, forward)
                } else {
                    vec![]
                }
                .iter()
                .chain(
                    if grid[left.0][left.1] == '.' as u8 || grid[left.0][left.1] == 'E' as u8 {
                        let mut left_path = path.clone();
                        left_path.push((left.0, left.1, left.2));
                        recursive_search(grid, goal, solution, &left_path, left)
                    } else {
                        vec![]
                    }
                    .iter(),
                )
                .chain(
                    if grid[right.0][right.1] == '.' as u8 || grid[right.0][right.1] == 'E' as u8 {
                        let mut right_path = path.clone();
                        right_path.push((right.0, right.1, right.2));
                        recursive_search(grid, goal, solution, &right_path, right)
                    } else {
                        vec![]
                    }
                    .iter(),
                )
                .map(|&x| x)
                .collect()
            }
        }

        let paths = recursive_search(
            &grid,
            goal,
            solution,
            &vec![(start.0, start.1, DIR::RIGHT)],
            (start.0, start.1, DIR::RIGHT, 0),
        );

        let answer = paths
            .iter()
            .map(|&x| x)
            .collect::<HashSet<(usize, usize, DIR)>>()
            .iter()
            .map(|(x, y, dir)| {
                found2
                    .lock()
                    .unwrap()
                    .get(&(*x, *y, *dir))
                    .unwrap_or(&(0, vec![]))
                    .clone()
            })
            .map(|(_, paths)| paths)
            .flatten()
            .collect::<HashSet<(usize, usize, DIR)>>()
            .iter()
            .map(|(x, y, dir)| (*x, *y))
            .collect::<HashSet<(usize, usize)>>()
            .iter()
            .count();

        // Ok(paths.iter().map(|&x| x).collect::<HashSet<(usize, usize, DIR)>>()
        //     .iter()
        //     .len())
        Ok(answer + 1)
    }
    assert_eq!(45, part2(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(64, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
