#![allow(unused)]

use std::collections::HashSet;

use array2d::Array2D;

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day10.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> u64 {
    let pipes = parse_pipes(input.trim());
    let size = get_loop_size(&pipes);
    if size % 2 == 1 {
        (size + 1) / 2
    } else {
        size / 2
    }
}

fn part2() {
    let input = include_str!("./day10.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> u64 {
    let pipes = parse_pipes(input.trim());
    get_inner_tiles(&pipes)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Pipe {
    Start,
    Horizontal,
    Vertical,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,

    // Part 2
    Outer,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PipePiece {
    pipe: Pipe,
    x: i32,
    y: i32,
}

type Pipes = Array2D<Pipe>;

fn parse_pipes(input: &str) -> Pipes {
    let chars = input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Pipe::None,
                    '|' => Pipe::Vertical,
                    '-' => Pipe::Horizontal,
                    '7' => Pipe::BottomLeft,
                    'L' => Pipe::TopRight,
                    'F' => Pipe::BottomRight,
                    'J' => Pipe::TopLeft,
                    'S' => Pipe::Start,
                    _ => panic!("Unknown character: '{c}'"),
                })
                .collect::<Vec<Pipe>>()
        })
        .collect::<Vec<Vec<Pipe>>>();
    Array2D::from_rows(&chars).expect("Could not build pipes array")
}

fn get_start_coords(pipes: &Pipes) -> (i32, i32) {
    let mut coords = (0, 0);
    for (y, row) in pipes.rows_iter().enumerate() {
        for (x, pipe) in row.enumerate() {
            if *pipe == Pipe::Start {
                coords.0 = i32::try_from(x).unwrap();
                coords.1 = i32::try_from(y).unwrap();
            }
        }
    }
    coords
}

fn get_pieces(pipes: &Pipes) -> HashSet<PipePiece> {
    let mut coords = get_start_coords(pipes);
    let mut pieces = HashSet::<PipePiece>::new();
    pieces.insert(PipePiece {
        pipe: Pipe::Start,
        x: coords.0,
        y: coords.1,
    });
    loop {
        let Some(next) = find_next(coords, &mut pieces, pipes) else {
            break;
        };
        coords = next;
    }
    pieces
}

fn get_loop_size(pipes: &Pipes) -> u64 {
    let pieces = get_pieces(pipes);
    u64::try_from(pieces.len()).unwrap()
}

/// Get the amount of inner tiles surrounded by pipes.
fn get_inner_tiles(pipes: &Pipes) -> u64 {
    // First, parse the pipes and remove all pipes that are not part of the main path
    let pieces = get_pieces(pipes);
    let mut pipes = clean_pipes(pipes, &pieces);

    // Then blow the pipes list up, 1 tile -> 3x3 tiles
    let mut pipes = blow_up(&pipes);

    // Flood fill the blown up tile map, starting from the border tiles
    for i in 0..pipes.row_len() {
        for j in 0..pipes.column_len() {
            if i != 0 && j != 0 && i != pipes.row_len() - 1 && j != pipes.column_len() - 1 {
                continue;
            }
            let x = i32::try_from(i).unwrap();
            let y = i32::try_from(j).unwrap();
            flood_fill(&mut pipes, (x, y), &Pipe::Outer);
        }
    }

    // Shrink the filled map back down, 3x3 tiles -> 1 (centre) tile
    let pipes = shrink_down(&pipes);

    // Count the amount of empty tiles that we didn't visit in the flood fill
    count_tiles(&pipes, &Pipe::None)
}

/// Blow up a map so each tile maps onto a 3x3 set of tiles.
fn blow_up(pipes: &Pipes) -> Pipes {
    let mut new = Array2D::filled_with(Pipe::None, pipes.num_rows() * 3, pipes.num_columns() * 3);
    for (y, row) in pipes.rows_iter().enumerate() {
        for (x, el) in row.enumerate() {
            set_blown_up(&mut new, (x, y), el);
        }
    }
    new
}

