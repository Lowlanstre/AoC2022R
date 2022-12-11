use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve() {

    let file = File::open("input\\day1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut elves = vec![];
    let mut elf: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<u32>().unwrap();
        }
    }
    elves.sort();
    let part_one = elves.last().unwrap();
    let part_two = elves[elves.len()-1] + elves[elves.len()-2] + elves[elves.len()-3];
    println!("Part 1: {}, Part 2: {}", part_one, part_two);
}