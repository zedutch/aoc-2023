fn main() {
    let input = include_str!("./day1");
    let sum = part1(input);
    println!("{sum}");
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.trim().lines() {
        let value = get_value(line);
        sum += value;
    }
    sum
}

fn get_value(line: &str) -> u32 {
    let first = line
        .find(|c| char::is_digit(c, 10))
        .expect("No digit found in line: {line}");
    let last = line
        .rfind(|c| char::is_digit(c, 10))
        .expect("No digit found in line: {line}");
    let ifirst = line
        .chars()
        .nth(first)
        .unwrap()
        .to_digit(10)
        .expect("Could not parse as u32");
    let ilast = line
        .chars()
        .nth(last)
        .unwrap()
        .to_digit(10)
        .expect("Could not parse as u32");
    format!("{}{}", ifirst, ilast)
        .parse()
        .expect("Could not parse as u32")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1p1() {
        assert_eq!(get_value("1abc2"), 12);
        assert_eq!(get_value("pqr3stu8vwx"), 38);
        assert_eq!(get_value("a1b2c3d4e5f"), 15);
        assert_eq!(get_value("treb7uchet"), 77);
        assert_eq!(
            part1(
                r#"1abc2
                   pqr3stu8vwx
                   a1b2c3d4e5f
                   treb7uchet
                "#
            ),
            142
        );
    }
}