/// Set a value in a blown up map using its coordinates and value in the original map.
fn set_blown_up(pipes: &mut Pipes, (column, row): (usize, usize), el: &Pipe) {
    // Set all elements to None
    for i in 0..3 {
        for j in 0..3 {
            pipes.set(row * 3 + i, column * 3 + j, Pipe::None);
        }
    }
    match el {
        Pipe::Start => {
            for i in 0..3 {
                for j in 0..3 {
                    pipes.set(row * 3 + i, column * 3 + j, Pipe::Start);
                }
            }
        }
        Pipe::Horizontal => {
            for i in 0..3 {
                pipes.set(row * 3 + 1, column * 3 + i, Pipe::Horizontal);
            }
        }
        Pipe::Vertical => {
            for i in 0..3 {
                pipes.set(row * 3 + i, column * 3 + 1, Pipe::Vertical);
            }
        }
        Pipe::TopRight => {
            pipes.set(row * 3, column * 3 + 1, Pipe::Vertical);
            pipes.set(row * 3 + 1, column * 3 + 1, Pipe::TopRight);
            pipes.set(row * 3 + 1, column * 3 + 2, Pipe::Horizontal);
        }
        Pipe::TopLeft => {
            pipes.set(row * 3, column * 3 + 1, Pipe::Horizontal);
            pipes.set(row * 3 + 1, column * 3 + 1, Pipe::TopLeft);
            pipes.set(row * 3 + 1, column * 3, Pipe::Vertical);
        }
        Pipe::BottomRight => {
            pipes.set(row * 3 + 2, column * 3 + 1, Pipe::Vertical);
            pipes.set(row * 3 + 1, column * 3 + 1, Pipe::BottomRight);
            pipes.set(row * 3 + 1, column * 3 + 2, Pipe::Horizontal);
        }
        Pipe::BottomLeft => {
            pipes.set(row * 3 + 2, column * 3 + 1, Pipe::Vertical);
            pipes.set(row * 3 + 1, column * 3 + 1, Pipe::BottomLeft);
            pipes.set(row * 3 + 1, column * 3, Pipe::Horizontal);
        }
        Pipe::Outer | Pipe::None => (),
    }
}

/// Shrink down a blown up map so each set of 3x3 tiles maps back to 1 tile.
/// The shrunken down values are the centres of the 3x3 tiles.
fn shrink_down(pipes: &Pipes) -> Pipes {
    let mut new = Array2D::filled_with(Pipe::None, pipes.num_rows() / 3, pipes.num_columns() / 3);
    for (y, row) in pipes.rows_iter().enumerate() {
        if y % 3 != 1 {
            continue;
        }
        for (x, el) in row.enumerate() {
            if x % 3 != 1 {
                continue;
            }
            let row = y / 3;
            let column = x / 3;
            new.set(row, column, el.clone());
        }
    }
    new
}

/// Flood fill the map with the specified type, starting from the specified coordinate.
/// Only tiles with value `Pipe::None` will be replaced.
/// This uses a looping todo-list to prevent stack overflows.
fn flood_fill(pipes: &mut Pipes, coords: (i32, i32), value: &Pipe) {
    let mut todo: HashSet<(i32, i32)> = HashSet::new();
    let mut new: HashSet<(i32, i32)> = HashSet::new();
    todo.insert(coords);
    loop {
        todo.extend(new.iter());
        new.clear();
        let elements = todo.drain();
        for coords in elements {
            new.extend(flood_fill_helper(pipes, coords.to_owned(), value).iter());
        }
        if todo.is_empty() && new.is_empty() {
            break;
        }
    }
}

/// Helper function for the flood fill function that returns the next tiles to check
fn flood_fill_helper(pipes: &mut Pipes, coords: (i32, i32), value: &Pipe) -> HashSet<(i32, i32)> {
    let mut next_coords: HashSet<(i32, i32)> = HashSet::new();
    // Fill in the current tile if it's empty
    if let Some(Pipe::None) = get_pipe(pipes, coords.0, coords.1) {
        let x = usize::try_from(coords.0).unwrap();
        let y = usize::try_from(coords.1).unwrap();
        pipes.set(y, x, value.clone());
    }
    for delta in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let mut next = (coords.0 + delta.0, coords.1 + delta.1);
        let pipe = get_pipe(pipes, next.0, next.1);
        if let Some(Pipe::None) = pipe {
            next_coords.insert(next);
        };
    }
    next_coords
}

