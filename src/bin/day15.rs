#![allow(unused)]

fn main() {
    part2();
}

type Map = [Vec<(String, u32)>; 256];

fn part1() {
    let input = include_str!("./day15.in");
    let result = run1(input);
    println!("{result}");
}

fn part2() {
    let input = include_str!("./day15.in");
    let result = run2(input);
    println!("{result}");
}

fn run1(input: &str) -> u32 {
    let input = input.trim().lines().next().unwrap();
    input.split(',').map(hash).sum()
}

fn run2(input: &str) -> u32 {
    let map = build_map(input);
    calc_result(map)
}

fn hash(input: &str) -> u32 {
    input
        .chars()
        .map(u32::from)
        .fold(0, |acc, el| ((acc + el) * 17) % 256)
}

fn build_map(input: &str) -> Map {
    const EMPTY: Vec<(String, u32)> = Vec::new();
    let mut map: Map = [EMPTY; 256];
    let input = input.trim().lines().next().unwrap();
    input.split(',').for_each(|c| {
        if let [label, number] = c.split('=').collect::<Vec<&str>>()[..] {
            let hash = hash(label);
            let idx = usize::try_from(hash).unwrap();
            let bucket = map.get_mut(idx).unwrap();
            let num = number.parse::<u32>().expect("Could not convert to number");
            if let Some(old_idx) = bucket
                .iter()
                .enumerate()
                .filter(|(_, (lbl, ..))| lbl == label)
                .map(|(idx, _)| idx)
                .next()
            {
                let entry = bucket.get_mut(old_idx).unwrap();
                entry.1 = num;
            } else {
                bucket.push((label.to_string(), num));
            }
        } else if let Some(label) = c.split('-').next() {
            let hash = hash(label);
            let idx = usize::try_from(hash).unwrap();
            let bucket = map.get_mut(idx).unwrap();
            if let Some(old_idx) = bucket
                .iter()
                .enumerate()
                .filter(|(_, (lbl, ..))| lbl == label)
                .map(|(idx, _)| idx)
                .next()
            {
                bucket.remove(old_idx);
            }
        }
    });
    map
}

fn calc_result(map: Map) -> u32 {
    let mut sum = 0;
    for i in 0..256 {
        let bucket = map.get(i).unwrap();
        let i = u32::try_from(i).unwrap();
        for (j, (_, num)) in bucket.iter().enumerate() {
            let j = u32::try_from(j).unwrap();
            let result = (i + 1) * (j + 1) * *num;
            sum += result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15p1_run() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run1(input), 1320);
    }

    #[test]
    fn test_day15p1_hash() {
        let input = "HASH";
        assert_eq!(hash(input), 52);
    }

    #[test]
    fn test_day15p2_run() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(run2(input), 145);
    }
}
