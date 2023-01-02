use std::collections::HashSet;

pub fn process_two(input: &str) -> usize {
    let chars: &[u8] = &input.as_bytes();
    let mut frequency: HashSet<char> = HashSet::new();
    let mut considering: Vec<char> = Vec::default();
    const COUNT: usize = 4;

    for i in 0..COUNT {
        considering.push(chars[i] as char);
    }

    for i in 0..input.len() {
        // prevent indexing out of bounds
        if i + COUNT > input.len() {
            panic!("No unique encoding of size {COUNT} found in this input.")
        }

        // is unique?
        for c in considering.iter() {
            let _ = frequency.insert(c.clone());
            if frequency.len() == COUNT {
                return i + COUNT;
            }
        }
        // update consider
        let _ = considering.remove(0);
        considering.push(input.chars().nth(i + COUNT).unwrap());
        assert_eq!(considering.len(), COUNT);
        // clear frequency
        frequency.clear();
    }

    panic!("Will never reach here.");
}
