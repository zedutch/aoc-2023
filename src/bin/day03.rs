#![allow(unused)]

use std::collections::HashMap;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day03.in");
    let sum = run1(input);
    println!("{sum}");
}

fn part2() {
    let input = include_str!("./day03.in");
    let sum = run2(input);
    println!("{sum}");
}

#[derive(Debug)]
struct Entry {
    start: usize,
    end: usize,
    number: u32,
}

fn build_map(input: &str) -> HashMap<usize, Vec<Entry>> {
    let mut map: HashMap<usize, Vec<Entry>> = HashMap::new();
    for (idx, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        let pattern = regex::Regex::new(r"\d+").unwrap();
        let numbers = pattern.find_iter(line);
        let mut entries = vec![];
        for num in numbers {
            entries.push(Entry {
                start: num.start(),
                end: num.end() - 1, // Regex end is inclusive, we want exclusive
                number: num.as_str().parse().expect("Could not parse to a number"),
            });
        }
        map.insert(idx, entries);
    }
    map
}

fn get_numbers_bordering(x: usize, y: usize, map: &HashMap<usize, Vec<Entry>>) -> u32 {
    let mut sum = 0;
    for j in -1..=1_i32 {
        // There's no symbols on the first or last line so we can just unwrap this cast
        let index = usize::try_from(i32::try_from(y).unwrap() + j).unwrap();
        let numbers = map.get(&index).unwrap();
        for entry in numbers {
            if entry.start <= x + 1 && entry.end >= x - 1 {
                sum += entry.number;
            }
        }
    }
    sum
}

fn run1(input: &str) -> u32 {
    let map = build_map(input);
    let mut total = 0;
    for (y, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        for (x, char) in line.chars().enumerate() {
            let sum = match char {
                // Skip numbers and dots
                c if c.is_ascii_digit() => 0,
                '.' => 0,
                // Check all other symbols
                _ => get_numbers_bordering(x, y, &map),
            };
            total += sum;
        }
    }
    total
}

fn get_gear_ratio(x: usize, y: usize, map: &HashMap<usize, Vec<Entry>>) -> (u32, u32) {
    let mut prod = 0;
    let mut amount = 0;
    for j in -1..=1_i32 {
        // There's no symbols on the first or last line so we can just unwrap this cast
        let index = usize::try_from(i32::try_from(y).unwrap() + j).unwrap();
        let numbers = map.get(&index).unwrap();
        for entry in numbers {
            if entry.start <= x + 1 && entry.end >= x - 1 {
                if prod == 0 {
                    prod = entry.number
                } else {
                    prod *= entry.number;
                }
                amount += 1;
            }
        }
    }
    (prod, amount)
}

fn run2(input: &str) -> u32 {
    let map = build_map(input);
    let mut total = 0;
    for (y, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        for (x, char) in line.chars().enumerate() {
            let sum = match char {
                '*' => match get_gear_ratio(x, y, &map) {
                    (prod, 2) => prod,
                    _ => 0,
                },
                // Skip everything else
                _ => 0,
            };
            total += sum;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day03p1() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(run1(input), 4361);

        let input = r#"
            467.114...
            ..1*4.....
            .35.633...
        "#;
        assert_eq!(run1(input), 1254);

        let input = r#"
            467...114.
            ..1.*.4...
            .35...633.
        "#;
        assert_eq!(run1(input), 0);

        let input = r#"
            ..........
            ...*1.4...
            ..........
        "#;
        assert_eq!(run1(input), 1);

        let input = r#"
            ..........
            ....1/4...
            .........1   
        "#;
        assert_eq!(run1(input), 5);

        let input = r#"
            .........4
            .@........
            230.....@.
            4.........
        "#;
        assert_eq!(run1(input), 230);
    }

    #[test]
    pub fn test_day03p2() {
        let input = r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        "#;
        assert_eq!(run2(input), 467835);
    }
}
