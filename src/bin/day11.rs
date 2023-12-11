#![allow(unused)]

use std::collections::BTreeSet;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day11.in");
    let result = run1(input);
    println!("{result}");
}

fn part2() {
    let input = include_str!("./day11.in");
    let result = run2(input, 999999);
    println!("{result}");
}

fn run1(input: &str) -> u64 {
    let pairs = get_galaxy_pairs(input, 1);
    pairs
        .into_iter()
        .map(|(g1, g2)| get_distance(&g1, &g2))
        .sum()
}

fn run2(input: &str, expansion_rate: usize) -> u64 {
    let pairs = get_galaxy_pairs(input, expansion_rate);
    pairs
        .into_iter()
        .map(|(g1, g2)| get_distance(&g1, &g2))
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Galaxy {
    x: usize,
    y: usize,
}

fn get_distance(g1: &Galaxy, g2: &Galaxy) -> u64 {
    let mut dist = 0;
    if g1.x > g2.x {
        dist += g1.x - g2.x;
    } else {
        dist += g2.x - g1.x;
    }
    if g1.y > g2.y {
        dist += g1.y - g2.y;
    } else {
        dist += g2.y - g1.y;
    }
    u64::try_from(dist).unwrap()
}

fn get_galaxy_pairs(input: &str, expansion_rate: usize) -> Vec<(Galaxy, Galaxy)> {
    let universe = get_galaxies(input, expansion_rate);
    let mut pairs: BTreeSet<(Galaxy, Galaxy)> = BTreeSet::new();

    for g1 in universe.iter() {
        for g2 in universe.iter() {
            if g1.x == g2.x && g1.y == g2.y {
                continue;
            }
            let g1 = g1.to_owned();
            let g2 = g2.to_owned();
            let mut list = [g1, g2];
            list.sort();
            let [g1, g2] = list;
            pairs.insert((g1, g2));
        }
    }
    pairs.into_iter().collect()
}

fn get_galaxies(input: &str, expansion_rate: usize) -> Vec<Galaxy> {
    let mut universe: Vec<Galaxy> = Vec::new();
    let empty_cols = get_empty_cols(input);
    let mut dy = 0;

    for (y, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        if line.chars().all(|c| c == '.') {
            dy += expansion_rate;
            continue;
        }
        let mut dx = 0;
        for (x, c) in line.chars().enumerate() {
            if empty_cols.contains(&x) {
                dx += expansion_rate;
                continue;
            }
            match c {
                '.' => continue,
                '#' => universe.push(Galaxy {
                    x: x + dx,
                    y: y + dy,
                }),
                _ => panic!("Unknown symbol: {c}"),
            };
        }
    }
    universe
}

fn get_empty_cols(input: &str) -> BTreeSet<usize> {
    let mut cols = input.trim().lines().map(|line| {
        line.trim()
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| match c {
                '.' => Some(idx),
                _ => None,
            })
            .collect::<BTreeSet<usize>>()
    });
    let mut empty = cols.next().unwrap();
    for col in cols {
        empty = empty.intersection(&col).cloned().collect();
    }
    empty
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11p1_run() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        assert_eq!(run1(input), 374);
    }

    #[test]
    fn test_day11p1_pairs() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        let pairs = get_galaxy_pairs(input, 1);
        let galaxies = get_galaxies(input, 1);
        assert_eq!(galaxies.len(), 9);
        assert_eq!(pairs.len(), 36);
        assert_eq!(
            pairs
                .iter()
                .filter(|(g1, g2)| (g1.x == 4 && g1.y == 0) || (g2.x == 4 && g2.y == 0))
                .count(),
            8
        );
    }

    #[test]
    fn test_day11p1_dist() {
        let g1 = Galaxy { x: 4, y: 0 };
        let g3 = Galaxy { x: 0, y: 2 };
        let g5 = Galaxy { x: 1, y: 6 };
        let g6 = Galaxy { x: 12, y: 7 };
        let g7 = Galaxy { x: 9, y: 10 };
        let g8 = Galaxy { x: 0, y: 11 };
        let g9 = Galaxy { x: 5, y: 11 };
        let g10 = Galaxy { x: 0, y: 10 };
        assert_eq!(get_distance(&g5, &g9), 9);
        assert_eq!(get_distance(&g9, &g5), 9);
        assert_eq!(get_distance(&g1, &g7), 15);
        assert_eq!(get_distance(&g7, &g1), 15);
        assert_eq!(get_distance(&g3, &g6), 17);
        assert_eq!(get_distance(&g8, &g9), 5);
        assert_eq!(get_distance(&g3, &g8), 9);
        assert_eq!(get_distance(&g3, &g10), 8);
    }

    #[test]
    fn test_day11p2_run() {
        let input = r#"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "#;
        assert_eq!(run2(input, 1), 374);
        assert_eq!(run2(input, 9), 1030);
        assert_eq!(run2(input, 99), 8410);
    }
}
