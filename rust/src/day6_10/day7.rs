extern crate nom;

use std::collections::{HashMap, HashSet};
use std::collections::linked_list::LinkedList;
use nom::sequence::{separated_pair, terminated};
use nom::bytes::complete::{take_while, tag, take};
use nom::IResult;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::branch::alt;
use nom::combinator::consumed;
use nom::combinator::map;
use nom::multi::separated_list0;
use std::collections::hash_map::Entry;
use std::convert::TryFrom;

pub struct BagSpecs<'a> {
    graph: HashMap<&'a str, Vec<(&'a str, u32)>>,
    inverted: HashMap<&'a str, Vec<&'a str>>
}

pub fn solve_simple(arg: &BagSpecs, name: &'static str) -> u32 {
    let mut stack: LinkedList<&str> = LinkedList::new();
    let mut visited: HashMap<& str, bool> =
        arg.graph.keys().cloned().map(|k| (k, false)).collect();
    let mut result: HashSet<&str> = HashSet::new();
    stack.push_back(name);
    while let Some(focus) = stack.pop_back() {
        visited.insert(name, true);
        for bag_name in &arg.inverted[focus] {
            if !visited[bag_name] {
                stack.push_back(bag_name);
                result.insert(bag_name);
            }
        }
    }
    u32::try_from(result.len()).expect("Failed")
}

//danger, might blow, assuming correct input
pub fn solve_advanced(arg: &BagSpecs, name: &'static str) -> u32 {
    let mut stack: LinkedList<(&str, u32)> = LinkedList::new();
    let mut res: u32 = 0;
    stack.push_back((name, 1));
    while let Some((focus, multiplier)) = stack.pop_back() {
        for (bag_name, w) in &arg.graph[focus] {
            let new_multiplier = multiplier * w;
            stack.push_back((bag_name, new_multiplier));
            res += new_multiplier;
        }
    }
    res
}

pub fn parse_all_bags_specs(input: &str) -> Result<BagSpecs, ()> {
    match _parse_all_bags_specs(input) {
        Ok((_, res)) => Ok(res),
        Err(_) => {
            eprintln!("Failed to parse input");
            Err(())
        }

    }
}

fn _parse_all_bags_specs(input: &str) -> IResult<&str, BagSpecs> {
    map(
        parse_raw_bag_specs,
        |ipt| BagSpecs::build(ipt)
    )(input)
}

impl BagSpecs<'_> {
    fn build(input: Vec<RawBagSpec>) -> BagSpecs {
        let mut graph: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
        let mut inverted: HashMap<&str, Vec<&str>> = HashMap::new();
        for RawBagSpec(n, vec) in &input {
            graph.insert(n, vec.clone());
        }
        for RawBagSpec(to, vec) in &input {
            for (from, _) in vec {
                match inverted.entry(from) {
                    Entry::Vacant(e) => {
                        e.insert(vec![to]);
                    },
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(to);
                    }
                }
            }
            match inverted.entry(to) {
                Entry::Vacant(e) => {
                    e.insert(vec![]);
                },
                Entry::Occupied(_) => {}
            }
        }


        BagSpecs {
            graph,
            inverted
        }
    }

    fn get_edges(&self, bag: &str) -> Option<&Vec<(&str, u32)>> {
        match self.graph.get(bag) {
            Some(v) => Some(v),
            None => None
        }
    }
}


struct RawBagSpec<'a>(&'a str, Vec<(&'a str, u32)>);

fn parse_raw_bag_specs(input: &str) -> IResult<&str, Vec<RawBagSpec>> {
    separated_list0(tag("\n"), parse_bag_spec)(input)
}

fn parse_bag_spec(input: &str) -> IResult<&str, RawBagSpec> {
    let (input, (name, s)) = separated_pair(
        parse_bag,
        tag(" contain "),
        terminated(
            alt((
                separated_list1(
                    tag(", "),
                    separated_pair(
                        map_res(take(1 as usize), |s: &str| s.parse::<u32>()),
                        tag(" "),
                        parse_bag
                    )
                ),
                map(tag("no other bags"), |_| Vec::new())
            )),
            tag(".")
        )
    )(input)?;

    let parsed_spec: Vec<(&str, u32)> = s.iter().map(|&(n, s)| (s, n)).collect();
    Ok((input, RawBagSpec(name, parsed_spec)))
}

fn parse_bag(input: &str) -> IResult<&str, &str> {
    fn parse_name_part(input: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_alphabetic())(input)
    }

    let (input, (f, _)) = terminated(
        consumed(
            separated_pair(
                parse_name_part,
                tag(" "),
                parse_name_part
            )
        ),
        alt((tag(" bags"), tag(" bag")))
    )(input)?;

    Ok((input, f))
}

#[cfg(test)]
mod tests {
    use crate::day6_10::day7::{parse_raw_bag_specs, parse_bag_spec, RawBagSpec, parse_all_bags_specs, solve_simple, solve_advanced};

    fn input() -> &'static str {
        "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
    }

    fn single_input() -> &'static str {
        "light red bags contain 1 bright white bag, 2 muted yellow bags."
    }

    #[test]
    fn it_should_parse_line_of_input() {
        let input = single_input();
        let (_, RawBagSpec(name, depends)) = parse_bag_spec(input).expect("Failed to parse input");
        assert_eq!(name, "light red");
        assert_eq!(depends, vec![("bright white", 1 as u32), ("muted yellow", 2 as u32)]);
    }

    #[test]
    fn it_should_parse_input() {
        let input = input();
        let (_, result) = parse_raw_bag_specs(input).expect("Failed to parse input");
        let expected_bags = ["light red", "dark orange", "bright white", "muted yellow", "shiny gold", "dark olive", "vibrant plum", "faded blue", "dotted black"];
        assert_eq!(result.len(), expected_bags.len());
    }

    #[test]
    fn it_should_build_proper_graph() {
        let input = input();
        let result = parse_all_bags_specs(input).expect("Failed to parse input");
        assert_eq!(result.graph.len(), input.lines().count());
        assert_eq!(result.graph["dark orange"][0], ("bright white", 3));
        assert_eq!(result.graph["dark orange"][1], ("muted yellow", 4));
    }

    #[test]
    fn it_should_solve_simple_case() {
        let input = input();
        let parsed_input = parse_all_bags_specs(input).expect("Failed to parse input");
        assert_eq!(solve_simple(&parsed_input, "shiny gold"), 4);
    }

    #[test]
    fn it_should_solve_advanced_case() {
        let input = input();
        let parsed_input = parse_all_bags_specs(input).expect("Failed to parse input");
        assert_eq!(solve_advanced(&parsed_input, "shiny gold"), 32);
    }

}