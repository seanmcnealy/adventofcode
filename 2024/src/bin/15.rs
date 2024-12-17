use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, Lines};
use std::iter::Flatten;

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
const TEST2: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn main() -> Result<(), Error> {
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let (grid_reader, instruction_reader): (Vec<_>, Vec<_>) =
            reader.lines().flatten().partition(|r| r.contains("#"));

        let mut grid = grid_reader
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let instructions = instruction_reader
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .flatten()
            .map(|&x| x)
            .collect::<Vec<u8>>();

        let mut agent = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' as u8 {
                    agent = (i, j);
                    break;
                }
            }
        }
        grid[agent.0][agent.1] = '.' as u8;

        fn m(
            grid_ref: &Vec<Vec<u8>>,
            x: usize,
            y: usize,
            dx: isize,
            dy: isize,
        ) -> (Vec<Vec<u8>>, bool) {
            let mut grid = grid_ref.clone();
            if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == '.' as u8 {
                grid[(x as isize + dx) as usize][(y as isize + dy) as usize] = grid[x][y];
                (grid, true)
            } else if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == 'O' as u8 {
                let (mut inner_grid, worked) = m(
                    &grid,
                    (x as isize + dx) as usize,
                    (y as isize + dy) as usize,
                    dx,
                    dy,
                );
                if worked {
                    inner_grid[(x as isize + dx) as usize][(y as isize + dy) as usize] =
                        inner_grid[x][y];
                    (inner_grid, true)
                } else {
                    (inner_grid, false)
                }
            } else {
                (grid, false)
            }
        }

        for i in instructions {
            if match i as char {
                '^' => {
                    let (next, worked) = m(&grid, agent.0, agent.1, -1, 0);
                    grid = next;
                    if worked {
                        agent = (agent.0 - 1, agent.1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                '<' => {
                    let (next, worked) = m(&grid, agent.0, agent.1, 0, -1);
                    grid = next;
                    if worked {
                        agent = (agent.0, agent.1 - 1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                '>' => {
                    let (next, worked) = m(&grid, agent.0, agent.1, 0, 1);
                    grid = next;
                    if worked {
                        agent = (agent.0, agent.1 + 1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                'v' => {
                    let (next, worked) = m(&grid, agent.0, agent.1, 1, 0);
                    grid = next;
                    if worked {
                        agent = (agent.0 + 1, agent.1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            } {
                grid[agent.0][agent.1] = '.' as u8;
            }

            // println!("{}", i as char);
            // for l in grid.clone() {
            //     println!("{}", String::from_utf8(l).unwrap());
            // }
        }

        let mut acc = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'O' as u8 {
                    acc += i * 100 + j
                }
            }
        }

        Ok(acc)
    }
    assert_eq!(2028, part1(BufReader::new(TEST1.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        let (grid_reader, instruction_reader): (Vec<_>, Vec<_>) =
            reader.lines().flatten().partition(|r| r.contains("#"));

        let mut grid = grid_reader
            .iter()
            .map(|line| {
                line.as_bytes()
                    .to_vec()
                    .iter()
                    .map(|&c| match c as char {
                        '#' => "##".as_bytes().to_vec(),
                        'O' => "[]".as_bytes().to_vec(),
                        '.' => "..".as_bytes().to_vec(),
                        '@' => "@.".as_bytes().to_vec(),
                        _ => "..".as_bytes().to_vec(),
                    })
                    .collect::<Vec<Vec<u8>>>()
                    .iter()
                    .flatten()
                    .map(|&x| x)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let instructions = instruction_reader
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>()
            .iter()
            .flatten()
            .map(|&x| x)
            .collect::<Vec<u8>>();

        let mut agent = (0, 0);
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' as u8 {
                    agent = (i, j);
                    break;
                }
            }
        }
        grid[agent.0][agent.1] = '.' as u8;

        fn can_m(grid_ref: &Vec<Vec<u8>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
            let mut grid = grid_ref.clone();
            if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == '.' as u8 {
                grid[(x as isize + dx) as usize][(y as isize + dy) as usize] = grid[x][y];
                true
            } else if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == '[' as u8 {
                let worked = can_m(
                    &grid,
                    (x as isize + dx) as usize,
                    (y as isize + dy) as usize,
                    dx,
                    dy,
                );
                let worked2 = if dx != 0 {
                    can_m(
                        &grid,
                        (x as isize + dx) as usize,
                        (y as isize + dy + 1) as usize,
                        dx,
                        dy,
                    )
                } else {
                    true
                };
                if worked && worked2 {
                    true
                } else {
                    false
                }
            } else if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == ']' as u8 {
                let worked = can_m(
                    &grid,
                    (x as isize + dx) as usize,
                    (y as isize + dy) as usize,
                    dx,
                    dy,
                );
                let worked2 = if dx != 0 {
                    can_m(
                        &grid,
                        (x as isize + dx) as usize,
                        (y as isize + dy - 1) as usize,
                        dx,
                        dy,
                    )
                } else {
                    true
                };
                if worked && worked2 {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        fn do_m(grid_ref: &Vec<Vec<u8>>, x: usize, y: usize, dx: isize, dy: isize) -> Vec<Vec<u8>> {
            let mut grid = grid_ref.clone();
            if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == '.' as u8 {
                grid[(x as isize + dx) as usize][(y as isize + dy) as usize] = grid[x][y];
                grid
            } else if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == '[' as u8 {
                let mut inner_grid = do_m(
                    &grid,
                    (x as isize + dx) as usize,
                    (y as isize + dy + 1) as usize,
                    dx,
                    dy,
                );
                let mut inner_grid2 = if (dx != 0) {
                    do_m(
                        &inner_grid,
                        (x as isize + dx) as usize,
                        (y as isize + dy) as usize,
                        dx,
                        dy,
                    )
                } else {
                    inner_grid[(x as isize + dx) as usize][(y as isize + dy * 2) as usize] =
                        inner_grid[(x as isize + dx) as usize][(y as isize + dy) as usize];
                    inner_grid
                };
                if dx != 0 {
                    inner_grid2[(x as isize + dx) as usize][(y as isize + dy + 1) as usize] =
                        '.' as u8;
                }
                inner_grid2[(x as isize + dx) as usize][(y as isize + dy) as usize] =
                    inner_grid2[x][y];
                inner_grid2
            } else if grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == ']' as u8 {
                let mut inner_grid = do_m(
                    &grid,
                    (x as isize + dx) as usize,
                    (y as isize + dy - 1) as usize,
                    dx,
                    dy,
                );
                let mut inner_grid2 = if dx != 0 {
                    do_m(
                        &inner_grid,
                        (x as isize + dx) as usize,
                        (y as isize + dy) as usize,
                        dx,
                        dy,
                    )
                } else {
                    inner_grid[(x as isize + dx) as usize][(y as isize + dy * 2) as usize] =
                        inner_grid[(x as isize + dx) as usize][(y as isize + dy) as usize];
                    inner_grid
                };
                if dx != 0 {
                    inner_grid2[(x as isize + dx) as usize][(y as isize + dy - 1) as usize] =
                        '.' as u8;
                }
                inner_grid2[(x as isize + dx) as usize][(y as isize + dy) as usize] =
                    inner_grid2[x][y];
                inner_grid2
            } else {
                grid
            }
        }

        for i in instructions {
            if match i as char {
                '^' => {
                    let worked = can_m(&grid, agent.0, agent.1, -1, 0);
                    if worked {
                        let next = do_m(&grid, agent.0, agent.1, -1, 0);
                        grid = next;
                        agent = (agent.0 - 1, agent.1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                '<' => {
                    let worked = can_m(&grid, agent.0, agent.1, 0, -1);
                    if worked {
                        let next = do_m(&grid, agent.0, agent.1, 0, -1);
                        grid = next;
                        agent = (agent.0, agent.1 - 1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                '>' => {
                    let worked = can_m(&grid, agent.0, agent.1, 0, 1);
                    if worked {
                        let next = do_m(&grid, agent.0, agent.1, 0, 1);
                        grid = next;
                        agent = (agent.0, agent.1 + 1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                'v' => {
                    let worked = can_m(&grid, agent.0, agent.1, 1, 0);
                    if worked {
                        let next = do_m(&grid, agent.0, agent.1, 1, 0);
                        grid = next;
                        agent = (agent.0 + 1, agent.1);
                        grid[agent.0][agent.1] = '.' as u8;
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            } {
                grid[agent.0][agent.1] = '.' as u8;
            }

            // println!("{}", i as char);
            // for l in grid.clone() {
            //     println!("{}", String::from_utf8(l).unwrap());
            // }
        }

        let mut acc = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '[' as u8 {
                    acc += i * 100 + j
                }
            }
        }

        Ok(acc)
    }
    assert_eq!(9021, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
