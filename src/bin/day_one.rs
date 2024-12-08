use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part_one() -> io::Result<()> {
    let file = File::open(".\\files\\day_one.txt")?;
    let mut reader = BufReader::new(file);

    let mut col1 = vec![];
    let mut col2 = vec![];

    let mut string = String::new();

    while reader.read_line(&mut string)? > 0 {
        let numbers: Vec<&str> = string.split_whitespace().collect();

        if numbers.len() != 2 {
            panic!("Line {} has {} numbers, not valid", string, numbers.len());
        }

        col1.push(numbers[0].parse::<u32>().unwrap());
        col2.push(numbers[1].parse::<u32>().unwrap());

        string.clear();
    }

    col1.sort();
    col2.sort();

    let mut total_distance: u32 = 0;

    for i in 0..col1.len() {
        total_distance += col1[i].abs_diff(col2[i]);
    }

    println!("{}", total_distance);

    Ok(())
}

fn part_two() -> io::Result<()> {
    let file = File::open(".\\files\\day_one.txt")?;
    let mut reader = BufReader::new(file);

    let mut col1 = vec![];
    let mut map2 = HashMap::new();

    let mut string = String::new();

    while reader.read_line(&mut string)? > 0 {
        let numbers: Vec<&str> = string.split_whitespace().collect();

        if numbers.len() != 2 {
            panic!("Line {} has {} numbers, not valid", string, numbers.len());
        }

        col1.push(numbers[0].parse::<u32>().unwrap());
        map2.entry(numbers[1].parse::<u32>().unwrap()).and_modify(|x| *x += 1).or_insert(1);

        string.clear();
    }

    let mut total: u32 = 0;

    for number in col1 {
        if let Some(n) = map2.get(&number) {
            total += number * n;
        }
    }

    println!("{}", total);

    Ok(())
}

fn main() -> io::Result<()> {
    part_one().expect("Failed to run day one part one");
    part_two()
}