/// Remove all pipes from the map that are not part of the main loop
fn clean_pipes(pipes: &Pipes, pieces: &HashSet<PipePiece>) -> Pipes {
    Array2D::from_rows(
        &pipes
            .rows_iter()
            .enumerate()
            .map(|(y, row)| {
                row.enumerate()
                    .map(move |(x, pipe)| {
                        let x = i32::try_from(x).unwrap();
                        let y = i32::try_from(y).unwrap();
                        if !pieces.contains(&PipePiece {
                            pipe: pipe.clone(),
                            x,
                            y,
                        }) {
                            Pipe::None
                        } else {
                            pipe.clone()
                        }
                    })
                    .collect()
            })
            .collect::<Vec<Vec<Pipe>>>(),
    )
    .unwrap()
}

/// Find the next pipe that's part of the main loop
fn find_next(
    coords: (i32, i32),
    pieces: &mut HashSet<PipePiece>,
    pipes: &Pipes,
) -> Option<(i32, i32)> {
    let prev = get_pipe(pipes, coords.0, coords.1).unwrap();
    let to_check: Vec<(i32, i32)> = match prev {
        Pipe::Start => vec![(0, 1), (1, 0), (-1, 0), (0, -1)],
        Pipe::Horizontal => vec![(-1, 0), (1, 0)],
        Pipe::Vertical => vec![(0, 1), (0, -1)],
        Pipe::TopRight => vec![(0, -1), (1, 0)],
        Pipe::TopLeft => vec![(0, -1), (-1, 0)],
        Pipe::BottomRight => vec![(0, 1), (1, 0)],
        Pipe::BottomLeft => vec![(0, 1), (-1, 0)],
        _ => panic!("Came from invalid tile"),
    };

    for delta in to_check {
        let mut next = (coords.0 + delta.0, coords.1 + delta.1);
        let pipe = get_pipe(pipes, next.0, next.1);
        let Some(pipe) = pipe else {
            continue;
        };
        if pieces.contains(&PipePiece {
            pipe: pipe.clone(),
            x: next.0,
            y: next.1,
        }) {
            // Pipe already parsed
            continue;
        }
        match (delta, pipe) {
            // DOWN
            ((0, 1), Pipe::TopLeft) | ((0, 1), Pipe::TopRight) | ((0, 1), Pipe::Vertical) => {
                if !matches!(
                    prev,
                    Pipe::Start | Pipe::Vertical | Pipe::BottomLeft | Pipe::BottomRight
                ) {
                    continue;
                }
            }
            // UP
            ((0, -1), Pipe::BottomLeft)
            | ((0, -1), Pipe::BottomRight)
            | ((0, -1), Pipe::Vertical) => {
                if !matches!(
                    prev,
                    Pipe::Start | Pipe::Vertical | Pipe::TopLeft | Pipe::TopRight
                ) {
                    continue;
                }
            }
            // RIGHT
            ((1, 0), Pipe::TopLeft) | ((1, 0), Pipe::BottomLeft) | ((1, 0), Pipe::Horizontal) => {
                if !matches!(
                    prev,
                    Pipe::Start | Pipe::Horizontal | Pipe::TopRight | Pipe::BottomRight
                ) {
                    continue;
                }
            }
            // LEFT
            ((-1, 0), Pipe::TopRight)
            | ((-1, 0), Pipe::BottomRight)
            | ((-1, 0), Pipe::Horizontal) => {
                if !matches!(
                    prev,
                    Pipe::Start | Pipe::Horizontal | Pipe::TopLeft | Pipe::BottomLeft
                ) {
                    continue;
                }
            }
            _ => continue,
        };
        pieces.insert(PipePiece {
            pipe: pipe.clone(),
            x: next.0,
            y: next.1,
        });
        return Some(next);
    }
    None
}

/// Helper function to make it easier to retrieve a tile by i32 coordinates.
/// Returns None if out of bounds
fn get_pipe(pipes: &Pipes, x: i32, y: i32) -> Option<&Pipe> {
    let x = match usize::try_from(x) {
        Ok(v) => v,
        Err(_) => return None,
    };
    let y = match usize::try_from(y) {
        Ok(v) => v,
        Err(_) => return None,
    };
    pipes.get(y, x)
}

