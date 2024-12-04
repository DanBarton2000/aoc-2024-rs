use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn is_safe(numbers: &Vec<u32>) -> bool {
    let increasing: bool = numbers[1] > numbers[0];
    let mut is_safe: bool = true;

    for i in 1..numbers.len() {
        let direction = numbers[i] > numbers[i-1];
        let difference = numbers[i].abs_diff(numbers[i-1]);

        if direction != increasing || difference < 1 || difference > 3 {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn part_one() -> io::Result<()> {
    let file = File::open(".\\files\\day_two.txt")?;
    let mut reader = BufReader::new(file);

    let mut string = String::new();
    let mut safe_reports: u32 = 0;

    while reader.read_line(&mut string)? > 0 {
        let numbers: Vec<u32> = string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        if numbers.len() < 2 {
            panic!("Line has less than two numbers: {}", string);
        }

        let is_safe = is_safe(&numbers);

        if is_safe {
            safe_reports += 1;
        }

        string.clear();
    }

    println!("{safe_reports}");

    Ok(())
}

fn part_two() -> io::Result<()> {
    let file = File::open(".\\files\\day_two.txt")?;
    let mut reader = BufReader::new(file);

    let mut string = String::new();
    let mut safe_reports: u32 = 0;

    while reader.read_line(&mut string)? > 0 {
        let mut numbers: Vec<u32> = string.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();

        if numbers.len() < 2 {
            panic!("Line has less than two numbers: {}", string);
        }

        let mut is_safe = is_safe(&numbers);

        if !is_safe {
            for i in 0..numbers.len() {
                let mut numbers_clone = numbers.clone();
                numbers_clone.remove(i);

                if crate::is_safe(&numbers_clone) {
                    is_safe = true;
                    break;
                }
            }
        }

        safe_reports += (is_safe == true) as u32;

        string.clear();
    }

    println!("{safe_reports}");

    Ok(())
}

fn main() -> io::Result<()> {
    part_two()
}