#![allow(unused)]

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day09.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        sum += predict_next(line);
    }
    sum
}

fn predict_next(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .split(' ')
        .filter_map(|str| str.trim().parse::<i64>().ok())
        .collect();
    let last = *numbers.last().unwrap();
    last + get_prediction_next(numbers)
}

fn get_prediction_next(input: Vec<i64>) -> i64 {
    let deltas = get_deltas(input);
    if deltas.iter().all(|d| *d == 0) {
        0
    } else {
        let last = *deltas.last().unwrap();
        last + get_prediction_next(deltas)
    }
}


fn part2() {
    let input = include_str!("./day09.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        sum += predict_prev(line);
    }
    sum
}

fn predict_prev(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .split(' ')
        .filter_map(|str| str.trim().parse::<i64>().ok())
        .collect();
    let first = *numbers.first().unwrap();
    first - get_prediction_prev(numbers)
}

fn get_prediction_prev(input: Vec<i64>) -> i64 {
    let deltas = get_deltas(input);
    if deltas.iter().all(|d| *d == 0) {
        0
    } else {
        let first = *deltas.first().unwrap();
        first - get_prediction_prev(deltas)
    }
}

fn get_deltas(input: Vec<i64>) -> Vec<i64> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09p1_run() {
        let input = r#"0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"#;
        assert_eq!(run1(input), 114);
    }

    #[test]
    fn test_day09p1_predict_next() {
        assert_eq!(predict_next("0 3 6 9 12 15"), 18);
        assert_eq!(predict_next("1 3 6 10 15 21"), 28);
        assert_eq!(predict_next("10 13 16 21 30 45"), 68);
    }

    #[test]
    fn test_day09p2_run() {
        let input = r#"0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45"#;
        assert_eq!(run2(input), 2);
    }

    #[test]
    fn test_day09p2_predict_prev() {
        assert_eq!(predict_prev("0 3 6 9 12 15"), -3);
        assert_eq!(predict_prev("1 3 6 10 15 21"), 0);
        assert_eq!(predict_prev("10 13 16 21 30 45"), 5);
    }
}
