#![feature(iter_next_chunk)]
use part_one::process_one;
use part_two::process_two;

mod part_one;
mod part_two;

#[test]
fn part_one() {
    let result = process_one();
    dbg!(result);
}

#[test]
fn part_two() {
    let result = process_two();
    dbg!(result);
}