use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve() {

    let file = File::open("input\\day2.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut part_one, mut part_two) = (0,0);

    for line in reader.lines() {

        let tokens: Vec<char> = line.unwrap().chars().collect();

        let first = tokens[0] as i32 - 'A' as i32;
        let second = tokens[2] as i32 - 'X' as i32;

        match (((first-second) % 3) + 3) % 3 {
            0 => part_one += second+4,
            1 => part_one += second+1,
            _ => part_one += second+7
        }

        match second {
            0 => part_two += 1 + (first+2) % 3,
            1 => part_two += 4 + first,
            _ => part_two += 7 + (first+1) % 3
        }
    }
    println!("Part 1: {}, Part 2: {}", part_one, part_two);
}