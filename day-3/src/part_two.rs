use std::collections::{HashMap, HashSet};

pub fn process_two() -> String {
    let content = std::fs::read_to_string("D:/krist/Projects/rust/advent-of-code/day-3/input.txt")
        .expect("Could not find file");

    let letter_scores = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();

    let mut total: usize = 0;
    let mut lines = content.lines();
    while let Ok(elf_group) = lines.next_chunk::<3>() {
        let mut map: HashMap<char, usize> = HashMap::new();
        let badge: char = 'badge: {
            for rucksack in elf_group {
                let mut set: HashSet<char> = HashSet::new();
                for item in rucksack.chars() {
                    set.insert(item);
                }
                for item in set.iter() {
                    // Note: inserts the item with zero if not in the HashMap if it's not already present
                    let count = map.entry(*item).or_insert(0);
                    *count += 1;
                    if *count == 3 {
                        break 'badge *item;
                    }
                }
            }
            panic!("no badge found");
        };
        total += letter_scores.get(&badge).unwrap();
    }
    total.to_string()
}

fn value(item: char) -> u64 {
    match item {
        'a'..='z' => item as u64 - 96,
        'A'..='Z' => item as u64 - 38,
        _ => panic!("unrecognized input"),
    }
}
