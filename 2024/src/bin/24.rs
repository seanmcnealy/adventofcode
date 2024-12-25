use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use lazy_static::lazy_static;
use nalgebra::SimdBool;
use rand::random;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, Lines};
use std::iter::Flatten;

const DAY: &str = "24";
const INPUT_FILE: &str = concatcp!("data/", DAY);
const TEST1: &str = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
const TEST2: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
const TEST3: &str = "\
x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00
";

fn main() -> Result<(), Error> {
    lazy_static! {
        // x00: 1
        static ref REGEX1: Regex = Regex::new(r"^([a-z0-9]{3}): ([0-1])$").unwrap();
        // ntg XOR fgs -> mjb
        static ref REGEX2: Regex = Regex::new(r"^([a-z0-9]{3}) (XOR|AND|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})$").unwrap();
    }
    fn parse_line1(line: &String) -> Option<(String, u8)> {
        REGEX1
            .captures(line.as_str())
            .map(|c| (c[1].to_string(), c[2].to_string().parse::<u8>().unwrap()))
    }
    fn parse_line2(line: &String) -> Option<(String, String, String, String)> {
        REGEX2.captures(line.as_str()).map(|c| {
            (
                c[1].to_string(),
                c[2].to_string(),
                c[3].to_string(),
                c[4].to_string(),
            )
        })
    }
    println!("=== Part 1 ===");
    fn part1<R: BufRead>(reader: R, output_count: usize) -> Result<u64, Error> {
        let (input_reader, gate_reader): (Vec<_>, Vec<_>) =
            reader.lines().flatten().partition(|r| r.contains(":"));

        let inputs = input_reader
            .iter()
            .map(parse_line1)
            .flatten()
            .collect::<Vec<(String, u8)>>();

        let gates = gate_reader
            .iter()
            .map(parse_line2)
            .flatten()
            .collect::<Vec<(String, String, String, String)>>();

        let input_map: HashMap<String, u8> = inputs.iter().map(|x| x.clone()).collect();

        let gate_map: HashMap<String, (String, String, String)> = gates
            .iter()
            .map(|(x1, op, x2, y)| (y.clone(), (x1.clone(), op.clone(), x2.clone())))
            .collect();

        fn get_output(
            input_map: &HashMap<String, u8>,
            gate_map: &HashMap<String, (String, String, String)>,
            gate_name: String,
        ) -> bool {
            if input_map.contains_key(&gate_name) {
                input_map[&gate_name] == 1
            } else {
                let (x1_name, op, x2_name) = gate_map[&gate_name].clone();
                let x1 = get_output(input_map, gate_map, x1_name);
                let x2 = get_output(input_map, gate_map, x2_name);
                match op.as_str() {
                    "AND" => x1 && x2,
                    "OR" => x1 || x2,
                    "XOR" => x1 ^ x2,
                    _ => false,
                }
            }
        }

        Ok((0..output_count)
            .map(|z| get_output(&input_map, &gate_map, format!("z{:02}", z)))
            .rev()
            .fold(0u64, |acc, x| if x { (acc << 1) + 1 } else { acc << 1 }))
    }
    assert_eq!(4, part1(BufReader::new(TEST1.as_bytes()), 3)?);
    assert_eq!(2024, part1(BufReader::new(TEST2.as_bytes()), 12)?);

    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part1(input_file, 46)?);
    // println!("Result = {}", result);

    println!("=== Part 2 ===");
    fn part2<R: BufRead>(
        reader: R,
        input_count: usize,
        output_count: usize,
        goals: Vec<(u64, u64, u64)>,
    ) -> Result<String, Error> {
        let (_, gate_reader): (Vec<_>, Vec<_>) =
            reader.lines().flatten().partition(|r| r.contains(":"));

        let gates = gate_reader
            .iter()
            .map(parse_line2)
            .flatten()
            .collect::<Vec<(String, String, String, String)>>();

        let gate_map: HashMap<String, (String, String, String)> = gates
            .iter()
            .map(|(x1, op, x2, y)| (y.clone(), (x1.clone(), op.clone(), x2.clone())))
            .collect();

        fn get_output(
            input_map: &HashMap<String, u8>,
            cycle_check: Vec<String>,
            gate_map: &HashMap<String, (String, String, String)>,
            swap1: &String,
            swap2: &String,
            gate_name: String,
        ) -> Option<bool> {
            let my_gate_name = if gate_name == *swap1 {
                swap2.clone()
            } else if gate_name == *swap2 {
                swap1.clone()
            } else {
                gate_name
            };
            if cycle_check.contains(&my_gate_name) {
                None
            } else {
                if input_map.contains_key(&my_gate_name) {
                    Some(input_map[&my_gate_name] == 1)
                } else {
                    let (x1_name, op, x2_name) = gate_map[&my_gate_name].clone();
                    get_output(
                        input_map,
                        cycle_check
                            .iter()
                            .map(|x| x.clone())
                            .chain(vec![my_gate_name.clone()])
                            .map(|x| x.clone())
                            .collect(),
                        gate_map,
                        swap1,
                        swap2,
                        x1_name,
                    )
                    .map(|x1| {
                        get_output(
                            input_map,
                            cycle_check
                                .iter()
                                .map(|x| x.clone())
                                .chain(vec![my_gate_name.clone()])
                                .map(|x| x.clone())
                                .collect(),
                            gate_map,
                            swap1,
                            swap2,
                            x2_name,
                        )
                        .map(|x2| match op.as_str() {
                            "AND" => x1 && x2,
                            "OR" => x1 || x2,
                            "XOR" => x1 ^ x2,
                            _ => false,
                        })
                    })
                    .flatten()
                }
            }
        }

        let test_gates = |gate_map: &HashMap<String, (String, String, String)>,
                          input_map: &HashMap<String, u8>,
                          swap1: &String,
                          swap2: &String| {
            (0..output_count)
                .map(|z| {
                    get_output(
                        &input_map,
                        vec![],
                        &gate_map,
                        swap1,
                        swap2,
                        format!("z{:02}", z),
                    )
                })
                .rev()
                .fold(Some(0u64), |opt_acc, opt_x| {
                    opt_acc
                        .map(|acc| opt_x.map(|x| if x { (acc << 1) + 1 } else { acc << 1 }))
                        .flatten()
                })
        };

        fn create_input(input_size: usize, i: u64, label: char) -> Vec<(String, u8)> {
            (0..input_size)
                .map(|x| {
                    if (1 << x) & i > 0 {
                        (format!("{}{:02}", label, x), 1u8)
                    } else {
                        (format!("{}{:02}", label, x), 0u8)
                    }
                })
                .collect()
        }

        fn create_input_2(input_size: usize, x: u64, y: u64) -> HashMap<String, u8> {
            create_input(input_size, x, 'x')
                .iter()
                .chain(create_input(input_size, y, 'y').iter())
                .map(|x| x.clone())
                .collect::<HashMap<String, u8>>()
        }

        fn print_rules(
            input_map: &HashMap<String, u8>,
            gate_map: &HashMap<String, (String, String, String)>,
            gate_name: String,
            depth: usize,
        ) -> String {
            if depth >= 4 || input_map.contains_key(&gate_name) {
                gate_name
            } else {
                let (x1_name, op, x2_name) = gate_map[&gate_name].clone();
                let x1 = print_rules(input_map, gate_map, x1_name, depth + 1);
                let x2 = print_rules(input_map, gate_map, x2_name, depth + 1);
                format!("({} {} {})", x1, op, x2)
            }
        }
        (0..output_count).for_each(|z| {
            println!(
                "{}",
                print_rules(
                    &create_input_2(input_count, 0, 0),
                    &gate_map,
                    format!("z{:02}", z),
                    0
                )
            )
        });

        let pairs: Vec<(String, String)> = gate_map
            .keys()
            .flat_map(|x| gate_map.keys().map(|y| (x.clone(), y.clone())))
            .collect();
        let goals_and_wrong: Vec<(u64, u64, u64, u64, HashMap<String, u8>)> = goals
            .iter()
            .map(|(x1, x2, g)| {
                let inp = create_input_2(input_count, *x1, *x2);
                test_gates(&gate_map, &inp, &"test".to_string(), &"test".to_string())
                    .map(|w| (*x1, *x2, *g, w ^ *g, inp))
            })
            .flatten()
            .collect();

        let percent_done = (pairs.iter().count() / 100) + 1;
        let answer = pairs
            .iter()
            .enumerate()
            .filter(|(i, (s1, s2))| {
                if (i % percent_done) == 0 {
                    println!("{}%", i / percent_done);
                }
                s1 > s2
                    && goals_and_wrong.iter().all(|(x1, x2, g, wrong, inp)| {
                        {
                            test_gates(&gate_map, &inp, s1, s2)
                                .map(|t| (*g ^ t).count_ones() <= wrong.count_ones())
                        }
                        .unwrap_or(false)
                    })
                    && goals_and_wrong.iter().any(|(x1, x2, g, wrong, inp)| {
                        {
                            test_gates(&gate_map, &inp, s1, s2)
                                .map(|t| (*g ^ t).count_ones() < wrong.count_ones())
                        }
                        .unwrap_or(false)
                    })
            })
            .flat_map(|(_, (x, y))| {
                println!("{} {}", x, y);
                vec![x.clone(), y.clone()]
            })
            .collect::<HashSet<String>>();

        Ok(answer.iter().sorted().join(","))
    }
    assert_eq!(
        "z00,z01,z02,z05",
        part2(
            BufReader::new(TEST3.as_bytes()),
            6,
            6,
            vec![
                (0b101010, 0b101100, 0b101000),
                (0b111000, 0b000111, 0b000000),
                (0b011100, 0b000111, 0b000100),
                (0b000001, 0b111111, 0b000001),
                (0b000010, 0b111111, 0b000010),
                (0b000100, 0b111111, 0b000100),
                (0b001000, 0b111111, 0b001000),
                (0b010000, 0b111111, 0b010000),
            ]
        )?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let tests: Vec<(u64, u64, u64)> = (0..512)
        .map(|_| {
            let a: u64 = random::<u64>() & 0x1FFFFFFFFFFFu64;
            let b: u64 = random::<u64>() & 0x1FFFFFFFFFFFu64;
            (a, b, a + b)
        })
        .collect();
    let result = time_snippet!(part2(input_file, 45, 46, tests)?);
    println!("Result = {}", result);

    Ok(())
}
