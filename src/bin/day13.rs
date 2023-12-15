#![allow(unused)]

use array2d::Array2D;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day13.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    let maps = split(input);
    for map in maps {
        let result = find_mirror(map);
        sum += result;
    }
    sum
}

fn part2() {
    let input = include_str!("./day13.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> u32 {
    let mut sum = 0;
    let maps = split(input);
    for map in maps {
        let result = find_mirror_smudged(map);
        sum += result;
    }
    sum
}

fn split(input: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let mut curr = Vec::new();
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() {
            result.push(curr.clone());
            curr.clear();
        } else {
            curr.push(line.to_string());
        }
    }
    if !curr.is_empty() {
        result.push(curr.clone());
    }
    result
}

fn find_mirror(input: Vec<String>) -> u32 {
    let mut result = 100 * find_horizontal(&input);
    if result == 0 {
        result = find_vertical(&input);
    }
    result
}

fn find_mirror_smudged(input: Vec<String>) -> u32 {
    let mut result = 100 * find_horizontal_smudged(&input);
    if result == 0 {
        result = find_vertical_smudged(&input);
    }
    result
}

fn find_vertical(input: &[String]) -> u32 {
    let matrix = Array2D::from_rows(
        &input
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap();
    let cols = matrix
        .columns_iter()
        .map(|c| c.collect::<String>())
        .collect::<Vec<String>>();
    find_horizontal(&cols)
}

fn find_horizontal(input: &[String]) -> u32 {
    for (idx, (line1, line2)) in input.iter().zip(input.iter().skip(1)).enumerate() {
        if line1 == line2 && is_mirror_line(input, idx) {
            return u32::try_from(idx + 1).unwrap();
        }
    }
    0
}

fn find_vertical_smudged(input: &[String]) -> u32 {
    let matrix = Array2D::from_rows(
        &input
            .iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    )
    .unwrap();
    let cols = matrix
        .columns_iter()
        .map(|c| c.collect::<String>())
        .collect::<Vec<String>>();
    find_horizontal_smudged(&cols)
}

fn find_horizontal_smudged(input: &[String]) -> u32 {
    for (idx, (line1, line2)) in input.iter().zip(input.iter().skip(1)).enumerate() {
        let dist = line1
            .chars()
            .zip(line2.chars())
            .filter(|(c1, c2)| c1 != c2)
            .count();
        // The error is in the mirror line
        if (dist == 1 && is_mirror_line(input, idx))
            // The error is somewhere else
            || (dist == 0 && is_mirror_line_smudged(input, idx))
        {
            return u32::try_from(idx + 1).unwrap();
        }
    }
    0
}

fn is_mirror_line_smudged(input: &[String], candidate: usize) -> bool {
    let first = input.iter().take(candidate);
    let second = input.iter().skip(candidate + 2).rev();

    // Make both iterators the same size, skipping elements at the front
    let first_len = first.len();
    let second_len = second.len();
    let length = first_len.min(second_len);
    let first = first.skip(first_len - length);
    let second = second.skip(second_len - length);

    let mut smudge_fixed = false;
    for (s1, s2) in first.zip(second) {
        let dist = s1
            .chars()
            .zip(s2.chars())
            .filter(|(c1, c2)| c1 != c2)
            .count();
        if dist == 0 {
            continue;
        } else if dist == 1 && !smudge_fixed {
            smudge_fixed = true;
            continue;
        } else {
            // Dist > 1 or dist = 1 while smudge was already fixed
            return false;
        }
    }
    // If the smudge was not fixed, this is not a smudged mirror line
    smudge_fixed
}

fn is_mirror_line(input: &[String], candidate: usize) -> bool {
    let first = input.iter().take(candidate);
    let second = input.iter().skip(candidate + 2).rev();

    // Make both iterators the same size, skipping elements at the front
    let first_len = first.len();
    let second_len = second.len();
    let length = first_len.min(second_len);
    let first = first.skip(first_len - length);
    let second = second.skip(second_len - length);

    first.zip(second).all(|(s1, s2)| s1 == s2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13p1_horizontal() {
        let input = split(
            r#"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#,
        );
        assert_eq!(find_horizontal(input.first().unwrap()), 4);
    }

    #[test]
    fn test_day13p1_vertical() {
        let input = split(
            r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "#,
        );
        assert_eq!(find_vertical(input.first().unwrap()), 5);
    }

    #[test]
    fn test_day13p1_run() {
        let input = r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#;
        assert_eq!(run1(input), 405);
    }

    #[test]
    fn test_day13p2_horizontal_1() {
        let input = split(
            r#"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#,
        );
        assert_eq!(find_horizontal_smudged(input.first().unwrap()), 1);
    }

    #[test]
    fn test_day13p2_horizontal_2() {
        let input = split(
            r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
        "#,
        );
        assert_eq!(find_horizontal_smudged(input.first().unwrap()), 3);
    }

    #[test]
    fn test_day13p2_run() {
        let input = r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "#;
        assert_eq!(run2(input), 400);
    }
}
