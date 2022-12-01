pub fn process_one() -> String {
    let mut max_cals: u64 = 0;
    let mut temp_cals: u64 = 0;
    let content = std::fs::read_to_string("input.txt").expect("Could not find file.");

    for elf in content.split("\r\n\r\n") {
        for calorie in elf.split("\r\n") {
            temp_cals += calorie.parse::<u64>().unwrap();
        }
        if temp_cals > max_cals {
            max_cals = temp_cals
        }
        temp_cals = 0
    }
    max_cals.to_string()
}