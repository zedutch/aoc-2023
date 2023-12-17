#![allow(unused)]

use std::{collections::HashMap, fmt::Display};

use array2d::Array2D;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day14.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> usize {
    let map = build_map(input);
    let map = tilt_map_north(&map);
    calculate_total_load(&map)
}

fn part2() {
    let input = include_str!("./day14.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> usize {
    let mut map = build_map(input);
    let mut cache = HashMap::new();
    let mut maps = HashMap::new();
    let mut period = 0;
    let mut start = 0;
    let mut i = 0;
    loop {
        i += 1;
        let new_map = cycle(&map);
        if let Some(idx) = cache.get(&new_map) {
            period = i - idx;
            start = *idx;
            break;
        } else {
            cache.insert(new_map.clone(), i);
            maps.insert(i, new_map.clone());
        }
        map = new_map;
    }
    let needed = start + (1_000_000_000 - start) % period;
    if let Some(map) = maps.get(&needed) {
        calculate_total_load(map)
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map(Array2D<char>);

impl std::hash::Hash for Map {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0
            .rows_iter()
            .for_each(|row| row.for_each(|c| c.hash(state)));
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        self.0.rows_iter().for_each(|row| {
            row.for_each(|c| result.push(c.to_owned()));
            result.push('\n');
        });
        write!(f, "{}", result)
    }
}

fn build_map(input: &str) -> Map {
    let tiles: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect())
        .collect();
    Map(Array2D::from_rows(&tiles).unwrap())
}

fn cycle(map: &Map) -> Map {
    let map = tilt_map_north(map);
    let map = tilt_map_west(&map);
    let map = tilt_map_south(&map);
    tilt_map_east(&map)
}

fn tilt_map_north(map: &Map) -> Map {
    let mut columns: Vec<Vec<char>> = Vec::new();
    map.0.columns_iter().for_each(|col| {
        let mut curr: Vec<char> = Vec::new();
        col.enumerate().for_each(|(idx, c)| {
            match c {
                '.' => (),
                '#' => {
                    // Fill the current column up to this index
                    for i in 0..(idx - curr.len()) {
                        curr.push('.');
                    }
                    curr.push('#');
                }
                'O' => {
                    curr.push('O');
                }
                _ => panic!("Unhandled char: {c}"),
            };
        });
        for i in 0..(map.0.column_len() - curr.len()) {
            curr.push('.');
        }
        columns.push(curr);
    });
    Map(Array2D::from_columns(&columns).unwrap())
}

fn tilt_map_west(map: &Map) -> Map {
    let mut rows: Vec<Vec<char>> = Vec::new();
    map.0.rows_iter().for_each(|row| {
        let mut curr: Vec<char> = Vec::new();
        row.enumerate().for_each(|(idx, c)| {
            match c {
                '.' => (),
                '#' => {
                    // Fill the current row up to this index
                    for i in 0..(idx - curr.len()) {
                        curr.push('.');
                    }
                    curr.push('#');
                }
                'O' => {
                    curr.push('O');
                }
                _ => panic!("Unhandled char: {c}"),
            };
        });
        for i in 0..(map.0.row_len() - curr.len()) {
            curr.push('.');
        }
        rows.push(curr);
    });
    Map(Array2D::from_rows(&rows).unwrap())
}

fn tilt_map_south(map: &Map) -> Map {
    let mut columns: Vec<Vec<char>> = Vec::new();
    map.0.columns_iter().for_each(|col| {
        let mut curr: Vec<char> = Vec::new();
        col.rev().enumerate().for_each(|(idx, c)| {
            match c {
                '.' => (),
                '#' => {
                    // Fill the current column up to this index
                    for i in 0..(idx - curr.len()) {
                        curr.push('.');
                    }
                    curr.push('#');
                }
                'O' => {
                    curr.push('O');
                }
                _ => panic!("Unhandled char: {c}"),
            };
        });
        for i in 0..(map.0.column_len() - curr.len()) {
            curr.push('.');
        }
        curr.reverse();
        columns.push(curr);
    });
    Map(Array2D::from_columns(&columns).unwrap())
}

fn tilt_map_east(map: &Map) -> Map {
    let mut rows: Vec<Vec<char>> = Vec::new();
    map.0.rows_iter().for_each(|row| {
        let mut curr: Vec<char> = Vec::new();
        row.rev().enumerate().for_each(|(idx, c)| {
            match c {
                '.' => (),
                '#' => {
                    // Fill the current row up to this index
                    for i in 0..(idx - curr.len()) {
                        curr.push('.');
                    }
                    curr.push('#');
                }
                'O' => {
                    curr.push('O');
                }
                _ => panic!("Unhandled char: {c}"),
            };
        });
        for i in 0..(map.0.row_len() - curr.len()) {
            curr.push('.');
        }
        curr.reverse();
        rows.push(curr);
    });
    Map(Array2D::from_rows(&rows).unwrap())
}

fn calculate_total_load(map: &Map) -> usize {
    map.0
        .columns_iter()
        .map(|col| {
            col.rev()
                .enumerate()
                .map(|(idx, c)| if *c == 'O' { idx + 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14p1_run() {
        let input = r#"O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        assert_eq!(run1(input), 136);
    }

    #[test]
    fn test_day14p1_tilt_1() {
        let input = r#"
            O....#....
            O...#....#
            ....O##...
            ...#O..#..
            .....#.O#.
            ..#..O.#.#
            ..O..#.O.O
            .....O.O..
            ###..###..
            #OO..#....
        "#;
        let map = build_map(input);
        let map2 = tilt_map_north(&map);
        assert_eq!(map, map2);
    }

    #[test]
    fn test_day14p1_tilt_2() {
        let input1 = r#"
            .O......#.
            ..#.#.O...
            OOO.......
            ...OO.O.O.
        "#;
        let input2 = r#"
            OO.O..O.#.
            .O#.#.O.O.
            ..O.O.....
            ..........
        "#;
        let map = build_map(input1);
        let map = tilt_map_north(&map);
        let map2 = build_map(input2);
        assert_eq!(map, map2);
    }

    #[test]
    fn test_day14p2_tilt_south() {
        let input1 = r#"
            OOO.......
            ..#.#.O...
            ...OO.O.O.
            .O......#.
        "#;
        let input2 = r#"
            ..O.......
            ..#.#.....
            .O....O.O.
            OO.OO.O.#.
        "#;
        let map = build_map(input1);
        let map = tilt_map_south(&map);
        let map2 = build_map(input2);
        assert_eq!(map, map2);
    }

    #[test]
    fn test_day14p2_tilt_west() {
        let input1 = r#"
            OOO.......
            ..#.#.O...
            ...OO.O.O.
            .O......#.
        "#;
        let input2 = r#"
            OOO.......
            ..#.#O....
            OOOO......
            O.......#.
        "#;
        let map = build_map(input1);
        let map = tilt_map_west(&map);
        let map2 = build_map(input2);
        assert_eq!(map, map2);
    }

    #[test]
    fn test_day14p2_tilt_east() {
        let input1 = r#"
            O.O..#....
            ..#..#O.#.
            ...O..O.O.
            .O......#.
        "#;
        let input2 = r#"
            ...OO#....
            ..#..#.O#.
            .......OOO
            .......O#.
        "#;
        let map = build_map(input1);
        let map = tilt_map_east(&map);
        let map2 = build_map(input2);
        assert_eq!(map, map2);
    }

    #[test]
    fn test_day14p2_cycle() {
        let input1 = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        let input2 = r#"
            .....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....
        "#;
        let input3 = r#"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O
        "#;
        let input4 = r#"
            .....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O
        "#;
        let input = build_map(input1);
        let map2 = cycle(&input);
        assert_eq!(map2, build_map(input2));
        let map3 = cycle(&map2);
        assert_eq!(map3, build_map(input3));
        let map4 = cycle(&map3);
        assert_eq!(map4, build_map(input4));
    }

    #[test]
    fn test_day14p2_run() {
        let input = r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "#;
        assert_eq!(run2(input), 64);
    }
}
