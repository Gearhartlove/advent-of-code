use std::collections::HashSet;

pub fn process_one() -> String {
    let content = std::fs::read_to_string("input.txt").expect("Could not find file");
    let mut total: u64 = 0;
    for line in content.lines() {
        let mut start_hash: HashSet<char> = HashSet::new();
        let mut end_hash: HashSet<char> = HashSet::new();
        let mid = line.len() / 2;
        let start = &line[..mid];
        let end = &line[mid..];
        for i in 0..mid {
            let start_item = start.chars().nth(i).unwrap();
            let end_item = end.chars().nth(i).unwrap();
            let _ = start_hash.insert(start_item);
            let _ = end_hash.insert(end_item);
        }
        for start_item in start_hash.iter() {
            for end_item in end_hash.iter() {
                if start_item == end_item {
                    total += value(*start_item)
                }
            }
        }
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
