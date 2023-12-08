#![allow(unused)]

use std::collections::HashMap;

use regex::Regex;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day08.in");
    let result = run1(input);
    println!("{result}");
}

type StepsMap = HashMap<String, (String, String)>;

fn parse_input(input: &str) -> (Vec<char>, StepsMap) {
    let mut map: StepsMap = HashMap::new();
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().trim().chars().collect();
    assert_eq!(lines.next(), Some("")); // Skip empty line

    let pattern = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    for line in lines.map(|line| line.trim()) {
        let Some((_, [s, l, r])) = pattern.captures(line).map(|c| c.extract()) else {
            panic!("Invalid input: {line}");
        };
        map.insert(s.to_string(), (l.to_string(), r.to_string()));
    }

    (instructions, map)
}

fn run1(input: &str) -> u32 {
    let (instructions, map) = parse_input(input);

    let mut steps = 0;
    let mut curr = "AAA".to_string();
    for instr in instructions.repeat(100) {
        let new = match instr {
            'L' => get_left(&curr, &map),
            'R' => get_right(&curr, &map),
            _ => panic!("Unknown instruction: {instr}"),
        };
        steps += 1;
        if new == "ZZZ" {
            return steps;
        }
        curr = new;
    }

    0
}

fn part2() {
    let input = include_str!("./day08.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> u64 {
    let (instructions, map) = parse_input(input);

    let starts: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_owned())
        .collect();
    println!("STARTS: {starts:?}");
    let mut cycles: Vec<u64> = Vec::new();
    for start in starts {
        let mut steps = 0;
        let mut curr = start;
        for instr in instructions.clone().into_iter().cycle() {
            steps += 1;
            let new = match instr {
                'L' => get_left(&curr, &map),
                'R' => get_right(&curr, &map),
                _ => panic!("Unknown instruction: {instr}"),
            };
            if new.ends_with('Z') {
                cycles.push(steps);
                break;
            }
            curr = new;
        }
    }
    println!("CYCLES: {cycles:?}");

    cycles.into_iter().fold(1, num::integer::lcm)
}

fn get_left(step: &str, map: &StepsMap) -> String {
    map.get(step).unwrap().0.clone()
}

fn get_right(step: &str, map: &StepsMap) -> String {
    map.get(step).unwrap().1.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day08p1_run_1() {
        let input = r#"RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)"#;

        assert_eq!(run1(input), 2);
    }

    #[test]
    fn test_day08p1_run_2() {
        let input = r#"LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)"#;

        assert_eq!(run1(input), 6);
    }

    #[test]
    fn test_day08p2_run_1() {
        let input = r#"LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)"#;

        assert_eq!(run2(input), 6);
    }
}
