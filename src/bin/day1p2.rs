fn main() {
    let input = include_str!("./day1");
    let sum = part2(input);
    println!("{sum}");
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.trim().lines() {
        let value = get_value(line.trim());
        sum += value;
    }
    sum
}

fn replace_words(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

fn get_value(line: &str) -> u32 {
    let line = replace_words(line);
    let pattern = regex::Regex::new(r"\d").unwrap();

    let first_match = pattern.find(line.as_str()).unwrap();
    let last_match = pattern
        .find_iter(line.as_str())
        .last()
        .unwrap();

    let first = first_match.as_str().chars().next().unwrap().to_digit(10).unwrap();
    let last = last_match.as_str().chars().next().unwrap().to_digit(10).unwrap();

    format!("{}{}", first, last)
        .parse()
        .expect("Could not parse as u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1p2() {
        assert_eq!(get_value("1abc2"), 12);
        assert_eq!(get_value("pqr3stu8vwx"), 38);
        assert_eq!(get_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_value("treb7uchet"), 77);
        assert_eq!(
            part2(
                r#"1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet
                "#
            ),
            142
        );

        assert_eq!(get_value("two1nine"), 29);
        assert_eq!(get_value("eightwothree"), 83);
        assert_eq!(get_value("abcone2threexyz"), 13);
        assert_eq!(get_value("xtwone3four"), 24);
        assert_eq!(get_value("4nineeightseven2"), 42);
        assert_eq!(get_value("zoneight234"), 14);
        assert_eq!(get_value("7pqrstsixteen"), 76);
        assert_eq!(
            part2(
                r#"two1nine
                   eightwothree
                   abcone2threexyz
                   xtwone3four
                   4nineeightseven2
                   zoneight234
                   7pqrstsixteen
                 "#
            ),
            281
        );

        assert_eq!(get_value("oneight"), 18);
        assert_eq!(get_value("eighthree"), 83);
        assert_eq!(get_value("oneighthree"), 13);
    }
}
