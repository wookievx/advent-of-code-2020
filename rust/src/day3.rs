use std::convert::TryFrom;

#[derive(Eq, PartialEq)]
pub enum GridContent {
    Empty,
    Tree
}

pub struct Move {
    pub(crate) right: u16,
    pub(crate) down: u16
}

impl Move {
    fn add_mod(&self, next: &Move, right_mod: u16) -> Move {
        Move {
            right: (self.right + next.right) % right_mod,
            down: self.down + next.down
        }
    }
}

pub fn parse(input: Vec<String>) -> Vec<Vec<GridContent>> {
    input
        .iter()
        .map(|s| -> Vec<GridContent> {
            s.chars()
                .map(|c| {
                if c == '.' {
                    GridContent::Empty
                } else if c == '#' {
                    GridContent::Tree
                } else {
                    panic!("Unsupported input: {}", c)
                }
            }).collect()
        })
        .collect()
}

pub fn solve(input: &Vec<Vec<GridContent>>, mve: &Move, mut position: Move) -> usize {
    let dim: u16 = u16::try_from(input[0].len()).expect("Illegal size of input");
    let mut acc: usize = 0;
    while usize::from(position.down) <= (input.len() - usize::from(mve.down)) {
        if input[usize::from(position.down)][usize::from(position.right)] == GridContent::Tree {
            acc += 1;
        }
        position = position.add_mod(&mve, dim);
    }
    acc
}