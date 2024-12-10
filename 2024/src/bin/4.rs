use std::cmp;
use std::collections::HashSet;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};
use diagonal::{diagonal_pos_neg, diagonal_pos_pos};


const DAY: &str = "4";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"XMAS").unwrap();
        static ref RE2: Regex = Regex::new(r"MAS").unwrap();
    }
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
        let answer = reader.lines().flatten().map(|line| line.into_bytes()).collect::<Vec<Vec<u8>>>();

        let mut c = answer.clone().iter().map (|l| {
            let s = String::from_utf8(l.clone()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r).unwrap();
            let cap = RE.find_iter(s.as_str());
            let cap_r = RE.find_iter(s2.as_str());
            return cap.count() + cap_r.count();
        }).sum::<usize>();

        let mut transposed = transpose(answer.clone());

        c += transposed.iter().map (|l| {
            let s = String::from_utf8(l.clone()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r).unwrap();
            let cap = RE.find_iter(s.as_str());
            let cap_r = RE.find_iter(s2.as_str());
            return cap.count() + cap_r.count();
        }).sum::<usize>();

        let answer_clone = answer.clone();
        let diag = diagonal_pos_pos(&answer_clone);
        c += diag.iter().map (|l| {
            let s = String::from_utf8(l.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let cap = RE.find_iter(s.as_str());
            let cap_r = RE.find_iter(s2.as_str());
            return cap.count() + cap_r.count();
        }).sum::<usize>();

        let diag2 = diagonal_pos_neg(&answer_clone);
        c += diag2.iter().map (|l| {
            let s = String::from_utf8(l.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let cap = RE.find_iter(s.as_str());
            let cap_r = RE.find_iter(s2.as_str());
            return cap.count() + cap_r.count();
        }).sum::<usize>();


        Ok(c)
    }
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize, Error> {
        let answer = reader.lines().flatten().map(|line| line.into_bytes()).collect::<Vec<Vec<u8>>>();

        let mut c: usize = 0;

        let answer_clone = answer.clone();
        let diag = diagonal_pos_pos(&answer_clone);
        let a_s = diag.iter().enumerate().map (|(i, l)| {
            let s = String::from_utf8(l.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let cap = RE2.find_iter(s.as_str());
            let cap_r = RE2.find_iter(s2.as_str());
            let mut loc =  cap.map(|m|
                (answer.len() - 1 + m.start() + 1 - cmp::min(i, answer.len()-1),
                 m.start() + 1 + cmp::max(answer.len()-1, i) - (answer.len()-1))).collect::<Vec<(usize, usize)>>();
            let loc2 = cap_r.map(|m| {
                let m_r = s.len() - 3 - m.start();
                (answer.len() - 1 + m_r + 1 - cmp::min(i, answer.len()-1),
                 m_r + 1 + cmp::max(answer.len()-1, i) - (answer.len()-1))
            }).collect::<Vec<(usize, usize)>>();
            loc.extend(loc2);
            let (x, y) = loc.get(0).unwrap_or(&(0,0));
            // println!("{}: {} {}", i, x, y);
            return loc;
        }).flatten().collect::<HashSet<(usize, usize)>>();

        let diag2 = diagonal_pos_neg(&answer_clone);
        let a2_s = diag2.iter().enumerate().map (|(i, l)| {
            let s = String::from_utf8(l.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let mut r = l.clone();
            r.reverse();
            let s2 = String::from_utf8(r.iter().map(|&&c| c).collect::<Vec<u8>>()).unwrap();
            let cap = RE2.find_iter(s.as_str());
            let cap_r = RE2.find_iter(s2.as_str());
            let mut loc =  cap.map(|m| (
                m.start() + 1 + cmp::max(answer.len()-1, i) - (answer.len()-1),
                cmp::min(i, answer.len()-1) - m.start() - 1)).collect::<Vec<(usize, usize)>>();
            let loc2 = cap_r.map(|m| {
                let m_r = s.len() - 3 - m.start();
                (m_r + 1 + cmp::max(answer.len()-1, i) - (answer.len()-1),
                 cmp::min(i, answer.len()-1) - m_r - 1)
            }).collect::<Vec<(usize, usize)>>();
            loc.extend(loc2);
            return loc;
        }).flatten().collect::<HashSet<(usize, usize)>>();

        Ok(a_s.intersection(&a2_s).count())
    }
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
