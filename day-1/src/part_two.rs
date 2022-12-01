pub fn process_two() -> String {
    let mut elves_total_calories: Vec<u64> = vec!();
    let content = std::fs::read_to_string("input.txt").expect("Could not find file.");

    for elf in content.split("\r\n\r\n") {
        let mut total_cals: u64 = 0;
        for calorie in elf.split("\r\n") {
            total_cals += calorie.parse::<u64>().unwrap();
        }
        elves_total_calories.push(total_cals);
    }
    elves_total_calories.sort();
    elves_total_calories.reverse();
    let total = elves_total_calories.iter().take(3).sum::<u64>();
    total.to_string()
}