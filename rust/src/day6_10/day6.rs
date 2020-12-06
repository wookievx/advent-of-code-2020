extern crate nom;

use nom::multi::separated_list1;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map;
use nom::IResult;
use nom::branch::alt;
use self::nom::bytes::complete::take_while;

#[derive(Eq, PartialEq, Debug)]
pub struct GroupInput(Vec<Vec<char>>);

pub fn parse_input(input: &str) -> Result<Vec<GroupInput>, ()> {
    match internal_parse(input) {
        Ok((_, res)) => Ok(res),
        Err(_) => {
                eprintln!("Failed to parse input: {}", input);
                Err(())
            }
    }
}

fn internal_parse(input: &str) -> IResult<&str, Vec<GroupInput>> {
    separated_list1(
        tag("\n\n"),
        map(
            alt((
                take_until("\n\n"),
                take_while(|c: char| c.is_alphabetic()))
            ),
            parse_single_group
        )
    )(input)
}

fn parse_single_group(group_str: &str) -> GroupInput {
    GroupInput(
        group_str
            .lines()
            .map(|str| str.chars().collect())
            .collect()
    )
}

#[cfg(test)]
mod tests {
    use crate::day6_10::day6::{parse_input, GroupInput};

    #[test]
    fn check_parsing() {
        let input = "abc\n\
                          \n\
                          a\n\
                          b\n\
                          c\n\
                          \n\
                          ab\n\
                          ac\n\
                          \n\
                          a\n\
                          a\n\
                          a\n\
                          a\n\
                          \n\
                          b";
        let parse_res = parse_input(input);

        let expected = vec![
            GroupInput(vec![vec!['a', 'b', 'c']]),
            GroupInput(vec![vec!['a'], vec!['b'], vec!['c']]),
            GroupInput(vec![vec!['a', 'b'], vec!['a', 'c']]),
            GroupInput(vec![vec!['a'], vec!['a'], vec!['a'], vec!['a']]),
            GroupInput(vec![vec!['b']])
        ];

        assert_eq!(parse_res, Ok(expected));
    }
}