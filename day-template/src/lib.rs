use part_one::process_one;
use part_two::process_two;

mod part_one;
mod part_two;

#[test]
fn part_one_test() {
    let result = process_one("input_test.txt");
    assert_eq!("", result);
}

#[test]
fn part_one() {
    let result = process_one("input.txt");
    dbg!(result);
}

#[test]
fn part_two_test() {
    let result = process_two("input_test.txt");
    assert_eq!("", result);
}

#[test]
fn part_two() {
    let result = process_two("input.txt");
    dbg!(result);
}
