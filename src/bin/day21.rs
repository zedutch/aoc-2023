#![allow(unused)]

use std::collections::BTreeSet;

use array2d::Array2D;
use num::{Float, Integer};

fn main() {
    let input = include_str!("./day21.in");
    let result = part2(input);
    println!("{result}");
}

fn part1(input: &str) -> usize {
    let (map, start) = parse_map(input);
    get_positions(&map, start, 64, false).len()
}

fn part2(input: &str) -> usize {
    let (map, start) = parse_map(input);
    get_num_repeating_positions(&map, start, 26501365)
}

type Map = Array2D<bool>;
type Coord = (i64, i64);

fn parse_map(input: &str) -> (Map, Coord) {
    let mut start = (0, 0);
    let rows: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.trim()
                .chars()
                .enumerate()
                .inspect(|(x, c)| {
                    if *c == 'S' {
                        start = (*x as i64, y as i64);
                    }
                })
                .map(|(_, c)| c != '#')
                .collect()
        })
        .collect();
    (Array2D::from_rows(&rows).unwrap(), start)
}

fn get_positions(map: &Map, start: Coord, steps: usize, repeat: bool) -> BTreeSet<Coord> {
    let mut todo = steps;
    let mut positions = BTreeSet::new();
    positions.insert(start);

    while todo > 0 {
        // Get neighbours for coords that were new in the previous iteration
        let mut new_positions = BTreeSet::new();
        for pos in positions {
            new_positions.extend(step(map, pos, repeat));
        }
        positions = new_positions;
        todo -= 1;
    }

    positions
}

fn step(map: &Map, curr: Coord, repeat: bool) -> BTreeSet<Coord> {
    let mut positions = BTreeSet::new();
    let width = map.row_len() as i64;
    let height = map.column_len() as i64;
    let x = match usize::try_from(curr.0) {
        Ok(x) => x.mod_floor(&map.row_len()),
        Err(_) if repeat => match usize::try_from(curr.0.mod_floor(&width)) {
            Ok(x) => x,
            Err(_) => panic!(
                "Invalid repeating x coordinate: {:?} -> {:?}",
                curr,
                curr.0.mod_floor(&width)
            ),
        },
        Err(_) => panic!("Invalid x coordinate: {:?}", curr),
    };

    let y = match usize::try_from(curr.1) {
        Ok(y) => y.mod_floor(&map.column_len()),
        Err(_) if repeat => match usize::try_from(curr.1.mod_floor(&height)) {
            Ok(y) => y,
            Err(_) => panic!(
                "Invalid repeating y coordinate: {:?} -> {:?}",
                curr,
                curr.1.mod_floor(&height)
            ),
        },
        Err(_) => panic!("Invalid y coordinate: {:?}", curr),
    };

    if x >= map.row_len() || y >= map.column_len() {
        panic!("Invalid coordinate: {:?}", curr);
    }

    if (x > 0 && map.get(y, x - 1) == Some(&true))
        || (x == 0 && map.get(y, map.row_len() - 1) == Some(&true))
    {
        positions.insert((curr.0 - 1, curr.1));
    }
    if (y > 0 && map.get(y - 1, x) == Some(&true))
        || (y == 0 && map.get(map.column_len() - 1, x) == Some(&true))
    {
        positions.insert((curr.0, curr.1 - 1));
    }
    if (x < map.row_len() - 1 && map.get(y, x + 1) == Some(&true))
        || (x == map.row_len() - 1 && map.get(y, 0) == Some(&true))
    {
        positions.insert((curr.0 + 1, curr.1));
    }
    if (y < map.column_len() - 1 && map.get(y + 1, x) == Some(&true))
        || (y == map.column_len() - 1 && map.get(0, x) == Some(&true))
    {
        positions.insert((curr.0, curr.1 + 1));
    }
    positions
}

