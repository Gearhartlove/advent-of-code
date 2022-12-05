pub fn process_two() -> String {
    let content = std::fs::read_to_string("input.txt").expect("Could not find file");
    let mut total_points: usize = 0;
    for line in content.lines() {
        let round_points = line
            .split(' ')
            .map(data)
            .collect::<Round>()
            .calculate_points();
        total_points += round_points;
    }
    total_points.to_string()
}

fn data(input: &str) -> usize {
    match input {
        "A" | "B" | "C" => RPS::from(input) as usize,
        "X" | "Y" | "Z" => RoundResult::from(input) as usize,
        _ => panic!("Unrecognized user input."),
    }
}

struct Round {
    Them: RPS,
    RoundResult: RoundResult,
}

impl FromIterator<usize> for Round {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Round {
            Them: RPS::from(iter.next().unwrap()),
            RoundResult: RoundResult::from(iter.next().unwrap()),
        }
    }
}

impl Round {
    pub fn calculate_points(&self) -> usize {
        self.find_my_shape() + self.round_points()
    }

    pub fn round_points(&self) -> usize {
        self.RoundResult as usize
    }

    pub fn find_my_shape(&self) -> usize {
        let me = match (self.Them, self.RoundResult) {
            (RPS::Rock, RoundResult::Win) => RPS::Paper,
            (RPS::Scissors, RoundResult::Win) => RPS::Rock,
            (RPS::Paper, RoundResult::Win) => RPS::Scissors,
            (RPS::Rock, RoundResult::Draw) => RPS::Rock,
            (RPS::Scissors, RoundResult::Draw) => RPS::Scissors,
            (RPS::Paper, RoundResult::Draw) => RPS::Paper,
            (RPS::Rock, RoundResult::Loss) => RPS::Scissors,
            (RPS::Scissors, RoundResult::Loss) => RPS::Paper,
            (RPS::Paper, RoundResult::Loss) => RPS::Rock,
        };
        return me as usize;
    }
}

#[derive(Clone, Copy)]
enum RoundResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl From<&str> for RoundResult {
    fn from(round_result: &str) -> Self {
        match round_result {
            "Z" => RoundResult::Win,
            "Y" => RoundResult::Draw,
            "X" => RoundResult::Loss,
            _ => panic!("Unrecognized user input."),
        }
    }
}

impl From<usize> for RoundResult {
    fn from(item: usize) -> Self {
        match item {
            6 => RoundResult::Win,
            3 => RoundResult::Draw,
            0 => RoundResult::Loss,
            _ => panic!("Unrecognized user input."),
        }
    }
}

#[derive(Clone, Copy)]
enum RPS {
    Rock = 1,     // A
    Paper = 2,    // B
    Scissors = 3, // C
}

impl From<&str> for RPS {
    fn from(item: &str) -> Self {
        match item {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!("Unrecognized user input."),
        }
    }
}

impl From<usize> for RPS {
    fn from(item: usize) -> Self {
        match item {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissors,
            _ => panic!("User input not recognized."),
        }
    }
}
