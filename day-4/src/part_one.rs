use std::ops::{Range, RangeInclusive};

use nom::{
    bytes::complete::{tag, take_while1, take_while_m_n},
    character::is_digit,
    combinator::map_res,
    sequence::tuple,
    IResult,
};

fn sections(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = nom::character::complete::u32(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;
    Ok((input, start..=end))
}

fn section_assignments(input: &str) -> IResult<&str, Vec<(Range<u32>, Range<u32>)>> {
    // let (input, _) = tag("#")(input)?;
    // let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;
    Ok((input, vec![]))
}

pub fn process_one(input: &str) -> String {
    let content = std::fs::read_to_string(input).expect("Could not find file");

    let number = take_while1(is_digit);
    let dash = take_while1(|c| c == '-');
    let comma = take_while1(|c| c == ',');
    // let range = preceded(number, dash, number);
    // fn request_interval_pair(i: &[u8]) -> <&[u8], Request> {
    //     let (interval, _, interval) = tuple((range, comma, range));
    //     Ok(())
    // }
    for line in content.lines() {}
    todo!()
    // Q : how do I know if one range is in another?
    // A : determine the smaller starting interval(A), then compare it's end time
    //     with the start of the next interval(B). If the start of B is less than
    //     or equal to the end of A, then the times are conflicting
}