/// Count the amount of tiles from the specified type in the map
fn count_tiles(pipes: &Pipes, value: &Pipe) -> u64 {
    pipes
        .rows_iter()
        .map(|row| u64::try_from(row.filter(|p| *p == value).count()).unwrap())
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10p1_run_1() {
        let input = r#"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "#;
        assert_eq!(run1(input), 4);
    }

    #[test]
    fn test_day10p1_run_2() {
        let input = r#"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        "#;
        assert_eq!(run1(input), 4);
    }

    #[test]
    fn test_day10p1_run_3() {
        let input = r#"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        "#;
        assert_eq!(run1(input), 8);
    }

    #[test]
    fn test_day10p1_run_4() {
        let input = r#"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "#;
        assert_eq!(run1(input), 8);
    }

    // PART 2

    #[test]
    fn test_day10p2_clean_pipes() {
        let input = r#"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "#;
        let pipes = parse_pipes(input.trim());
        assert_eq!(count_tiles(&pipes, &Pipe::None), 2);
        let pieces = get_pieces(&pipes);
        let pipes = clean_pipes(&pipes, &pieces);
        assert_eq!(count_tiles(&pipes, &Pipe::None), 9);
    }

    #[test]
    fn test_day10p2_flood_fill() {
        let input = r#"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "#;
        let mut pipes = parse_pipes(input.trim());
        flood_fill(&mut pipes, (0, 0), &Pipe::Outer);
        assert_eq!(count_tiles(&pipes, &Pipe::Outer), 16);
    }

    #[test]
    fn test_day10p2_blow_up_1() {
        let input = r#"
        ..
        .S
        "#;
        let mut pipes = parse_pipes(input.trim());
        let pipes = blow_up(&pipes);
        assert_eq!(count_tiles(&pipes, &Pipe::Start), 9);
        assert_eq!(count_tiles(&pipes, &Pipe::None), 27);
    }

    #[test]
    fn test_day10p2_blow_up_2() {
        let input = r#"
        S-7
        |.|
        L-J
        "#;
        let mut pipes = parse_pipes(input.trim());
        let pipes = blow_up(&pipes);
        assert_eq!(count_tiles(&pipes, &Pipe::Start), 9);
        assert_eq!(count_tiles(&pipes, &Pipe::Horizontal), 9);
        assert_eq!(count_tiles(&pipes, &Pipe::Vertical), 9);
        assert_eq!(count_tiles(&pipes, &Pipe::TopRight), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::TopLeft), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::BottomLeft), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::None), 51);
    }

    #[test]
    fn test_day10p2_shrink_down_1() {
        let input = r#"
        ..
        .S
        "#;
        let mut pipes = parse_pipes(input.trim());
        let pipes = blow_up(&pipes);
        let pipes = shrink_down(&pipes);
        assert_eq!(count_tiles(&pipes, &Pipe::Start), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::None), 3);
    }

    #[test]
    fn test_day10p2_shrink_down_2() {
        let input = r#"
        S-7
        |.|
        L-J
        "#;
        let mut pipes = parse_pipes(input.trim());
        let pipes = blow_up(&pipes);
        let pipes = shrink_down(&pipes);
        assert_eq!(count_tiles(&pipes, &Pipe::Start), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::Horizontal), 2);
        assert_eq!(count_tiles(&pipes, &Pipe::Vertical), 2);
        assert_eq!(count_tiles(&pipes, &Pipe::TopRight), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::TopLeft), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::BottomLeft), 1);
        assert_eq!(count_tiles(&pipes, &Pipe::None), 1);
    }

    #[test]
    fn test_day10p2_run_1() {
        let input = r#"
        .....
        .S-7.
        .|.|.
        .L-J.
        .....
        "#;
        assert_eq!(run2(input), 1);
    }

    #[test]
    fn test_day10p2_run_2() {
        let input = r#"
        -L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF
        "#;
        assert_eq!(run2(input), 1);
    }

    #[test]
    fn test_day10p2_run_3() {
        let input = r#"
        ..F7.
        .FJ|.
        SJ.L7
        |F--J
        LJ...
        "#;
        assert_eq!(run2(input), 1);
    }

    #[test]
    fn test_day10p2_run_4() {
        let input = r#"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
        "#;
        assert_eq!(run2(input), 1);
    }

    #[test]
    fn test_day10p2_run_5() {
        let input = r#"
        ...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ...........
        "#;
        assert_eq!(run2(input), 4);
    }

    #[test]
    fn test_day10p2_run_6() {
        let input = r#"
        ..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        ..........
        "#;
        assert_eq!(run2(input), 4);
    }

    #[test]
    fn test_day10p2_run_7() {
        let input = r#"
        .F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ...
        "#;
        assert_eq!(run2(input), 8);
    }

    #[test]
    fn test_day10p2_run_8() {
        let input = r#"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        "#;
        assert_eq!(run2(input), 10);
    }
}
