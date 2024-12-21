use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::io::{Error, Lines};
use std::iter::Flatten;
use std::sync::Mutex;

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
029A
980A
179A
456A
379A
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref keypad : Vec<Vec<&'static str>> = vec![
            vec![    "",   "<^",   "^",   ">^",  "<^^",  "^^", ">^^",  "<^^^", "^^^", ">^^^",      ">"], // 0
            vec![  "v>",     "",   ">",   ">>",    "^",  ">^", ">>^",    "^^",  ">^^", ">>^^",   ">>v"], // 1
            vec![   "v",    "<",    "",    ">",   "<^",   "^",  ">^",   "<^^",   "^^",  ">^^",    "v>"], // 2
            vec![  "<v",   "<<",   "<",     "",  "<<^",  "<^",   "^",  "<<^^",  "<^^",   "^^",     "v"], // 3
            vec![ "vv>",    "v",  "v>",  "v>>",     "",   ">",  ">>",     "^",   ">^",  ">>^",  ">>vv"], // 4
            vec![  "vv",   "<v",   "v",   "v>",    "<",    "",   ">",    "<^",    "^",   ">^",   "vv>"], // 5
            vec![ "<vv",  "<<v",  "<v",    "v",   "<<",   "<",    "",   "<<^",   "<^",    "^",    "vv"], // 6
            vec!["vvv>",   "vv", "vv>", "vv>>",    "v",  "v>", "v>>",      "",    ">",   ">>", ">>vvv"], // 7
            vec![ "vvv",  "<vv",  "vv",  "vv>",   "<v",   "v",  "v>",     "<",     "",    ">",  "vvv>"], // 8
            vec!["<vvv", "<<vv", "<vv",   "vv",  "<<v",  "<v",   "v",    "<<",    "<",     "",   "vvv"], // 9
            vec![   "<",  "^<<",  "^<",    "^", "^^<<", "^^<",  "^^", "^^^<<", "^^^<",  "^^^",      ""], // A
        ];

        static ref ind: Vec<u8> = vec!['0' as u8, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, '9' as u8, 'A' as u8];

        static ref keypad2 : Vec<Vec<&'static str>> = vec![
            vec![   "", ">^",  ">", ">>", ">>^"], // <
            vec![ "v<",   "",  "v", "v>",   ">"], // ^
            vec![  "<",  "^",   "",  ">",  "^>"], // v
            vec![ "<<", "<^",  "<",   "",   "^"], // >
            vec!["v<<",  "<",  "<v", "v",    ""], // A
        ];

        static ref ind2: Vec<u8> = vec!['<' as u8, '^' as u8, 'v' as u8, '>' as u8, 'A' as u8];
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        // ORDER TEST
        // vec!["<^A", "^<A", "<vA", "v<A", ">^A", "^>A", ">vA", "v>A"].iter().for_each(|goal| {
        vec!["v>>A", ">>vA"].iter().for_each(|goal| {
            let mut agent = 'A' as u8;
            let t = goal
                .chars()
                .map(|m| {
                    let from = ind2.iter().position(|&c| c == agent).unwrap();
                    let to = ind2.iter().position(|&c| c == m as u8).unwrap();

                    agent = m as u8;
                    format!("{}{}", keypad2[from][to], "A")
                })
                .collect::<Vec<String>>()
                .join("");
            println!("{} {}", goal, t)
        });
        // END TEST

        let goals = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let moves = goals
            .iter()
            .map(|goal| {
                let mut agent = 'A' as u8;
                goal.iter()
                    .map(|&m| {
                        let from = ind.iter().position(|&c| c == agent).unwrap();
                        let to = ind.iter().position(|&c| c == m).unwrap();

                        agent = m;
                        format!("{}{}", keypad[from][to], "A")
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        moves.iter().zip(&goals).for_each(|(x, y)| {
            println!("{} {}", x, String::from_utf8(y.clone()).unwrap());
        });

        let moves2 = moves
            .iter()
            .map(|goal| {
                let mut agent = 'A' as u8;
                goal.as_str()
                    .chars()
                    .map(|m| {
                        let from = ind2.iter().position(|&c| c == agent).unwrap();
                        let to = ind2.iter().position(|&c| c == m as u8).unwrap();

                        agent = m as u8;
                        format!("{}{}", keypad2[from][to], "A")
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        moves2.iter().zip(&goals).for_each(|(x, y)| {
            println!("{} {}", x, String::from_utf8(y.clone()).unwrap());
        });

        let moves3 = moves2
            .iter()
            .map(|goal| {
                let mut agent = 'A' as u8;
                goal.chars()
                    .map(|m| {
                        let from = ind2.iter().position(|&c| c == agent).unwrap();
                        let to = ind2.iter().position(|&c| c == m as u8).unwrap();

                        agent = m as u8;
                        format!("{}{}", keypad2[from][to], "A")
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        let answer = moves3
            .iter()
            .zip(goals)
            .map(|(x, y)| {
                println!("{} {}", x, String::from_utf8(y.clone()).unwrap());
                String::from_utf8(y[0..y.len() - 1].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    * x.len()
            })
            .sum::<usize>();

        Ok(answer)
    }
    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    lazy_static! {
        static ref memo: Mutex<HashMap<(u8, u8, usize), usize>> = Mutex::new(HashMap::new());
    }
    fn part2<R: BufRead>(reader: R, iterations: usize) -> Result<usize, Error> {
        let goals = reader
            .lines()
            .flatten()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();

        let moves = goals
            .iter()
            .map(|goal| {
                let mut agent = 'A' as u8;
                goal.iter()
                    .map(|&m| {
                        let from = ind.iter().position(|&c| c == agent).unwrap();
                        let to = ind.iter().position(|&c| c == m).unwrap();

                        agent = m;
                        format!("{}{}", keypad[from][to], "A")
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>();

        fn move_rec(moves: &str, i: usize) -> usize {
            if i == 0 {
                1
            } else {
                let mut agent = 'A' as u8;
                moves
                    .chars()
                    .map(|m| {
                        if memo.lock().unwrap().contains_key(&(agent, m as u8, i)) {
                            let n = *memo.lock().unwrap().get(&(agent, m as u8, i)).unwrap();
                            agent = m as u8;
                            n
                        } else {
                            let from = ind2.iter().position(|&c| c == agent).unwrap();
                            let to = ind2.iter().position(|&c| c == m as u8).unwrap();

                            let answer =
                                move_rec(format!("{}{}", keypad2[from][to], "A").as_str(), i - 1);
                            memo.lock().unwrap().insert((agent, m as u8, i), answer);
                            agent = m as u8;
                            answer
                        }
                    })
                    .sum()
            }
        }

        let answer = moves
            .iter()
            .zip(goals)
            .map(|(x, y)| {
                // println!("{} {}", x, String::from_utf8(y.clone()).unwrap());
                String::from_utf8(y[0..y.len() - 1].to_vec())
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
                    * move_rec(x, iterations + 1)
            })
            .sum::<usize>();

        Ok(answer)
    }
    assert_eq!(126384, part2(BufReader::new(TEST.as_bytes()), 2)?);

    // retest part 1 known value
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    assert_eq!(128962, part2(input_file, 2)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 25)?);
    println!("Result = {}", result);

    Ok(())
}
