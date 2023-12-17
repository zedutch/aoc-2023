#![allow(unused)]

use std::{
    collections::HashSet,
    fmt::Display,
};

use array2d::Array2D;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day16.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> usize {
    let map = build_map(input);
    let tiles = get_energized_tiles(
        &map,
        State {
            coords: (-1, 0),
            dir: (1, 0),
        },
    );
    tiles.len()
}

fn part2() {
    let input = include_str!("./day16.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> usize {
    let map = build_map(input);
    let mut max = 0;
    let width = map.0.row_len();
    let height = map.0.column_len();
    let iwidth = i32::try_from(width).unwrap();
    let iheight = i32::try_from(height).unwrap();

    for i in 0..width {
        if let Ok(x) = i32::try_from(i) {
            // Top row
            let tiles = get_energized_tiles(
                &map,
                State {
                    coords: (x, -1),
                    dir: (0, 1),
                },
            );
            let len = tiles.len();
            if len > max {
                max = len;
            }
        }
        if let Ok(x) = i32::try_from(i) {
            // Bottom row
            let tiles = get_energized_tiles(
                &map,
                State {
                    coords: (x, iheight),
                    dir: (0, -1),
                },
            );
            let len = tiles.len();
            if len > max {
                max = len;
            }
        }
    }

    for j in 0..height {
        if let Ok(y) = i32::try_from(j) {
            // Left column
            let tiles = get_energized_tiles(
                &map,
                State {
                    coords: (-1, y),
                    dir: (1, 0),
                },
            );
            let len = tiles.len();
            if len > max {
                max = len;
            }
        }
        if let Ok(y) = i32::try_from(j) {
            // Right column
            let tiles = get_energized_tiles(
                &map,
                State {
                    coords: (iwidth, y),
                    dir: (-1, 0),
                },
            );
            let len = tiles.len();
            if len > max {
                max = len;
            }
        }
    }
    max
}

fn build_map(input: &str) -> Map {
    let tiles: Vec<Vec<Tile>> = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().map(Tile::from).collect())
        .collect();
    let map: Map = Map(Array2D::from_rows(&tiles).unwrap());
    map
}

fn get_energized_tiles(map: &Map, initial: State) -> HashSet<(i32, i32)> {
    let mut tiles = HashSet::new();
    let mut todo = vec![initial];
    let mut evaluated = HashSet::new();
    while let Some(state) = todo.pop() {
        if let Some(result) = tick(map, &state) {
            tiles.insert(result.state.coords);
            if !evaluated.contains(&result.state) {
                todo.push(result.state);
            }
            if let Some(split) = result.split {
                if !evaluated.contains(&split) {
                    todo.push(split);
                }
            }
        }
        evaluated.insert(state);
    }
    tiles
}

fn tick(map: &Map, state: &State) -> Option<TickResult> {
    let coords = (state.coords.0 + state.dir.0, state.coords.1 + state.dir.1);
    if coords.0 < 0 || coords.1 < 0 {
        // println!("Path out of bounds");
        return None;
    }
    let dir = state.dir;
    let x = usize::try_from(coords.0).unwrap();
    let y = usize::try_from(coords.1).unwrap();
    let tile = map.0.get(y, x);
    // println!("Checking tile {coords:?}, direction: {dir:?}. Found: {tile:?}");
    match tile {
        Some(Tile::None) => Some(TickResult {
            state: State { coords, dir },
            split: None,
        }),
        Some(Tile::SplitterV) => {
            if dir.0 != 0 {
                Some(TickResult {
                    state: State {
                        coords,
                        dir: (0, 1),
                    },
                    split: Some(State {
                        coords,
                        dir: (0, -1),
                    }),
                })
            } else {
                Some(TickResult {
                    state: State { coords, dir },
                    split: None,
                })
            }
        }
        Some(Tile::SplitterH) => {
            if dir.1 != 0 {
                Some(TickResult {
                    state: State {
                        coords,
                        dir: (1, 0),
                    },
                    split: Some(State {
                        coords,
                        dir: (-1, 0),
                    }),
                })
            } else {
                Some(TickResult {
                    state: State { coords, dir },
                    split: None,
                })
            }
        }
        Some(Tile::MirrorL) => {
            let new_dir = match dir {
                (1, 0) => (0, 1),
                (-1, 0) => (0, -1),
                (0, 1) => (1, 0),
                (0, -1) => (-1, 0),
                _ => panic!("Invalid direction: {dir:?}"),
            };
            Some(TickResult {
                state: State {
                    coords,
                    dir: new_dir,
                },
                split: None,
            })
        }
        Some(Tile::MirrorR) => {
            let new_dir = match dir {
                (1, 0) => (0, -1),
                (-1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (0, -1) => (1, 0),
                _ => panic!("Invalid direction: {dir:?}"),
            };
            Some(TickResult {
                state: State {
                    coords,
                    dir: new_dir,
                },
                split: None,
            })
        }
        Some(tile) => None,
        None => None,
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct State {
    coords: (i32, i32),
    dir: (i32, i32),
}

struct TickResult {
    state: State,
    split: Option<State>,
}

struct Map(Array2D<Tile>);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Tile {
    None,
    MirrorR,
    MirrorL,
    SplitterV,
    SplitterH,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::None,
            '/' => Tile::MirrorR,
            '\\' => Tile::MirrorL,
            '|' => Tile::SplitterV,
            '-' => Tile::SplitterH,
            _ => panic!("Unknown tile char: {value}"),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::None => '.',
            Tile::MirrorR => '/',
            Tile::MirrorL => '\\',
            Tile::SplitterV => '|',
            Tile::SplitterH => '-',
            _ => '?',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = self.to_owned().into();
        write!(f, "{}", c)
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        self.0.rows_iter().for_each(|row| {
            row.for_each(|t| result.push_str(&t.to_string()));
            result.push('\n');
        });
        write!(f, "{}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16p1_run() {
        let input = r#".|...\....
                       |.-.\.....
                       .....|-...
                       ........|.
                       ..........
                       .........\
                       ..../.\\..
                       .-.-/..|..
                       .|....-|.\
                       ..//.|....
                    "#;
        assert_eq!(run1(input), 46);
    }

    #[test]
    fn test_day16p1_build_map() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
        let map = build_map(input);
        assert_eq!(map.to_string(), input);
    }

    #[test]
    fn test_day16p2_run() {
        let input = r#".|...\....
                       |.-.\.....
                       .....|-...
                       ........|.
                       ..........
                       .........\
                       ..../.\\..
                       .-.-/..|..
                       .|....-|.\
                       ..//.|....
                    "#;
        assert_eq!(run2(input), 51);
    }
}
