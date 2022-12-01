use part_one::process_one;
use part_two::process_two;

mod part_one;
mod part_two;


#[test] 
fn part_one() {
    let result = process_one();
    assert_eq!("69693", result)
}


#[test] 
fn part_two() {
    let result = process_two();
    assert_eq!("200945", result)
}