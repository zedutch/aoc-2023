#![allow(unused)]

use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};

fn main() {
    let input = include_str!("./day22.in");
    let result = part2(input);
    println!("{result}");
}

fn part1(input: &str) -> usize {
    let mut safe_bricks: BTreeSet<usize> = BTreeSet::new();
    let bricks = parse_input(input);
    let bricks = drop_all_bricks(&bricks);
    let mut supports: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut supporting: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &brick in bricks.iter() {
        let above = find_bricks_above(brick, &bricks);
        let below = find_bricks_below(brick, &bricks);
        let above = above.iter().map(|b| b.id).collect::<BTreeSet<_>>();
        let below = below.iter().map(|b| b.id).collect::<BTreeSet<_>>();
        supports.insert(brick.id, below);
        supporting.insert(brick.id, above);
    }
    for brick in bricks {
        println!(
            "Brick {} supported by: {:?}, supports: {:?}",
            brick.id,
            supports.get(&brick.id).unwrap(),
            supporting.get(&brick.id).unwrap()
        );
        match supporting.get(&brick.id) {
            Some(list) if list.is_empty() => {
                safe_bricks.insert(brick.id);
            }
            Some(list) => {
                if list.iter().all(|&b| supports.get(&b).unwrap().len() > 1) {
                    safe_bricks.insert(brick.id);
                }
            }
            None => (),
        }
    }
    println!("Safe bricks: {safe_bricks:?}");
    safe_bricks.len()
}

fn part2(input: &str) -> usize {
    let bricks = parse_input(input);
    let bricks = drop_all_bricks(&bricks);
    let mut supports: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut supporting: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for &brick in bricks.iter() {
        let above = find_bricks_above(brick, &bricks);
        let below = find_bricks_below(brick, &bricks);
        let above = above.iter().map(|b| b.id).collect::<BTreeSet<_>>();
        let below = below.iter().map(|b| b.id).collect::<BTreeSet<_>>();
        supports.insert(brick.id, below);
        supporting.insert(brick.id, above);
    }
    let mut fallen_bricks: Vec<usize> = Vec::new();
    for brick in bricks {
        fallen_bricks.push(remove_brick(brick.id, supports.clone(), supporting.clone()));
    }
    fallen_bricks.iter().sum()
}

fn remove_brick(
    brick_id: usize,
    mut supports: BTreeMap<usize, BTreeSet<usize>>,
    supporting: BTreeMap<usize, BTreeSet<usize>>,
) -> usize {
    let mut yeeted: BTreeSet<usize> = BTreeSet::new();
    println!();
    println!(
        "Brick {} supported by: {:?}, supports: {:?}",
        brick_id,
        supports.get(&brick_id).unwrap(),
        supporting.get(&brick_id).unwrap()
    );
    let mut todo = VecDeque::new();
    todo.push_back(brick_id);
    while let Some(removed) = todo.pop_front() {
        remove_support(removed, &mut supports);
        let mut supported = supporting.get(&removed).unwrap().clone();
        supported.retain(|&b| supports.get(&b).unwrap().is_empty());
        println!("Yeeting {removed} -> now unsupported: {supported:?}");
        yeeted.extend(supported.iter());
        todo.extend(supported);
    }
    println!("Yeeted: {yeeted:?}");
    yeeted.len()
}

fn remove_support(brick_id: usize, supports: &mut BTreeMap<usize, BTreeSet<usize>>) {
    for (_, list) in supports.iter_mut() {
        list.remove(&brick_id);
    }
}

fn find_bricks_below(brick: Brick, bricks: &[Brick]) -> HashSet<Brick> {
    let mut supporting_bricks = HashSet::new();
    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            for z in brick.start.z..=brick.end.z {
                let tile = Brick {
                    id: brick.id,
                    start: Coord { x, y, z: z - 1 },
                    end: Coord { x, y, z: z - 1 },
                };
                let bricks_above = find_bricks_colliding(tile, bricks);
                supporting_bricks.extend(bricks_above);
            }
        }
    }
    supporting_bricks
}

fn find_bricks_above(brick: Brick, bricks: &[Brick]) -> HashSet<Brick> {
    let mut supporting_bricks = HashSet::new();
    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            for z in brick.start.z..=brick.end.z {
                let tile = Brick {
                    id: brick.id,
                    start: Coord { x, y, z: z + 1 },
                    end: Coord { x, y, z: z + 1 },
                };
                let bricks_above = find_bricks_colliding(tile, bricks);
                supporting_bricks.extend(bricks_above);
            }
        }
    }
    supporting_bricks
}

fn find_bricks_colliding(brick: Brick, bricks: &[Brick]) -> Vec<&Brick> {
    bricks
        .iter()
        .filter(|b| b.id != brick.id)
        .filter(|b| {
            (brick.start.x <= b.end.x && brick.end.x >= b.start.x)
                && (brick.start.y <= b.end.y && brick.end.y >= b.start.y)
                && (brick.start.z <= b.end.z && brick.end.z >= b.start.z)
        })
        .collect()
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .filter_map(|line| (!line.trim().is_empty()).then_some(line.trim()))
        .enumerate()
        .map(|(id, line)| {
            let mut coords = line.split('~');
            let start = coords
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let end = coords
                .next()
                .unwrap()
                .split(',')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Brick {
                id,
                start: Coord {
                    x: start[0],
                    y: start[1],
                    z: start[2],
                },
                end: Coord {
                    x: end[0],
                    y: end[1],
                    z: end[2],
                },
            }
        })
        .collect()
}

