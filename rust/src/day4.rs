extern crate nom;

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple};
use nom::sequence::separated_pair;
use nom::bytes::complete::take;
use nom::bytes::complete::{is_not, is_a};
use nom::multi::many0;
use nom::combinator::map;
use nom::branch::alt;
use nom::multi::{separated_list1, many1};
use std::collections::HashMap;
use std::fmt;
use nom::lib::std::fmt::Formatter;
use nom::multi::many_m_n;
use nom::bytes::complete::{take_while_m_n, take_while};
use nom::sequence::preceded;
use nom::combinator::map_res;
use nom::error::ErrorKind;
use self::nom::character::is_digit;
use self::nom::number::complete::u8;

#[derive(Copy, Clone, Default)]
pub struct Passport<'a> {
    birth_year: Option<&'a str>,
    issue_year: Option<&'a str>,
    expiration_year: Option<&'a str>,
    height: Option<&'a str>,
    hair_color: Option<&'a str>,
    eye_color: Option<&'a str>,
    passport_id: Option<&'a str>,
    country_id: Option<&'a str>
}

struct NewOpt<T>(Option<T>);

impl <'a> fmt::Display for Passport<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Passport\n\tbirth_year: {}\n\tissue_year: {}\n\texpiration_year: {}\n\theight: {}\n\thair_color: {}\n\teye_color: {}\n\tpassport_id: {}\n\tcountry_id: {}",
            NewOpt(self.birth_year),
            NewOpt(self.issue_year),
            NewOpt(self.expiration_year),
            NewOpt(self.height),
            NewOpt(self.hair_color),
            NewOpt(self.eye_color),
            NewOpt(self.passport_id),
            NewOpt(self.country_id)
        )
    }
}

impl <T> fmt::Display for NewOpt<T>
where
    T: fmt::Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NewOpt(Some(v)) => v.fmt(f),
            NewOpt(None) => write!(f, "null")
        }
    }
}

pub fn solve_simple(input: &Vec<Passport>) -> usize {
    input
        .iter()
        .filter(|&p| -> bool {
            p.birth_year.is_some() && p.passport_id.is_some() && p.eye_color.is_some() && p.hair_color.is_some() && p.height.is_some() && p.expiration_year.is_some() && p.issue_year.is_some()
        })
        .count()
}

pub fn solve_advanced(input: &Vec<Passport>) -> usize {
    input
        .iter()
        .filter(|&p| -> bool {
            p.birth_year.filter(|&y| validate_year(1920, 2002, y)).is_some() &&
                p.issue_year.filter(|&y| validate_year(2010, 2020, y)).is_some() &&
                p.expiration_year.filter(|&y| validate_year(2020, 2030, y)).is_some() &&
                p.height.filter(|&h| validate_height(150, 193, 59, 76, h)).is_some() &&
                p.hair_color.filter(|&c| validate_color(c)).is_some() &&
                p.eye_color.filter(|&c| validate_eye_color(c)).is_some() &&
                p.passport_id.filter(|&p| validate_pid(p)).is_some()
        })
        .count()
}

fn validate_year(min_value: u16, max_value: u16, arg: &str) -> bool {
    let value = arg.parse::<u16>();
    match value {
        Ok(value) => min_value <= value && value <= max_value,
        Err(pr) => false
    }
}


fn validate_height(min_cm: u16, max_cm: u16, min_in: u16, max_in: u16, height: &str) -> bool {
    let res = parse_height(height);
    match res {
        Ok((_, Height::Cm(v))) => min_cm <= v && v <= max_cm,
        Ok((_, Height::In(v))) => min_in <= v && v <= max_in,
        Err(_) => false
    }
}

enum Height {
    Cm(u16),
    In(u16)
}

fn parse_height(input: &str) -> IResult<&str, Height> {
    fn is_digit(c: char) -> bool {
        c.is_digit(10)
    }
    let (input, (v, unit)) = tuple((map_res(take_while(is_digit), |s: &str| s.parse::<u16>()), alt((tag("cm"), tag("in")))))(input)?;
    if unit == "cm" {
        Ok((input, Height::Cm(v)))
    } else if unit == "in" {
        Ok((input, Height::In(v)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(input, ErrorKind::AlphaNumeric)))
    }
}

fn validate_eye_color(input: &str) -> bool {
    input == "amb" || input == "blu" || input == "brn" || input == "brn" || input == "brn" || input == "hzl" || input == "oth"
}

fn validate_color(input: &str) -> bool {
    hex_color(input).is_ok()
}

pub struct Color {
    pub red:   u8,
    pub green: u8,
    pub blue:  u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, is_hex_digit),
        from_hex
    )(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn validate_pid(input: &str) -> bool {
    parse_pid(input).is_ok()
}

fn parse_pid(input: &str) -> IResult<&str, Vec<u8>> {
    many_m_n(9, 9, map_res(take(1usize), |s: &str| s.parse::<u8>()))(input)
}

//input parsing begins
pub fn parse_input(input: &str) -> Vec<Passport> {
    let (_, res) = separated_list1(many1(tag("\n")), parse_passport)(input).expect("Illegal input");
    res
}

fn parse_passport(input: &str) -> IResult<&str, Passport> {
    let separator = alt((is_a("\n "), take(0usize)));
    preceded(separator, map(separated_list1(alt((tag("\n"), tag(" "))), parse_passport_entry), from_pairs))(input)
}

fn parse_passport_entry(input: &str) -> IResult<&str, (&str, &str)> {
    let tags = alt((
        tag("byr"),
        tag("iyr"),
        tag("eyr"),
        tag("hgt"),
        tag("hcl"),
        tag("ecl"),
        tag("pid"),
        tag("cid")
    ));
    let (input, (l, r)) = separated_pair(tags, tag(":"), is_not(" \t\r\n"))(input)?;
    Ok((input, (l, r)))
}

fn from_pairs<'a>(input: Vec<(&'a str, &'a str)>) -> Passport<'a> {
    let input_map: HashMap<&'a str, &'a str> = input.iter().cloned().collect();
    let get_tag = move |tag: &str| -> Option<&'a str> {
        match input_map.get(tag) {
            Some(v) => Some(*v),
            None => None
        }
    };
    Passport {
        birth_year: get_tag("byr"),
        issue_year: get_tag("iyr"),
        expiration_year: get_tag("eyr"),
        height: get_tag("hgt"),
        hair_color: get_tag("hcl"),
        eye_color: get_tag("ecl"),
        passport_id: get_tag("pid"),
        country_id: get_tag("cid")
    }
}