#![allow(unused)]

use std::collections::{HashMap, HashSet, VecDeque};

use num::integer::lcm;

fn main() {
    let input = include_str!("./day20.in");
    let result = part2(input);
    println!("{result}");
}

fn part1(input: &str) -> usize {
    let mut modules = parse_modules(input);
    let mut cache = HashSet::new();
    let mut cycle = 0;
    let mut pulses = Vec::new();
    loop {
        let state = flatten_state(&modules);
        if cache.contains(&state) || cycle == 1000 {
            break;
        }
        cache.insert(state);
        tick(&mut modules, &mut pulses);
        cycle += 1;
        println!("======{cycle}======")
    }
    let low = pulses.iter().filter(|&&p| !p).count();
    let high = pulses.iter().filter(|&&p| p).count();
    println!("Cycle: {cycle} - low: {low} - high: {high}");
    let repeats = 1000 / cycle;
    low * repeats * high * repeats
}

fn part2(input: &str) -> usize {
    let mut state = parse_modules(input);
    let (cycles_dd, low, high) = get_cycles(state.clone(), "dd", true);
    let (cycles_fh, ..) = get_cycles(state.clone(), "fh", true);
    let (cycles_xp, ..) = get_cycles(state.clone(), "xp", true);
    let (cycles_fc, ..) = get_cycles(state.clone(), "fc", true);
    println!("Cycles: {cycles_dd}, {cycles_fh}, {cycles_xp}, {cycles_fc} - DD: low: {low} - high: {high}");
    lcm(cycles_dd, lcm(cycles_fh, lcm(cycles_xp, cycles_fc)))
}

fn get_cycles(mut state: State, check_for: &str, check_signal: bool) -> (usize, usize, usize) {
    let mut cycle = 0;
    let mut pulses = Vec::new();
    loop {
        if tick_until(&mut state, &mut pulses, check_for, check_signal) {
            cycle += 1;
            break;
        }
        cycle += 1;
    }
    let low = pulses.iter().filter(|&&p| !p).count();
    let high = pulses.iter().filter(|&&p| p).count();
    (cycle, low, high)
}

fn parse_modules(input: &str) -> State {
    let mut modules = HashMap::new();
    let mut conjunctions = Vec::new();
    input
        .trim()
        .lines()
        .filter_map(|line| (!line.is_empty()).then_some(line.trim()))
        .for_each(|line| {
            let mut line = line.split(" -> ");
            let input = line.next().unwrap();
            let output = line.next().unwrap();
            let output: Vec<String> = output.split(", ").map(|s| s.to_string()).collect();
            match input {
                s if s.starts_with('%') => {
                    let name = s.chars().skip(1).collect::<String>();
                    modules.insert(
                        name,
                        Module::FlipFlop {
                            state: false,
                            output,
                        },
                    );
                }
                s if s.starts_with('&') => {
                    let name = s.chars().skip(1).collect::<String>();
                    conjunctions.push((name, output));
                }
                s => {
                    assert_eq!(s, "broadcaster");
                    modules.insert(s.to_string(), Module::Broadcaster { output });
                }
            }
        });
    // @TODO: We don't handle conjunctions leading into other conjunctions, is that an issue?
    for (name, output) in conjunctions {
        let inputs: Vec<String> = modules
            .iter()
            .filter_map(|(n, m)| match m {
                Module::FlipFlop { output, .. }
                | Module::Conjunction { output, .. }
                | Module::Broadcaster { output }
                    if output.contains(&name) =>
                {
                    Some(n.to_owned())
                }
                _ => None,
            })
            .collect();
        let mut state = HashMap::new();
        for input in inputs {
            state.insert(input, false);
        }
        modules.insert(name, Module::Conjunction { state, output });
    }
    modules
}

fn tick(state: &mut State, pulses: &mut Vec<bool>) {
    let mut todos = VecDeque::new();
    todos.push_back((false, "broadcaster".to_string(), "button".to_string()));
    loop {
        let Some((signal, name, previous)) = todos.pop_front() else {
            break;
        };
        if let Some((next_signal, modules)) =
            update_module(name.clone(), signal, previous, state, pulses)
        {
            for module in modules {
                todos.push_back((next_signal, module, name.clone()));
            }
        }
    }
}

fn tick_until(
    state: &mut State,
    pulses: &mut Vec<bool>,
    check_for: &str,
    check_signal: bool,
) -> bool {
    let mut todos = VecDeque::new();
    todos.push_back((false, "broadcaster".to_string(), "button".to_string()));
    loop {
        let Some((signal, name, previous)) = todos.pop_front() else {
            break;
        };
        if previous == check_for && signal == check_signal {
            return true;
        }
        if let Some((next_signal, modules)) =
            update_module(name.clone(), signal, previous, state, pulses)
        {
            for module in modules {
                todos.push_back((next_signal, module, name.clone()));
            }
        }
    }
    false
}

fn update_module(
    module_name: String,
    input: bool,
    pulse_from: String,
    state: &mut State,
    pulses: &mut Vec<bool>,
) -> Option<(bool, Vec<String>)> {
    pulses.push(input);
    // println!(
    //     "{pulse_from} -{}-> {module_name}",
    //     if input { "high" } else { "low" }
    // );
    let Some(module) = state.get_mut(&module_name) else {
        return None;
    };
    // println!("State: {module:?}");
    match module {
        Module::Broadcaster { output } => Some((input, output.clone())),
        Module::FlipFlop {
            ref mut state,
            output,
        } => {
            if !input {
                *state = !*state;
                Some((*state, output.clone()))
            } else {
                None
            }
        }
        Module::Conjunction {
            ref mut state,
            output,
        } => {
            state.insert(pulse_from, input);
            let memory = state.values().all(|&v| v);
            Some((!memory, output.clone()))
        }
    }
}

fn flatten_state(state: &State) -> Vec<bool> {
    let mut result = Vec::new();
    for module in state.values() {
        match module {
            Module::FlipFlop { state, .. } => result.push(*state),
            Module::Conjunction { state, .. } => {
                for value in state.values() {
                    result.push(*value);
                }
            }
            _ => (),
        }
    }
    result
}

type State = HashMap<String, Module>;

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        output: Vec<String>,
    },
    FlipFlop {
        state: bool,
        output: Vec<String>,
    },
    Conjunction {
        state: HashMap<String, bool>,
        output: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_day20p1_example1() {
        let input = r#"
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        "#;
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    pub fn test_day20p1_example2() {
        let input = r#"
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        "#;
        assert_eq!(part1(input), 11687500);
    }
}