fn drop_all_bricks(bricks: &[Brick]) -> Vec<Brick> {
    let (mut dropped, mut bricks) = drop_bricks_tick(bricks);
    // Keep dropping untill no bricks changed
    while dropped {
        let new_bricks: Vec<Brick>;
        (dropped, new_bricks) = drop_bricks_tick(&bricks);
        bricks = new_bricks;
    }
    bricks
}

fn drop_bricks_tick(bricks: &[Brick]) -> (bool, Vec<Brick>) {
    let mut moved = false;
    let mut moved_bricks = bricks.to_vec();
    for &brick in bricks.iter().filter(|b| b.start.z > 1 && b.end.z > 1) {
        // Create a copy that's moved down 1 cell
        let mut new_brick = brick;
        new_brick.start.z -= 1;
        new_brick.end.z -= 1;
        // Check if it collides with any other bricks
        if !collides(new_brick, &moved_bricks) {
            moved_bricks.iter_mut().for_each(|b| {
                if b.id == new_brick.id {
                    b.start.z = new_brick.start.z;
                    b.end.z = new_brick.end.z;
                }
            });
            moved = true;
        }
    }
    (moved, moved_bricks)
}

fn collides(brick: Brick, bricks: &[Brick]) -> bool {
    bricks.iter().filter(|b| b.id != brick.id).any(|&b| {
        (brick.start.x <= b.end.x && brick.end.x >= b.start.x)
            && (brick.start.y <= b.end.y && brick.end.y >= b.start.y)
            && (brick.start.z <= b.end.z && brick.end.z >= b.start.z)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    id: usize,
    start: Coord,
    end: Coord,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day22p1_example() {
        let input = r#"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "#;
        assert_eq!(part1(input), 5);
    }

    #[test]
    pub fn test_day22p1_drop() {
        let input = r#"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "#;
        let bricks = parse_input(input);
        assert_eq!(
            bricks,
            vec![
                Brick {
                    id: 0,
                    start: Coord { x: 1, y: 0, z: 1 },
                    end: Coord { x: 1, y: 2, z: 1 }
                },
                Brick {
                    id: 1,
                    start: Coord { x: 0, y: 0, z: 2 },
                    end: Coord { x: 2, y: 0, z: 2 }
                },
                Brick {
                    id: 2,
                    start: Coord { x: 0, y: 2, z: 3 },
                    end: Coord { x: 2, y: 2, z: 3 }
                },
                Brick {
                    id: 3,
                    start: Coord { x: 0, y: 0, z: 4 },
                    end: Coord { x: 0, y: 2, z: 4 }
                },
                Brick {
                    id: 4,
                    start: Coord { x: 2, y: 0, z: 5 },
                    end: Coord { x: 2, y: 2, z: 5 }
                },
                Brick {
                    id: 5,
                    start: Coord { x: 0, y: 1, z: 6 },
                    end: Coord { x: 2, y: 1, z: 6 }
                },
                Brick {
                    id: 6,
                    start: Coord { x: 1, y: 1, z: 8 },
                    end: Coord { x: 1, y: 1, z: 9 }
                }
            ]
        );

        let bricks = drop_all_bricks(&bricks);
        let solution = vec![
            Brick {
                id: 0,
                start: Coord { x: 1, y: 0, z: 1 },
                end: Coord { x: 1, y: 2, z: 1 },
            },
            Brick {
                id: 1,
                start: Coord { x: 0, y: 0, z: 2 },
                end: Coord { x: 2, y: 0, z: 2 },
            },
            Brick {
                id: 2,
                start: Coord { x: 0, y: 2, z: 2 },
                end: Coord { x: 2, y: 2, z: 2 },
            },
            Brick {
                id: 3,
                start: Coord { x: 0, y: 0, z: 3 },
                end: Coord { x: 0, y: 2, z: 3 },
            },
            Brick {
                id: 4,
                start: Coord { x: 2, y: 0, z: 3 },
                end: Coord { x: 2, y: 2, z: 3 },
            },
            Brick {
                id: 5,
                start: Coord { x: 0, y: 1, z: 4 },
                end: Coord { x: 2, y: 1, z: 4 },
            },
            Brick {
                id: 6,
                start: Coord { x: 1, y: 1, z: 5 },
                end: Coord { x: 1, y: 1, z: 6 },
            },
        ];
        bricks.iter().zip(solution.iter()).for_each(|(a, b)| {
            if a != b {
                println!("Not equal: {a:?}, {b:?}")
            }
            assert_eq!(a, b);
        });
    }

    #[test]
    pub fn test_day22p2_example() {
        let input = r#"
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        "#;
        assert_eq!(part2(input), 7);
    }
}
