pub fn process_one() -> String {
    let content = std::fs::read_to_string("input.txt").expect("Could not find file");
    let mut total_points: usize = 0;
    for line in content.lines() {
        let round_points = line
            .split(' ')
            .map(RPS::from)
            .collect::<Round>()
            .calculate_points();
        total_points += round_points;
    }
    total_points.to_string()
}

struct Round {
    Them: RPS,
    Me: RPS,
}

impl FromIterator<RPS> for Round {
    fn from_iter<T: IntoIterator<Item = RPS>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Round {
            Them: iter.next().unwrap(),
            Me: iter.next().unwrap(),
        }
    }
}

impl Round {
    const WIN: usize = 6;
    const DRAW: usize = 3;
    const LOSS: usize = 0;

    pub fn calculate_points(&self) -> usize {
        self.Me.shape_points() + self.round_points()
    }

    pub fn round_points(&self) -> usize {
        match (&self.Them, &self.Me) {
            (RPS::Rock, RPS::Paper) => Round::WIN,
            (RPS::Paper, RPS::Scissors) => Round::WIN,
            (RPS::Scissors, RPS::Rock) => Round::WIN,
            (RPS::Rock, RPS::Rock) => Round::DRAW,
            (RPS::Scissors, RPS::Scissors) => Round::DRAW,
            (RPS::Paper, RPS::Paper) => Round::DRAW,
            (RPS::Rock, RPS::Scissors) => Round::LOSS,
            (RPS::Paper, RPS::Rock) => Round::LOSS,
            (RPS::Scissors, RPS::Paper) => Round::LOSS,
        }
    }
}

enum RPS {
    Rock,     // A, X
    Paper,    // B, Y
    Scissors, // C, Z
}

impl From<&str> for RPS {
    fn from(item: &str) -> Self {
        match item {
            "A" | "X" => RPS::Rock,
            "B" | "Y" => RPS::Paper,
            "C" | "Z" => RPS::Scissors,
            _ => panic!("Unrecognized user input."),
        }
    }
}

impl RPS {
    pub fn shape_points(&self) -> usize {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}
