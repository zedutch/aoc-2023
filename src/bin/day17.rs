#![allow(unused)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    part1();
}

fn part1() {
    let input = include_str!("./day17.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> usize {
    let map: Map = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .filter_map(|d| usize::try_from(d).ok())
                .collect()
        })
        .collect();

    let start = (0, 0);
    let goal = (map.len() - 1, map[0].len() - 1);

    calc_cost(start, goal, &map)
}

fn part2() {
    let input = include_str!("./day17.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> usize {
    let map: Map = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .filter_map(|d| usize::try_from(d).ok())
                .collect()
        })
        .collect();

    let start = (0, 0);
    let goal = (map.len() - 1, map[0].len() - 1);

    calc_cost(start, goal, &map)
}

type Position = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
    position: Position,
    remaining_up: u32,
    remaining_right: u32,
    remaining_down: u32,
    remaining_left: u32,
    moves_in_current_direction: u32,
    direction: Direction,
}

type Map = Vec<Vec<usize>>;

fn get_neighbours(state: State, map: &Map) -> Vec<State> {
    let mut neighbours: Vec<State> = Vec::new();

    if state.moves_in_current_direction < 4 {
        match state.direction {
            Direction::Up => {
                if state.position.0 >= 1 {
                    neighbours.push(State {
                        position: (state.position.0 - 1, state.position.1),
                        remaining_up: state.remaining_up - 1,
                        remaining_right: 10,
                        remaining_down: 10,
                        remaining_left: 10,
                        moves_in_current_direction: state.moves_in_current_direction + 1,
                        direction: Direction::Up,
                    });
                }
                return neighbours;
            }
            Direction::Right => {
                if state.position.1 < map[0].len() - 1 {
                    neighbours.push(State {
                        position: (state.position.0, state.position.1 + 1),
                        remaining_up: 10,
                        remaining_right: state.remaining_right - 1,
                        remaining_down: 10,
                        remaining_left: 10,
                        moves_in_current_direction: state.moves_in_current_direction + 1,
                        direction: Direction::Right,
                    });
                }
                return neighbours;
            }
            Direction::Down => {
                if state.position.0 < map.len() - 1 {
                    neighbours.push(State {
                        position: (state.position.0 + 1, state.position.1),
                        remaining_up: 10,
                        remaining_right: 10,
                        remaining_down: state.remaining_down - 1,
                        remaining_left: 10,
                        moves_in_current_direction: state.moves_in_current_direction + 1,
                        direction: Direction::Down,
                    });
                }
                return neighbours;
            }
            Direction::Left => {
                if state.position.1 >= 1 {
                    neighbours.push(State {
                        position: (state.position.0, state.position.1 - 1),
                        remaining_up: 10,
                        remaining_right: 10,
                        remaining_down: 10,
                        remaining_left: state.remaining_left - 1,
                        moves_in_current_direction: state.moves_in_current_direction + 1,
                        direction: Direction::Left,
                    });
                }
                return neighbours;
            }
        }
    }

    if state.remaining_up >= 1 && state.position.0 >= 1 && state.direction != Direction::Down {
        neighbours.push(State {
            position: (state.position.0 - 1, state.position.1),
            remaining_up: state.remaining_up - 1,
            remaining_right: 10,
            remaining_down: 10,
            remaining_left: 10,
            moves_in_current_direction: if state.direction == Direction::Up {
                state.moves_in_current_direction + 1
            } else {
                1
            },
            direction: Direction::Up,
        });
    }

    if state.remaining_right >= 1
        && state.position.1 < map[0].len() - 1
        && state.direction != Direction::Left
    {
        neighbours.push(State {
            position: (state.position.0, state.position.1 + 1),
            remaining_up: 10,
            remaining_right: state.remaining_right - 1,
            remaining_down: 10,
            remaining_left: 10,
            moves_in_current_direction: if state.direction == Direction::Right {
                state.moves_in_current_direction + 1
            } else {
                1
            },
            direction: Direction::Right,
        });
    }

    if state.remaining_down >= 1
        && state.position.0 < map.len() - 1
        && state.direction != Direction::Up
    {
        neighbours.push(State {
            position: (state.position.0 + 1, state.position.1),
            remaining_up: 10,
            remaining_right: 10,
            remaining_down: state.remaining_down - 1,
            remaining_left: 10,
            moves_in_current_direction: if state.direction == Direction::Down {
                state.moves_in_current_direction + 1
            } else {
                1
            },
            direction: Direction::Down,
        });
    }

    if state.remaining_left >= 1 && state.position.1 >= 1 && state.direction != Direction::Right {
        neighbours.push(State {
            position: (state.position.0, state.position.1 - 1),
            remaining_up: 10,
            remaining_right: 10,
            remaining_down: 10,
            remaining_left: state.remaining_left - 1,
            moves_in_current_direction: if state.direction == Direction::Left {
                state.moves_in_current_direction + 1
            } else {
                1
            },
            direction: Direction::Left,
        });
    }

    neighbours
}

fn calc_cost(from: Position, to: Position, map: &Map) -> usize {
    let mut costs: HashMap<State, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let start_state_right = State {
        position: from,
        remaining_up: 10,
        remaining_right: 10,
        remaining_down: 10,
        remaining_left: 10,
        moves_in_current_direction: 0,
        direction: Direction::Right,
    };
    let start_state_down = State {
        position: from,
        remaining_up: 10,
        remaining_right: 10,
        remaining_down: 10,
        remaining_left: 10,
        moves_in_current_direction: 0,
        direction: Direction::Down,
    };

    costs.insert(start_state_right, 0);
    costs.insert(start_state_down, 0);
    heap.push(Reverse((0, start_state_right)));
    heap.push(Reverse((0, start_state_down)));

    while let Some(Reverse((curr_cost, curr_state))) = heap.pop() {
        if curr_state.position == to && curr_state.moves_in_current_direction >= 4 {
            return curr_cost;
        }

        if curr_cost > *costs.get(&curr_state).unwrap_or(&usize::MAX) {
            continue;
        }

        let neighbours = get_neighbours(curr_state, map);

        for neighbour in neighbours {
            let next_pos = neighbour.position;
            let next_cost = curr_cost + map[next_pos.0][next_pos.1];
            if next_cost < *costs.get(&neighbour).unwrap_or(&usize::MAX) {
                heap.push(Reverse((next_cost, neighbour)));
                costs.insert(neighbour, next_cost);
            }
        }
    }

    usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17p1_run() {
        let input = r#"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "#;
        assert_eq!(run1(input), 102);
    }

    #[test]
    fn test_day17p2_run_1() {
        let input = r#"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "#;
        assert_eq!(run2(input), 94);
    }

    #[test]
    fn test_day17p2_run_2() {
        let input = r#"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "#;
        assert_eq!(run2(input), 71);
    }
}
