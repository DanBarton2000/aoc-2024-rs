use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn map_and_updates() -> std::io::Result<(HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>)> {
    let file = std::fs::File::open(".\\files\\day_five.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in reader.lines() {
        if let Ok(l) = line {
            if l.contains("|") {
                let split: Vec<&str> = l.split("|").collect();
                match map.entry(split[0].parse().unwrap()) {
                    Entry::Vacant(e) => {
                        let mut update = HashSet::new();
                        update.insert(split[1].parse::<u32>().unwrap());
                        e.insert(update);
                    },
                    Entry::Occupied(mut e) => { e.get_mut().insert(split[1].parse::<u32>().unwrap()); }
                }
            } else if l.contains(",") {
                let split: Vec<&str> = l.split(",").collect();
                let mut vec: Vec<u32> = Vec::new();

                for num in split {
                    vec.push(num.parse().unwrap());
                }

                updates.push(vec);
            }
        }
    }

    Ok((map, updates))
}

fn part_one() -> std::io::Result<()> {
    let (map, updates) = map_and_updates()?;
    let mut count: u32 = 0;

    for update in updates {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut valid = true;

        for num in &update {
            if let Some(set) = map.get(&num)
            {
                if seen.intersection(set).count() > 0 {
                    valid = false;
                    break;
                }
            }

            seen.insert(*num);
        }

        if valid {
            count += update[update.len() / 2];
        }
    }

    println!("{count}");

    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one()
}