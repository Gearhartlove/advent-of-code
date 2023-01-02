use core::panic;
use std::collections::HashSet;

pub fn process_one(input: &str) -> usize {
    let chars: &[u8] = &input.as_bytes();
    let mut frequency: HashSet<char> = HashSet::new();
    for i in 0..chars.len() {
        let a = chars[i] as char;
        let b = chars[i + 1] as char;
        let c = chars[i + 2] as char;
        let d = chars[i + 3] as char;

        if frequency.insert(a) && frequency.insert(b) && frequency.insert(c) && frequency.insert(d) {
            println!("found {}{}{}{}", a, b, c, d);
            return i + 4
        }

        frequency.clear();
    }
    panic!("No answer found.");
}
