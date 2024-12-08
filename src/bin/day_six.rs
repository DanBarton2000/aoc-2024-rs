use std::collections::{HashMap, HashSet};
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn dir_map() -> HashMap<Dir, Dir> {
    let mut map = HashMap::new();
    map.insert(Dir::Up, Dir::Right);
    map.insert(Dir::Right, Dir::Down);
    map.insert(Dir::Down, Dir::Left);
    map.insert(Dir::Left, Dir::Up);
    map
}

fn dir_vector(dir: &Dir) -> (i32, i32) {
    match dir {
        Dir::Up => { (-1, 0) }
        Dir::Down => { (1, 0) }
        Dir::Left => { (0, -1) }
        Dir::Right => { (0, 1) }
    }
}

fn starting_position(content: &[String]) -> Option<(i32, i32)> {
    for (row, string) in content.iter().enumerate() {
        for (col, char) in string.chars().enumerate() {
            if char == '^' {
                return Some((row as i32, col as i32));
            }
        }
    }

    None
}

fn part_one() -> std::io::Result<()> {
    let file = std::fs::File::open(".\\files\\day_six.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut content = vec![];

    for line in reader.lines() {
        content.push(line?);
    }

    let Some(start_pos) = starting_position(&content) else { panic!("No start position found") };
    let mut direction = Dir::Up;
    let dir_map = dir_map();
    let mut set = HashSet::new();
    set.insert(start_pos);

    let mut current_pos: (i32, i32) = start_pos;

    loop {
        let Some(character) = content[current_pos.0 as usize].chars().nth(current_pos.1 as usize)
        else { panic!("Failed to get character at: {} {} {} {}", current_pos.0, current_pos.1, content.len(), content[0].chars().count()); };

        if character == '#' {
           if let Some(new_direction) = dir_map.get(&direction) {
               let dir_vector = dir_vector(&direction);
               current_pos.0 -= dir_vector.0;
               current_pos.1 -= dir_vector.1;
               direction = *new_direction;
           } else {
               panic!("Failed to get new direction for {:?}", direction);
           }
        } else {
            set.insert(current_pos);
        }

        let dir_vector = dir_vector(&direction);
        current_pos.0 += dir_vector.0;
        current_pos.1 += dir_vector.1;

        if current_pos.0 < 0 || current_pos.0 >= content.len() as i32 || current_pos.1 < 0 || current_pos.1 >= content[0].chars().count() as i32 {
            break;
        }
    }

    println!("{}", set.len());

    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()
}