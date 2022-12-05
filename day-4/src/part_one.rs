pub fn process_one(input: &str) -> String {
    let content = std::fs::read_to_string(input).expect("Could not find file");
    let mut stack: Vec<Interval> = Vec::new();
    let mut total: usize = 0;
    for line in content.lines() {
        let tokens = tokenize(line);
        let interval_pair: (Interval, Interval) = parse(tokens, &mut stack);
        let result = determine_overlap(interval_pair, &mut stack);
        total += result;
    }
    total.to_string()
    // Q : how do I know if one range is in another?
    // A : determine the smaller starting interval(A), then compare it's end time
    //     with the start of the next interval(B). If the start of B is less than
    //     or equal to the end of A, then the times are conflicting
    
}

fn tokenize(line: &str) -> Vec<IntervalToken> {
    let tokens = line
        .chars()
        .map(|c| -> IntervalToken {
            match c {
                '0'..='9' => IntervalToken::TOKEN_NUM(c.to_string().parse::<usize>().unwrap()),
                '-' => IntervalToken::TOKEN_DASH,
                ',' => IntervalToken::TOKEN_COMMA,
                _ => panic!("unrecognized character"),
            } 
        })
        .collect::<Vec<IntervalToken>>();
    tokens
}

fn parse(tokens: Vec<IntervalToken>, stack: &mut Vec<Interval>) -> (Interval, Interval) {
    let mut peekable = tokens.iter().peekable();
    'l: loop {
        match peekable.next() {
            Some(c) => {
                match c {
                    IntervalToken::TOKEN_NUM(start_digit) => {
                        let mut number: usize = *start_digit;
                        while let Some(IntervalToken::TOKEN_NUM(next_digit)) = peekable.peek(){
                            number = (number * 10) + next_digit;
                        }
                        
                    },
                    _ => continue,
                }
            },
            None => break 'l,
        }
    }
    todo!()
}

fn determine_overlap(interval: (Interval, Interval)) -> usize {0}

struct Interval(usize, usize);

#[derive(PartialEq, Eq, Copy, Clone)]
enum IntervalToken {
    TOKEN_NUM(usize),
    TOKEN_DASH,
    TOKEN_COMMA
}

fn matching(found: &IntervalToken, compare_too: IntervalToken) -> bool {
    *found == compare_too
}