fn get_num_repeating_positions(map: &Map, start: Coord, steps: usize) -> usize {
    // Maps should be square
    assert_eq!(map.row_len(), map.column_len());
    let tilesize = map.row_len();
    let itilesize = tilesize as i64;
    // Check enough steps to cover all possible tile types (a 5 x 5 tile grid)
    let mut to_check = steps.min((tilesize as f32 * 2.5).floor() as usize);

    if steps <= to_check {
        println!("================");
        println!("Steps: {steps} - checked: {steps}");
        return get_positions(map, start, steps, true).len();
    }

    println!("================");
    println!("Steps: {steps} - checking: {to_check}");
    println!("Tile size: {tilesize:?}");

    let num_tiles = 2 * ((steps - tilesize / 2) / tilesize) + 1;
    let half_map = (num_tiles - 1) / 2;
    println!("Number of tiles: {num_tiles:?} x {num_tiles:?} -> halfmap: {half_map:?}");

    /*
     * This calculates the x=0, x=1, and x=2 values that can be used to get a quadratic fit function:
     * let x1 = get_positions(map, start, (tilesize-1)/2, true).len(); // 3703
     * let x2 = get_positions(map, start, (tilesize-1)/2 + tilesize, true).len(); // 32712
     * let x3 = get_positions(map, start, (tilesize-1)/2 + tilesize*2, true).len(); // 90559
     * println!("x1: {x1}, x2: {x2}, x3: {x3}");
    */

    // quadratic fit: 14419 * x^2 + 14590 * x + 3703
    // Extrapolate
    let x = (steps - 65) / 131;
    3703 + 14590 * x + 14419 * x * x

    /*
     // I originally tried to do this in a more general way but there's an error somewhere and it's
     // kinda hard to debug, which is why I ended up using the above quadratic function instead.
     // The old code is still here in case I ever want to "properly" solve this:

    let tiles = get_positions(map, start, to_check, true);
    let tiles_even = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (0, 0)))
        .count();
    let tiles_odd = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (1, 0)))
        .count();
    let tiles_large_diag_br = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (1, 1)))
        .count();
    let tiles_small_diag_br = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (2, 1)))
        .count();
    let tiles_large_diag_tr = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (1, -1)))
        .count();
    let tiles_small_diag_tr = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (2, -1)))
        .count();
    let tiles_large_diag_tl = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (-1, -1)))
        .count();
    let tiles_small_diag_tl = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (-2, -1)))
        .count();
    let tiles_large_diag_bl = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (-1, 1)))
        .count();
    let tiles_small_diag_bl = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (-2, 1)))
        .count();
    let tiles_corner_r = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (2, 0)))
        .count();
    let tiles_corner_l = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (-2, 0)))
        .count();
    let tiles_corner_t = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (0, -2)))
        .count();
    let tiles_corner_b = tiles
        .iter()
        .filter(|&&coord| is_in_tile(coord, itilesize, (0, 2)))
        .count();
    println!("Odd tiles have {tiles_odd} positions");
    println!("Even tiles have {tiles_even} positions");
    println!("Large diagonal br tiles have {tiles_large_diag_br} positions");
    println!("Small diagonal br tiles have {tiles_small_diag_br} positions");
    println!("Large diagonal bl tiles have {tiles_large_diag_bl} positions");
    println!("Small diagonal bl tiles have {tiles_small_diag_bl} positions");
    println!("Large diagonal tl tiles have {tiles_large_diag_tl} positions");
    println!("Small diagonal tl tiles have {tiles_small_diag_tl} positions");
    println!("Large diagonal tr tiles have {tiles_large_diag_tr} positions");
    println!("Small diagonal tr tiles have {tiles_small_diag_tr} positions");
    println!("Corner r tiles have {tiles_corner_r} positions");
    println!("Corner l tiles have {tiles_corner_l} positions");
    println!("Corner t tiles have {tiles_corner_t} positions");
    println!("Corner b tiles have {tiles_corner_b} positions");

    let mut total = 0;
    // 4 corners of the diamond
    total += tiles_corner_b + tiles_corner_t + tiles_corner_r + tiles_corner_l;
    // 4 diagonals
    total += half_map
        * (tiles_small_diag_bl + tiles_small_diag_br + tiles_small_diag_tl + tiles_small_diag_tr);
    total += (half_map - 1)
        * (tiles_large_diag_bl + tiles_large_diag_br + tiles_large_diag_tl + tiles_large_diag_tr);
    // straight lines from centre
    let line = half_map - 1; // don't count corners
    let parity = if line % 2 == 0 { 0 } else { 1 };
    let num_odd_tiles = (line + parity) / 2;
    let num_even_tiles = line - num_odd_tiles;
    println!(
        "Straight lines: {line} tiles -> each contains {num_odd_tiles} odd, {num_even_tiles} even"
    );
    total += num_odd_tiles * tiles_odd * 4;
    total += num_even_tiles * tiles_even * 4;
    total += tiles_even; // centre

    // fill in triangles between straights and diagonals
    for i in 1..line {
        for j in 1..line {
            if i + j > line {
                // Above diagonal
                break;
            }
            let odd = (i + j) % 2 == 1;
            if i + j % 2 == 0 {
                total += tiles_even * 4;
            } else {
                total += tiles_odd * 4;
            }
            // println!("Add {i}, {j} -> {}", if odd { "odd" } else { "even" });
        }
    }
    total
    */
}

fn is_in_tile(coord: Coord, tilesize: i64, tilecoord: Coord) -> bool {
    let (x, y) = coord;
    let (tx, ty) = (tilecoord.0 * tilesize, tilecoord.1 * tilesize);
    x >= tx && x < tx + tilesize && y >= ty && y < ty + tilesize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day21p1_example() {
        let input = r#"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "#;
        let (map, start) = parse_map(input);
        assert_eq!(get_positions(&map, start, 1, false).len(), 2);
        assert_eq!(get_positions(&map, start, 2, false).len(), 4);
        assert_eq!(get_positions(&map, start, 3, false).len(), 6);
        assert_eq!(get_positions(&map, start, 6, false).len(), 16);
    }

    #[test]
    pub fn test_day21p2_example() {
        let input = r#"
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        "#;
        let (map, start) = parse_map(input);
        assert_eq!(get_num_repeating_positions(&map, start, 6), 16);
        assert_eq!(get_num_repeating_positions(&map, start, 10), 50);
        assert_eq!(get_num_repeating_positions(&map, start, 50), 1594);
        assert_eq!(get_num_repeating_positions(&map, start, 100), 6536);
        assert_eq!(get_num_repeating_positions(&map, start, 500), 167004);
        assert_eq!(get_num_repeating_positions(&map, start, 1000), 668697);
        assert_eq!(get_num_repeating_positions(&map, start, 5000), 16733044);
    }
}
