#![allow(unused)]

use std::{collections::HashMap, str::Lines};

use regex::Regex;

fn main() {
    part2();
}
fn part1() {
    let input = include_str!("./day05.in");
    let result = run1(input);
    println!("{result}");
}

fn run1(input: &str) -> u32 {
    let mut sum = 0;
    let maps = build_maps(input);
    input
        // Get a seeds iterator
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        // Parse individual seeds to u32
        .filter_map(|s| s.trim().parse::<u32>().ok())
        // Get their locations...
        .map(|seed| get_location(&maps, seed))
        // ...and find the minimum of all these locations
        .min()
        .unwrap()
}

fn part2() {
    let input = include_str!("./day05.in");
    let result = run2(input);
    println!("{result}");
}

fn run2(input: &str) -> u32 {
    /*
     * Note: this is really slow. It only finished because the compiled code is pretty fast and I could
     * leave it running for an hour without issues. Ideally, this should also be a sparse map,
     * similar to the x_to_y maps and then check if those ranges need to be divied up further.
     * But since I have the answer to my input and I don't have enough time to rewrite it to be better
     * atm, I'm leaving it like this.
     **/
    let mut sum = 0;
    let maps = build_maps(input);
    let numbers = input
        // Get a seeds iterator
        .lines()
        .next()
        .unwrap()
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        // Parse individual seeds to u32
        .filter_map(|s| s.trim().parse::<u32>().ok());
    let seeds: Vec<u32> = numbers.clone().enumerate().filter(|pair| pair.0 % 2 == 0).map(|pair| pair.1).collect();
    let ranges: Vec<u32> = numbers.enumerate().filter(|pair| pair.0 % 2 == 1).map(|pair| pair.1).collect();
    assert_eq!(seeds.len(), ranges.len());
    let mut all_seeds = Vec::new();
    for (i, seed) in seeds.iter().enumerate() {
        let range = ranges[i];
        for j in 0..range {
            all_seeds.push(seed + j);
        }
    }
    all_seeds.iter()
        // Get their locations...
        .map(|seed| get_location(&maps, *seed))
        // ...and find the minimum of all these locations
        .min()
        .unwrap()
}

struct Maps {
    seed_to_soil: Vec<Mapping>,
    soil_to_fertilizer: Vec<Mapping>,
    fertilizer_to_water: Vec<Mapping>,
    water_to_light: Vec<Mapping>,
    light_to_temperature: Vec<Mapping>,
    temperature_to_humidity: Vec<Mapping>,
    humidity_to_location: Vec<Mapping>,
}

struct Mapping {
    src: u32,
    dest: u32,
    range: u32,
}

fn build_maps(input: &str) -> Maps {
    let mut seed_to_soil: Vec<Mapping> = Vec::new();
    let mut soil_to_fertilizer: Vec<Mapping> = Vec::new();
    let mut fertilizer_to_water: Vec<Mapping> = Vec::new();
    let mut water_to_light: Vec<Mapping> = Vec::new();
    let mut light_to_temperature: Vec<Mapping> = Vec::new();
    let mut temperature_to_humidity: Vec<Mapping> = Vec::new();
    let mut humidity_to_location: Vec<Mapping> = Vec::new();

    let mut line_iter = input.trim().lines();
    while let Some(line) = line_iter.next() {
        let line = line.trim();
        match line {
            l if l.starts_with("seed-to-soil") => {
                println!("Seed to soil map");
                seed_to_soil = build_map(line_iter.clone());
            }
            l if l.starts_with("soil-to") => {
                println!("Soil to fertilizer map");
                soil_to_fertilizer = build_map(line_iter.clone());
            }
            l if l.starts_with("fertilizer-to") => {
                println!("Fertilizer to water map");
                fertilizer_to_water = build_map(line_iter.clone());
            }
            l if l.starts_with("water-to") => {
                println!("Water to light map");
                water_to_light = build_map(line_iter.clone());
            }
            l if l.starts_with("light-to") => {
                println!("Light to temperature map");
                light_to_temperature = build_map(line_iter.clone());
            }
            l if l.starts_with("temperature-to") => {
                println!("Temperature to humidity map");
                temperature_to_humidity = build_map(line_iter.clone());
            }
            l if l.starts_with("humidity-to") => {
                println!("Humidity to location map");
                humidity_to_location = build_map(line_iter.clone());
            }
            // Skip the other lines
            _ => (),
        }
    }
    Maps {
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    }
}

fn get_value(mappings: &Vec<Mapping>, key: u32) -> u32 {
    let key_i64 = i64::try_from(key).unwrap();
    for map in mappings {
        let src_i64 = i64::try_from(map.src).unwrap();
        let range_i64 = i64::try_from(map.range).unwrap();
        if src_i64 <= key_i64 && key_i64 - range_i64 < src_i64 {
            return key - map.src + map.dest;
        }
    }
    key
}

fn get_location(maps: &Maps, seed: u32) -> u32 {
    let soil = get_value(&maps.seed_to_soil, seed);
    let ferti = get_value(&maps.soil_to_fertilizer, soil);
    let water = get_value(&maps.fertilizer_to_water, ferti);
    let light = get_value(&maps.water_to_light, water);
    let temp = get_value(&maps.light_to_temperature, light);
    let humid = get_value(&maps.temperature_to_humidity, temp);
    get_value(&maps.humidity_to_location, humid)
}

fn build_map(mut input: Lines) -> Vec<Mapping> {
    let mut map = Vec::new();
    for line in input {
        let line = line.trim();
        if line.is_empty() {
            return map;
        }

        let numbers: Vec<u32> = line
            .split(' ')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect();
        let dest = numbers[0];
        let src = numbers[1];
        let range = numbers[2];
        map.push(Mapping { src, dest, range })
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05p1() {
        let input = r#"seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#;
        let maps = build_maps(input);

        assert_eq!(get_value(&maps.seed_to_soil, 98), 50);
        assert_eq!(get_value(&maps.seed_to_soil, 99), 51);
        assert_eq!(get_value(&maps.seed_to_soil, 53), 55);
        assert_eq!(get_value(&maps.seed_to_soil, 10), 10);
        assert_eq!(get_value(&maps.seed_to_soil, 79), 81);
        assert_eq!(get_value(&maps.seed_to_soil, 14), 14);
        assert_eq!(get_value(&maps.seed_to_soil, 55), 57);
        assert_eq!(get_value(&maps.seed_to_soil, 13), 13);

        assert_eq!(get_location(&maps, 79), 82);
        assert_eq!(get_location(&maps, 14), 43);
        assert_eq!(get_location(&maps, 55), 86);
        assert_eq!(get_location(&maps, 13), 35);

        assert_eq!(run1(input), 35);
    }

    #[test]
    fn test_day05p2() {
        let input = r#"seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "#;
        let maps = build_maps(input);

        assert_eq!(run2(input), 46);
    }
}
