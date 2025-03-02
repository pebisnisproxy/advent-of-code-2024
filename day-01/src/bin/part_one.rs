//! # Advent of code Day 1 - Part 1
//! [https://adventofcode.com/2024/day/1](https://adventofcode.com/2024/day/1)
#![allow(unused)]

use day_01::Location;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let (left, right) = Location::new(include_str!("data/location-pairs.txt")).into_parts();

    // Calculate total distance in a single iterator chain
    let total_distance: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(left_item, right_item)| (right_item.get_id() - left_item.get_id()).abs())
        .sum();

    println!("Total Distance: {}", total_distance);
    println!("Take: {}µs", now.elapsed().as_micros());
}
