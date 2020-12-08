extern crate nom;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{take_while, tag};
use nom::sequence::tuple;
use nom::combinator::map_res;
use nom::sequence::preceded;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
use std::fmt;
use std::borrow::Borrow;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Command {
    Nop { offset: i32 },
    Jmp { offset: i32 },
    Acc { change: i32 }
}

pub fn parse_input(arg: Vec<String>) -> Result<Vec<Command>, ()> {
    let res: Result<Vec<Command>, nom::Err<nom::error::Error<&str>>> = arg
        .iter()
        .map(|s| parse_command(s).map(|(_, res)| res))
        .collect();

    res.map_err(|e| {
        eprintln!("Failed to parse input: {:?}", e);
        ()
    })
}

pub fn solve_simple<'a>(arg: &'a Vec<Command>) -> (&'a [Command], i32) {
    let mut visited: Vec<bool> = vec![false; arg.len()];
    let mut position: usize = 0;
    let mut program_state: i32 = 0;

    loop {
        visited[position] = true;
        let (new_position, new_state) = next_position_and_state(arg, position, program_state);
        position = new_position;
        program_state = new_state;
        if visited[position] { break; }
    }
    let min_pos = max(0, position - 1);
    let max_pos = min(arg.len(), position + 1);
    (&arg[min_pos..max_pos], program_state)
}

fn next_position_and_state(arg: &Vec<Command>, position: usize, state: i32) -> (usize, i32) {
    match arg[position] {
        Command::Acc { change } => (position + 1, state + change),
        Command::Nop { offset: _ } => (position + 1, state),
        Command::Jmp { offset } => {
            let new_offset = if offset < 0 {
                position - (-offset) as usize
            } else {
               position + offset as usize
            };
            (new_offset, state)
        }
    }
}

impl Command {
    fn nop(offset: i32) -> Command {
        Command::Nop { offset }
    }

    fn jmp(offset: i32) -> Command {
        Command::Jmp { offset }
    }

    fn acc(change: i32) -> Command {
        Command::Acc { change }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn sign_of(arg: i32) -> &'static str {
            if arg >= 0 {
                "+"
            } else {
                ""
            }
        }
        match self {
            Command::Nop { offset } =>
                write!(f, "nop: {}{}", sign_of(*offset), offset),
            Command::Jmp { offset} =>
                write!(f, "jmp: {}{}", sign_of(*offset), offset),
            Command::Acc { change } =>
                write!(f, "acc: {}{}", sign_of(*change), change)
        }
    }
}

struct RenderedCommands<T>(T);

impl <I, T> Display for RenderedCommands<I> where
    T: Borrow<Command>,
    I: IntoIterator<Item = T> + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let res: Result<Vec<_>, _> = self.0.into_iter()
            .map(|c| {
                write!(f, "{}\n", c.borrow())
            })
            .collect();
        res.map(|_| ())
    }
}


fn parse_command(input: &str) -> IResult<&str, Command> {

    fn parse_number(input: &str) -> IResult<&str, i32> {
        fn parse_raw(input: &str) -> IResult<&str, (&str, &str)> {
            tuple((
                alt((tag("+"), tag("-"))),
                take_while(|c: char| c.is_digit(10))
            ))(input)
        }
        map_res(
            parse_raw,
            |(sign, to_num)| {
                if sign == "+" {
                    to_num.parse::<i32>()
                } else if sign == "-" {
                    to_num.parse::<i32>().map(|v| -v)
                } else {
                    "bad_number".parse::<i32>()
                }
            }
        )(input)
    }

    let (input, token) = alt((
        tag("nop"),
        tag("jmp"),
        tag("acc")
    ))(input)?;

    let (input, number) = preceded(tag(" "),parse_number)(input)?;

    match token {
        "nop" => Ok((input, Command::nop(number))),
        "jmp" => Ok((input, Command::jmp(number))),
        "acc" => Ok((input, Command::acc(number))),
        s => panic!("should not be reachable: {}", s)
    }

}

#[cfg(test)]
mod tests {
    use crate::day6_10::day8::{parse_input, Command, solve_simple, RenderedCommands};

    const INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn it_should_parse_input() {
        assert_eq!(
            parse_input(INPUT.lines().map(|s| s.to_string()).collect()),
            Ok(
                vec![
                    Command::nop(0),
                    Command::acc(1),
                    Command::jmp(4),
                    Command::acc(3),
                    Command::jmp(-3),
                    Command::acc(-99),
                    Command::acc(1),
                    Command::jmp(-4),
                    Command::acc(6)
                ]
            )
        )
    }

    #[test]
    fn it_should_solve_simple() {
        let input = parse_input(INPUT.lines().map(|s| s.to_string()).collect()).expect("Failed to parse input");
        let (commands, result) = solve_simple(&input);
        println!("Got results:\n{}Got state: {}", RenderedCommands(commands), result);
        assert_eq!(
            commands,
            &input[0..2]
        );
        assert_eq!(
            result,
            5
        )
    }

}

