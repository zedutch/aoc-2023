#![allow(unused)]

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day04.in");
    let sum = run1(input);
    println!("{sum}");
}

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.trim().lines() {
        sum += calc_card(line);
    }
    sum
}

fn calc_card(card: &str) -> u32 {
    let mut score = 0;
    let numbers = card.split(':').nth(1).unwrap();
    let winning_numbers = numbers
        .split('|')
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok());
    let my_numbers: Vec<u32> = numbers
        .split('|')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();
    for num in winning_numbers {
        if my_numbers.contains(&num) {
            score += match score {
                0 => 1,
                _ => score,
            };
        }
    }
    score
}

fn part2() {
    let input = include_str!("./day04.in");
    let sum = run2(input);
    println!("{sum}");
}

fn run2(input: &str) -> u32 {
    // Pre-allocate enough space for all games
    let game_count = input.trim().lines().count();
    let mut counts: Vec<u32> = Vec::with_capacity(game_count);
    counts.resize(game_count, 1);

    let games = input.trim().lines().filter_map(|l| l.split(':').nth(1).map(|s| s.trim()));
    for (idx, game) in games.enumerate() {
        let wins: usize = number_of_wins(game).try_into().unwrap();
        for i in 1..=wins {
            counts[idx + i] += counts[idx];
        }
    }

    counts.into_iter().sum()
}

fn number_of_wins(numbers: &str) -> u32 {
    let mut wins = 0;
    let winning_numbers = numbers
        .split('|')
        .next()
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok());
    let my_numbers: Vec<u32> = numbers
        .split('|')
        .nth(1)
        .unwrap()
        .trim()
        .split(' ')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .collect();
    for num in winning_numbers {
        if my_numbers.contains(&num) {
            wins += 1;
        }
    }
    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day04p1_cards() {
        assert_eq!(
            calc_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            calc_card("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            calc_card("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            calc_card("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            calc_card("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            calc_card("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    #[test]
    fn test_day04p1_run() {
        assert_eq!(
            run1(
                r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            ),
            13
        );
    }

    #[test]
    fn test_day04p2_cards() {
        assert_eq!(
            number_of_wins("41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            4
        );
        assert_eq!(
            number_of_wins("13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            number_of_wins(" 1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            number_of_wins("41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            number_of_wins("87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            number_of_wins("31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }

    #[test]
    fn test_day04p2_run() {
        assert_eq!(
            run2(
                r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            ),
            30
        );
    }
}
