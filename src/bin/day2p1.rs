#[derive(Debug, PartialEq, Default)]
struct Balls {
    r: u32,
    g: u32,
    b: u32,
}

fn main() {
    let input = include_str!("./day2");
    let sum = run(
        input,
        Balls {
            r: 12,
            g: 13,
            b: 14,
        },
    );
    println!("{sum}");
}

fn run(input: &str, max: Balls) -> u32 {
    let mut sum = 0;
    for line in input.trim().lines() {
        let pattern = regex::Regex::new(r"Game (\d+):").unwrap();
        let Some(game) = pattern.captures(line) else {
            panic!("Could not find game declaration: {}", line);
        };
        let game = game[1].parse::<u32>().expect("Could not parse game number");
        let value = get_max(line);
        if value.r <= max.r && value.g <= max.g && value.b <= max.b {
            sum += game;
        }
    }
    sum
}

fn get_max(game: &str) -> Balls {
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
    max
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2p1_max() {
        assert_eq!(get_max("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"), Balls { r: 4, g: 2, b: 6 });
        assert_eq!(get_max("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"), Balls { r: 1, g: 3, b: 4 });
        assert_eq!(get_max("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"), Balls { r: 20, g: 13, b: 6 });
        assert_eq!(get_max("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"), Balls { r: 14, g: 3, b: 15 });
        assert_eq!(get_max("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), Balls { r: 6, g: 3, b: 2 });
    }

    #[test]
    fn test_day2p1_run() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        let sum = run(input, Balls {
        r: 12,
        g: 13,
        b: 14,
    });
        assert_eq!(sum, 8);
    }

}
