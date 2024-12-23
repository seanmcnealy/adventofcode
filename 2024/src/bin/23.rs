use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

const DAY: &str = "23";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

fn main() -> Result<(), Error> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^([a-z]+)-([a-z]+)$").unwrap();
    }
    fn parse_line(line: String) -> Option<(String, String)> {
        REGEX
            .captures(line.as_str())
            .map(|c| (c[1].to_string(), c[2].to_string()))
    }

    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R) -> Result<usize, Error> {
        let connections_list = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(String, String)>>();

        let connections_map = connections_list
            .iter()
            .map(|x| x.clone())
            .chain(
                connections_list
                    .iter()
                    .map(|(s1, s2)| (s2.clone(), s1.clone())),
            )
            .into_group_map();

        let groups_of_three = connections_map.iter().flat_map(|(n1, n2s)| {
            n2s.iter().flat_map(|n2| {
                connections_map[n2].iter().flat_map(|n3| {
                    if n2s.contains(n3)
                        && (n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t'))
                    {
                        Some((n1.clone(), n2.clone(), n3.clone()))
                    } else {
                        None
                    }
                })
            })
        });

        Ok(groups_of_three.count() / 6)
    }
    assert_eq!(7, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<String, Error> {
        let connections_list = reader
            .lines()
            .flatten()
            .map(parse_line)
            .flatten()
            .collect::<Vec<(String, String)>>();

        let connections_map = connections_list
            .iter()
            .map(|x| x.clone())
            .chain(
                connections_list
                    .iter()
                    .map(|(s1, s2)| (s2.clone(), s1.clone())),
            )
            .into_group_map();

        let mut searched: HashSet<String> = HashSet::new();
        let mut largest: Vec<String> = vec![
            connections_list.first().unwrap().clone().0,
            connections_list.first().unwrap().clone().1,
        ];

        fn grow_group(
            connections_map: &HashMap<String, Vec<String>>,
            group: Vec<String>,
        ) -> Vec<Vec<String>> {
            group
                .iter()
                .map(|c| connections_map[c].clone())
                .fold(
                    connections_map[&group[0].clone()]
                        .iter()
                        .map(|x| x.clone())
                        .collect::<HashSet<String>>(),
                    |acc, c| {
                        acc.intersection(&c.iter().map(|x| x.clone()).collect::<HashSet<String>>())
                            .map(|x| x.clone())
                            .collect()
                    },
                )
                .iter()
                .filter(|s| !group.contains(s))
                .map(|s| group.iter().chain(vec![s]).map(|s| s.clone()).collect())
                .collect()
        }

        connections_map.iter().for_each(|(n1, n2s)| {
            let mut groups = VecDeque::from_iter(n2s.iter().map(|n2| vec![n1.clone(), n2.clone()]));
            while !groups.is_empty() {
                for grown in grow_group(&connections_map, groups.pop_front().unwrap()) {
                    let mut grown_sorted = grown;
                    grown_sorted.sort();
                    if grown_sorted.len() > largest.len() {
                        largest = grown_sorted.clone();
                    }
                    let key = grown_sorted.join(",");
                    if !searched.contains(&key) {
                        groups.push_back(grown_sorted);
                        searched.insert(key);
                    }
                }
            }
        });

        largest.sort();

        Ok(largest.iter().join(","))
    }
    assert_eq!("co,de,ka,ta", part2(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
