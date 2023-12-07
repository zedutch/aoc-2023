#![allow(unused)]

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day06.in");
    let result = run1(input);
    println!("{result}");
}

fn part2() {
    let input = include_str!("./day06.in");
    let result = run2(input);
    println!("{result}");
}

fn run1(input: &str) -> u64 {
    let mut sum = 0;
    let times = input
        .split('\n')
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok());
    let dist = input
        .split('\n')
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok());
    let mut result = 1;
    for entry in times.zip(dist) {
        result *= race(entry.0, entry.1);
    }
    result
}

fn run2(input: &str) -> u64 {
    // NOTE: This is another day where we can just brute-force the issue because of the fast
    // runtime performance in Rust. Normally you'd try to find the minimum and maximum times to
    // hold down the button and calculate the range between both. But the brute-force way takes
    // less than a second on my laptop so I'm keeping this.
    let mut sum = 0;
    let time = input
        .split('\n')
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.to_string())
        .reduce(|s, acc| [s, acc].concat())
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let dist = input
        .split('\n')
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .map(|s| s.to_string())
        .reduce(|s, acc| [s, acc].concat())
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    race(time, dist)
}

fn race(time: u64, record: u64) -> u64 {
    let mut wins = 0;
    for i in 1..time {
        let distance = i * (time - i);
        if distance > record {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06p1_races() {
        assert_eq!(race(7, 9), 4);
        assert_eq!(race(15, 40), 8);
        assert_eq!(race(30, 200), 9);
    }

    #[test]
    fn test_day06p1_run() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(run1(input), 288);
    }

    #[test]
    fn test_day06p2_run() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(run2(input), 71503);
    }
}
