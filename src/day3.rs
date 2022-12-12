use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solve() {

    let (mut part_one, mut part_two) = (0, 0);

    let file = File::open("input\\day3.txt").unwrap();
    let reader = BufReader::new(file);

    let mut group = vec![];

    for line in reader.lines() {

        let backpack = line.unwrap();

        group.push(backpack.clone());

        if group.len() == 3 {
            for x in group[0].chars() {
                if group[1].contains(x) && group[2].contains(x) {
                    part_two += trans(x);
                    break;
                }
            }
            group.clear();
        }

        let (left,right) = backpack.split_at(backpack.len()/2);

        for x in left.chars() {
            if right.contains(x) {
                part_one += trans(x);
                break;
            }
        }
    }
    println!("Part 1: {}, Part 2: {}",part_one,part_two);
}

fn trans(x: char) -> u32 {
    if (x as u32) > 90 { x as u32 - 'a' as u32 + 1 }
    else { x as u32 - 'A' as u32 + 27 }
}