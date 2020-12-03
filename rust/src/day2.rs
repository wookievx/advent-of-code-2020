extern crate nom;

use nom::{
    IResult,
    bytes::complete::tag,
    combinator::map_res,
    sequence::tuple};
use nom::sequence::separated_pair;
use nom::bytes::complete::take_while;
use self::nom::sequence::preceded;
use self::nom::bytes::complete::take;
use self::nom::AsChar;

pub struct Password {
    min_req: u16,
    max_req: u16,
    character: char,
    content: String
}

pub fn solve(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| {
            let count = p.content.as_str().matches(|c: char| c == p.character).count();
            let res = usize::from(p.min_req) <= count && count <= usize::from(p.max_req);
            res
        })
        .count()
}

pub fn solve_adv(input: &Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| {
            let chars: Vec<char> = p.content.chars().collect();
            let res = (chars[usize::from(p.min_req) - 1] == p.character) ^ (chars[usize::from(p.max_req) - 1] == p.character);
            res
        })
        .count()
}

pub fn parse_input(input: Vec<String>) -> Vec<Password> {
    input
        .iter()
        .map(|contents| parse_line(contents.as_str()).unwrap().1)
        .collect()
}

fn parse_line(input: &str) -> IResult<&str, Password> {
    let (input, (min_req, max_req)) = separated_pair(parse_number, tag("-"), parse_number)(input)?;
    let (_, (character, content)) = preceded(take_while(|c: char| c.is_whitespace()), separated_pair(take(1usize), tag(": "), take_rest))(input)?;
    IResult::Ok(("", Password { min_req, max_req, character: character.chars().next().unwrap(), content: content.to_string() }))
}

fn parse_number(input: &str) -> IResult<&str, u16> {
    map_res(take_while(|c: char| c.is_dec_digit()), |s: &str| s.parse::<u16>())(input)
}

fn take_rest(input: &str) -> IResult<&str, &str> {
    IResult::Ok(("", input))
}