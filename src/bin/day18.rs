#![allow(unused)]

use std::collections::HashSet;

use array2d::Array2D;
use polygonical::{point::Point, polygon::Polygon};

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day18.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> u64 {
    let cmds = parse_cmds(input, false);
    let mut map = build_map(cmds);
    let centre = (
        i32::try_from(map.column_len() / 2).unwrap(),
        i32::try_from(map.row_len() / 2).unwrap(),
    );
    flood_fill(&mut map, centre, false);
    count_values(&map, false)
}

fn part2() {
    let input = include_str!("./day18.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> f64 {
    let cmds = parse_cmds(input, true);
    let poly = build_polygon(cmds);
    // Calculate the "surface" of the axis-aligned lines
    let surface: f64 = 0.5 * poly
        .sides()
        .iter()
        .map(|(p1, p2)| ((p2.x - p1.x) + (p2.y - p1.y)).abs())
        .sum::<f64>();
    let mut area = poly.area().abs(); // Abs in case of wrong girality
    area + surface + 1.0 // add starting position
}

fn parse_cmds(input: &str, use_col: bool) -> Vec<Cmd> {
    input
        .trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let mut iter = line.trim().split(' ');
            if use_col {
                let col = match iter.nth(2) {
                    // Remove parentheses and # from around colour value
                    Some(c) => c.split_at(2).1.split_at(6).0.to_string(),
                    None => panic!("Cannot find colour"),
                };
                let col = col.split_at(5);
                let dir = match col.1 {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => panic!("Invalid dir: {:?}", col),
                };
                let num = u32::from_str_radix(col.0, 16)
                    .unwrap_or_else(|_| panic!("Number too large for 32 bit: {}", col.0));
                Cmd { dir, num }
            } else {
                let dir = match iter.next() {
                    Some("U") => Direction::Up,
                    Some("D") => Direction::Down,
                    Some("L") => Direction::Left,
                    Some("R") => Direction::Right,
                    d => panic!("Invalid direction: {:?}", d),
                };
                let num = match iter.next() {
                    Some(c) => match c.parse() {
                        Ok(num) => num,
                        Err(_) => panic!("Cannot parse number: {:?}", c),
                    },
                    d => panic!("Invalid number: {:?}", d),
                };
                Cmd { dir, num }
            }
        })
        .collect()
}

fn build_polygon(cmds: Vec<Cmd>) -> Polygon {
    let dim = cmds.iter().fold([(0, 0), (0, 0), (0, 0)], |acc, cmd| {
        let num = i32::try_from(cmd.num).unwrap();
        let mut val = acc[0];
        let min = acc[1];
        let max = acc[2];
        val = match cmd.dir {
            Direction::Up => (val.0, val.1 - num),
            Direction::Down => (val.0, val.1 + num),
            Direction::Left => (val.0 - num, val.1),
            Direction::Right => (val.0 + num, val.1),
        };
        let min = (min.0.min(val.0), min.1.min(val.1));
        let max = (max.0.max(val.0), max.1.max(val.1));
        [val, min, max]
    });
    let min = dim[1];
    let max = dim[2];
    println!("Got poly min: {min:?}, max: {max:?}");
    let dim = (1 + max.0 - min.0, 1 + max.1 - min.1);

    let mut pos: (i32, i32) = (min.0.abs(), min.1.abs());
    println!("Starting on {pos:?}");

    let mut points = Vec::new();
    for cmd in cmds {
        let num = i32::try_from(cmd.num).unwrap();
        pos = match cmd.dir {
            Direction::Up => (pos.0, pos.1 - num),
            Direction::Down => (pos.0, pos.1 + num),
            Direction::Left => (pos.0 - num, pos.1),
            Direction::Right => (pos.0 + num, pos.1),
        };
        points.push(Point::new(pos.0, pos.1));
    }

    Polygon::new(points)
}

fn build_map(cmds: Vec<Cmd>) -> Map {
    let dim = cmds.iter().fold([(0, 0), (0, 0), (0, 0)], |acc, cmd| {
        let num = i32::try_from(cmd.num).unwrap();
        let mut val = acc[0];
        let min = acc[1];
        let max = acc[2];
        val = match cmd.dir {
            Direction::Up => (val.0, val.1 - num),
            Direction::Down => (val.0, val.1 + num),
            Direction::Left => (val.0 - num, val.1),
            Direction::Right => (val.0 + num, val.1),
        };
        let min = (min.0.min(val.0), min.1.min(val.1));
        let max = (max.0.max(val.0), max.1.max(val.1));
        [val, min, max]
    });
    let min = dim[1];
    let max = dim[2];
    println!("Got map min: {min:?}, max: {max:?}");
    let dim = (1 + max.0 - min.0, 1 + max.1 - min.1);
    let cols = usize::try_from(dim.0).unwrap();
    let rows = usize::try_from(dim.1).unwrap();
    println!("Map dimensions: {dim:?}");
    let mut map = Array2D::filled_with(true, rows, cols);

    let mut pos: (i32, i32) = (min.0.abs(), min.1.abs());
    set_value(&mut map, pos.0, pos.1, false);
    println!("Starting at {pos:?}");

    for cmd in cmds {
        // println!("cmd: {cmd:?}");
        let delta: (i32, i32) = match cmd.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..cmd.num {
            pos.0 += delta.0;
            pos.1 += delta.1;
            set_value(&mut map, pos.0, pos.1, false);
        }
    }
    map
}

type Map = Array2D<bool>;

fn flood_fill(map: &mut Map, coords: (i32, i32), value: bool) {
    let mut todo: HashSet<(i32, i32)> = HashSet::new();
    let mut new: HashSet<(i32, i32)> = HashSet::new();
    todo.insert(coords);
    loop {
        todo.extend(new.iter());
        new.clear();
        let elements = todo.drain();
        for coords in elements {
            new.extend(flood_fill_helper(map, coords.to_owned(), value).iter());
        }
        if todo.is_empty() && new.is_empty() {
            break;
        }
    }
}

fn flood_fill_helper(map: &mut Map, coords: (i32, i32), value: bool) -> HashSet<(i32, i32)> {
    let mut next_coords: HashSet<(i32, i32)> = HashSet::new();
    // Fill in the current tile if it's empty
    if let Some(tile) = get_value(map, coords.0, coords.1) {
        if *tile != value {
            set_value(map, coords.0, coords.1, value);
        }
    }
    for delta in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let next = (coords.0 + delta.0, coords.1 + delta.1);
        if let Some(tile) = get_value(map, next.0, next.1) {
            if *tile != value {
                next_coords.insert(next);
            }
        }
    }
    next_coords
}

fn get_value(map: &Map, x: i32, y: i32) -> Option<&bool> {
    let x = match usize::try_from(x) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let y = match usize::try_from(y) {
        Ok(v) => v,
        Err(_) => return None,
    };
    map.get(y, x)
}

fn set_value(map: &mut Map, x: i32, y: i32, value: bool) {
    let x = usize::try_from(x).unwrap();
    let y = usize::try_from(y).unwrap();
    map.set(y, x, value).unwrap();
}

fn count_values(map: &Map, value: bool) -> u64 {
    map.rows_iter()
        .map(|row| u64::try_from(row.filter(|p| **p == value).count()).unwrap())
        .sum::<u64>()
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Cmd {
    pub dir: Direction,
    pub num: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day18p1_run() {
        let input = r#"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "#;
        assert_eq!(run1(input), 62);
    }

    #[test]
    pub fn test_day18p2_run() {
        let input = r#"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "#;
        assert_eq!(run2(input), 952408144115.0);
    }
}
