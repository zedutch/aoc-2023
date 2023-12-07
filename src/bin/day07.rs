#![allow(unused)]

use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

fn main() {
    part2();
}

fn part1() {
    let input = include_str!("./day07.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> u32 {
    let mut bids: HashMap<&str, u32> = HashMap::new();
    let mut hands: Vec<&str> = Vec::new();
    let total_ranks = input.trim().lines().count();
    for line in input.trim().lines() {
        let mut line = line.trim().split(' ');
        let hand = line.next().unwrap();
        let bid = line.next().unwrap();
        bids.insert(hand, bid.parse().unwrap());
        hands.push(hand);
    }
    hands.sort_unstable_by(cmp_hands_pt1);
    let mut result = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let bid = bids.get(hand).unwrap();
        let rank = u32::try_from(total_ranks - idx).unwrap();
        println!("Hand: {hand}, rank: {rank}, bid: {bid}");
        result += bid * rank;
    }
    result
}

fn part2() {
    let input = include_str!("./day07.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> u32 {
    let mut bids: HashMap<&str, u32> = HashMap::new();
    let mut hands: Vec<&str> = Vec::new();
    let total_ranks = input.trim().lines().count();
    for line in input.trim().lines() {
        let mut line = line.trim().split(' ');
        let hand = line.next().unwrap();
        let bid = line.next().unwrap();
        bids.insert(hand, bid.parse().unwrap());
        hands.push(hand);
    }
    hands.sort_unstable_by(cmp_hands_pt2);
    let mut result = 0;
    for (idx, hand) in hands.into_iter().enumerate() {
        let bid = bids.get(hand).unwrap();
        let rank = u32::try_from(total_ranks - idx).unwrap();
        println!("Hand: {hand}, rank: {rank}, bid: {bid}");
        result += bid * rank;
    }
    result
}

fn cmp_hands_pt1(a: &&str, b: &&str) -> Ordering {
    let equal_cards_a = get_num_equal(a);
    let order = get_num_equal(b).cmp(&equal_cards_a);
    if order != Ordering::Equal {
        return order;
    } else if equal_cards_a == 3 {
        match (is_full_house(a), is_full_house(b)) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => (),
        }
    } else if equal_cards_a == 2 {
        match (is_two_pair(a), is_two_pair(b)) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => (),
        }
    }

    for pair in a.chars().zip(b.chars()) {
        let a = pair.0;
        let b = pair.1;

        let order = card_value_pt1(b).cmp(&card_value_pt1(a));
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn cmp_hands_pt2(a: &&str, b: &&str) -> Ordering {
    let equal_cards_a = get_num_equal_joker(a);
    let order = get_num_equal_joker(b).cmp(&equal_cards_a);
    if order != Ordering::Equal {
        return order;
    } else if equal_cards_a == 3 {
        match (is_full_house_joker(a), is_full_house_joker(b)) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => (),
        }
    } else if equal_cards_a == 2 {
        // This does not need a joker version as that would always lead to 3 of a kind instead
        match (is_two_pair(a), is_two_pair(b)) {
            (true, false) => return Ordering::Less,
            (false, true) => return Ordering::Greater,
            _ => (),
        }
    }

    for pair in a.chars().zip(b.chars()) {
        let a = pair.0;
        let b = pair.1;

        let order = card_value_pt2(b).cmp(&card_value_pt2(a));
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}

fn get_char_map(hand: &&str) -> BTreeMap<char, u32> {
    let mut map: BTreeMap<char, u32> = BTreeMap::new();
    for c in hand.chars() {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    map
}

fn get_jokerless_map(hand: &&str) -> BTreeMap<char, u32> {
    let mut map: BTreeMap<char, u32> = BTreeMap::new();
    for c in hand.chars().filter(|c| c != &'J') {
        map.entry(c).and_modify(|v| *v += 1).or_insert(1);
    }
    map
}

fn get_num_equal(hand: &&str) -> u32 {
    let mut map = get_char_map(hand);
    *map.values().max().unwrap_or(&1)
}

fn is_full_house(hand: &&str) -> bool {
    let mut map = get_char_map(hand);
    let mut values = map.values();
    let mut values: Vec<&u32> = map.values().collect();
    values.sort_unstable();
    let count = values.len();

    count == 2 && *values[0] == 2 && *values[1] == 3
}

fn is_two_pair(hand: &&str) -> bool {
    let mut map = get_char_map(hand);
    let mut values: Vec<&u32> = map.values().collect();
    values.sort_unstable();
    let count = values.len();

    count == 3 && *values[0] == 1 && *values[1] == 2 && *values[2] == 2
}

fn get_num_equal_joker(hand: &&str) -> u32 {
    let mut map_nojoker = get_jokerless_map(hand);
    let mut map = get_char_map(hand);
    let max = *map_nojoker.values().max().unwrap_or(&0);
    let jokers = *map.get(&'J').unwrap_or(&0);
    max + jokers
}

fn is_full_house_joker(hand: &&str) -> bool {
    let mut map = get_char_map(hand);
    let jokers = *map.get(&'J').unwrap_or(&0);
    match jokers {
        0 => is_full_house(hand),
        1 => is_two_pair(hand), // 2 pairs + joker = full house
        _ => false, // 2 or more jokers can never lead to full house
    }
}

fn card_value_pt1(card: char) -> u32 {
    match (card) {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap(),
    }
}

fn card_value_pt2(card: char) -> u32 {
    match (card) {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        'J' => 1,
        _ => card.to_digit(10).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07_utils() {
        assert!(is_full_house(&"11222"));
        assert!(is_full_house(&"22121"));
        assert!(!is_full_house(&"22122"));
        assert!(!is_full_house(&"23112"));

        assert!(is_two_pair(&"11223"));
        assert!(is_two_pair(&"12132"));
        assert!(!is_two_pair(&"12122"));
        assert!(!is_two_pair(&"12121"));
    }

    #[test]
    fn test_day07p1_run() {
        assert_eq!(
            run1(
                r#"32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#
            ),
            6440
        );

        // Test full houses
        assert_eq!(
            run1(
                r#"33322 1
            33232 10
            22333 100
            33344 1000
        "#
            ),
            4123
        );

        // Test two pairs
        assert_eq!(
            run1(
                r#"11222 1
            11223 10
            22134 100
        "#
            ),
            123
        );
    }

    #[test]
    fn test_day07p2_run() {
        assert_eq!(
            run2(
                r#"32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        "#
            ),
            5905
        );
    }

    #[test]
    fn test_day07p2_full_house() {
        assert_eq!(
            run2(
                r#"33J22 1
            3323J 10
            22333 100
            33344 1000
        "#
            ),
            3142
        );
    }

    #[test]
    fn test_day07p2_two_pairs() {
        assert_eq!(
            run2(
                r#"11222 1
            11223 10
            22134 100
            221J4 1000
            1122J 10000
        "#
            ),
            43125
        );
    }

    #[test]
    fn test_day07p2_five_of_a_kind() {
        assert_eq!(
            run2(
                r#"JJJJJ 1
            QJJJJ 10
            JJJJQ 100
        "#
            ),
            231
        );
    }
}
