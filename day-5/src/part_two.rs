use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until, take_while1},
    character::{
        complete::{alpha1, alphanumeric1, newline, space0},
        is_alphabetic,
        streaming::space1,
    },
    combinator::{iterator, map, recognize},
    multi::{many0, many_till, separated_list1},
    number,
    number::complete::i8,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Instruction {
    MoveAmount: usize,
    From: usize,
    To: usize,
}

impl From<(&str, &str, &str)> for Instruction {
    fn from(instrucion: (&str, &str, &str)) -> Self {
        Instruction {
            MoveAmount: instrucion.0.parse::<usize>().unwrap() - 1,
            From: instrucion.1.parse::<usize>().unwrap() - 1,
            To: instrucion.2.parse::<usize>().unwrap() - 1,
        }
    }
}

fn crate_single(input: &str) -> IResult<&str, &str> {
    delimited(tag("["), alpha1, tag("]"))(input)
}

fn crate_empty(input: &str) -> IResult<&str, &str> {
    tag("   ")(input)
}

fn crate_line(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(" "), alt((crate_single, crate_empty)))(input)
}

fn crate_collection(input: &str) -> IResult<&str, Vec<Vec<&str>>> {
    let (rest, crate_levels) = separated_list1(newline, crate_line)(input)?;
    let mut crate_stack: Vec<Vec<&str>> = vec![Vec::new(); crate_levels.get(0).unwrap().len()];

    // println!("{}", crate_levels.len());
    dbg!(crate_levels.clone());
    // println!("{}", crate_stack.len());

    for level in crate_levels.iter() {
        for (i, c) in level.iter().enumerate() {
            if crate_has_contents(c) {
                crate_stack.get_mut(i).unwrap().push(c.clone())
            }
        }
    }

    for stack in crate_stack.iter_mut() {
        stack.reverse();
    }

    dbg!(crate_stack.clone());

    Ok((rest, crate_stack))
}

fn crate_numbers(input: &str) -> IResult<&str, (char, char)> {
    let (rest, _) = newline(input)?;
    let (rest, _) = take_until("\n")(rest)?;
    pair(newline, newline)(rest)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        newline,
        map(
            tuple((
                preceded(tag("move "), terminated(alphanumeric1, tag(" "))),
                preceded(tag("from "), terminated(alphanumeric1, tag(" "))),
                preceded(tag("to "), alphanumeric1),
            )),
            |s: (&str, &str, &str)| Instruction::from(s),
        ),
    )(input)
}

fn organize_crates(crates: &mut Vec<Vec<&str>>, instructions: Vec<Instruction>) {
    for inst in instructions {
        let mut moving: Vec<&str> = vec!();

        for _ in 0..=(inst.MoveAmount) {
            if let Some(top) = crates.get_mut(inst.From).unwrap().pop() {
                moving.push(top);
            }
        }

        for _ in 0..moving.len() {
            crates.get_mut(inst.To).unwrap().push(moving.pop().unwrap());
        }
    }

    dbg!(crates.clone());
}

pub fn process_two(input: &str) -> String {
    let content = std::fs::read_to_string(input).expect("Could not find file");
    let (rest, mut crates) = crate_collection(&content).unwrap();
    let (rest, _) = crate_numbers(rest).unwrap();
    let (_, instructions) = instructions(rest).unwrap();
    organize_crates(&mut crates, instructions);

    let result: String = crates
        .iter_mut()
        .map(|stack: &mut Vec<&str>| stack.pop().unwrap())
        .collect::<String>();

    result
}

fn crate_has_contents(c: &str) -> bool {
    c.len() == 1
}
