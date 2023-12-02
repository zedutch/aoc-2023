#[derive(Debug, PartialEq, Default)]
struct Balls {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let input = include_str!("./day2");
    let sum = run(
        input
   );
    println!("{sum}");
}

fn run(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.trim().lines() {
        let pattern = regex::Regex::new(r"Game (\d+):").unwrap();
        let Some(game) = pattern.captures(line) else {
            panic!("Could not find game declaration: {}", line);
        };
        let game = game[1].parse::<u32>().expect("Could not parse game number");
        let power = get_power(line);
        sum += power;
    }
    sum
}

fn get_power(game: &str) -> u32 {
    let pattern = regex::Regex::new(r"(\d+) (\w+)\b").unwrap();
    let mut max = Balls::default();
    for line in game.split(';') {
        for (_, [num, col]) in pattern.captures_iter(line).map(|c| c.extract()) {
            let num = num.parse::<u32>().expect("Could not parse number");
            let mut totals = Balls::default();
            match col {
                "red" => totals.r += num,
                "green" => totals.g += num,
                "blue" => totals.b += num,
                _ => panic!("Unknown color: {}", col),
            }
            if totals.r > max.r {
                max.r = totals.r;
            }
            if totals.g > max.g {
                max.g = totals.g;
            }
            if totals.b > max.b {
                max.b = totals.b;
            }
        }
    }
    max.r * max.g * max.b
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2p2_power() {
        assert_eq!(get_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), 48);
        assert_eq!(get_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), 12);
        assert_eq!(get_power("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), 1560);
        assert_eq!(get_power("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), 630);
        assert_eq!(get_power("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 36);
    }

    #[test]
    fn test_day2p2_run() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(run(input), 2286);
    }

}
