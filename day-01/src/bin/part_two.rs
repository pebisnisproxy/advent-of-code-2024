//! # Advent of code Day 1 - Part 2
//! [https://adventofcode.com/2024/day/1](https://adventofcode.com/2024/day/1#part2)
#![allow(unused)]

use day_01::Location;
use std::{collections::HashMap, time::Instant};

fn main() {
    let now = Instant::now();

    let (left, right) = Location::new(include_str!("data/location-pairs.txt")).into_parts();
    let mut similarity_score = 0;

    // Create a HashMap to store counts of strings in right_strings
    let right_counts: HashMap<String, usize> = right
        .iter()
        .map(|item| item.get_id().to_string())
        .fold(HashMap::new(), |mut acc, item| {
            *acc.entry(item).or_insert(0) += 1;
            acc
        });

    // Calculate similarity score using the HashMap
    for left_item in left.iter() {
        let left_id = left_item.get_id().to_string();
        if let Some(&count) = right_counts.get(&left_id) {
            similarity_score += left_item.get_id() * count as i64;
        }
    }

    println!("Similarity Score: {}", similarity_score);
    println!("Take: {}ms", now.elapsed().as_millis());
}
