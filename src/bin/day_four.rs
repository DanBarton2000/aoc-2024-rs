use std::{fs, io};
use std::io::BufRead;

enum Search {
    Up,
    Down,
    Right,
    Left,
    RightUp,
    RightDown,
    LeftUp,
    LeftDown
}

fn search_recursive(row: usize, column: usize, content: &Vec<String>, search_word: &str, search: Search) -> u32 {
    if search_word.len() == 0 { return 1; }
    if search_word.len() == 1 && content[row].as_bytes()[column] == search_word.as_bytes()[0] { return 1; }
    if content[row].as_bytes()[column] != search_word.as_bytes()[0] { return 0; }

    let range = 1..search_word.len();

    match search {
        Search::Up => {
            if (row as i32) - 1 >= 0 {
                return search_recursive(row - 1, column, &content, &search_word[range], search);
            }
        }
        Search::Down => {
            if row + 1 < content.len() {
                return search_recursive(row + 1, column, &content, &search_word[range], search);
            }
        }
        Search::Right => {
            if column + 1 < content[0].len() {
                return search_recursive(row, column + 1, &content, &search_word[range], search);
            }
        }
        Search::Left => {
            if (column as i32) - 1 >= 0 {
                return search_recursive(row, column - 1, &content, &search_word[range], search);
            }
        }
        Search::LeftDown => {
            if (column as i32) - 1 >= 0 && row + 1 < content.len() {
                return search_recursive(row + 1, column - 1, &content, &search_word[range], search);
            }
        }
        Search::LeftUp => {
            if (column as i32) - 1 >= 0 && (row as i32) - 1 >= 0 {
                return search_recursive(row - 1, column - 1, &content, &search_word[range], search);
            }
        }
        Search::RightDown => {
            if column + 1 < content[0].len() && row + 1 < content.len() {
                return search_recursive(row + 1, column + 1, &content, &search_word[range], search);
            }
        }
        Search::RightUp => {
            if column + 1 < content[0].len() && (row as i32) - 1 >= 0 {
                return search_recursive(row - 1, column + 1, &content, &search_word[range], search);
            }
        }
    }

    0
}

fn search(row: usize, column: usize, content: &Vec<String>, search_word: &str) -> u32 {
    search_recursive(row, column, &content, &search_word, Search::Up) +
    search_recursive(row, column, &content, &search_word, Search::Down) +
    search_recursive(row, column, &content, &search_word, Search::Left) +
    search_recursive(row, column, &content, &search_word, Search::Right) +
    search_recursive(row, column, &content, &search_word, Search::LeftUp) +
    search_recursive(row, column, &content, &search_word, Search::LeftDown) +
    search_recursive(row, column, &content, &search_word, Search::RightUp) +
    search_recursive(row, column, &content, &search_word, Search::RightDown)
}

fn part_one() -> io::Result<()> {
    let file = fs::File::open(".\\files\\day_four.txt")?;
    let reader = io::BufReader::new(file);
    let mut content = vec![];

    for line in reader.lines() {
        content.push(line?);
    }

    let mut counts = 0;

    for r in 0..content.len() {
        for c in 0..content[0].len() {
            counts += search(r, c, &content, "XMAS");
        }
    }

    println!("{counts}");

    Ok(())

}

fn main() -> io::Result<()> {
    part_one()
}