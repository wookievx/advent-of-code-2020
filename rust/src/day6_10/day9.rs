
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::num::ParseIntError;

struct Acceptable {
    symmetric_edges: HashMap<u64, HashSet<u64>>,
    possibilities: HashMap<u64, u32>
}

impl Acceptable {

    fn init(batch: &[u64]) -> Acceptable {
        let mut builder = Acceptable {
            symmetric_edges: HashMap::new(),
            possibilities: HashMap::new()
        };
        for n in batch {
            builder.add_number(*n);
        }
        builder
    }

    fn possibilities(&self) -> HashMap<u64, u32> {
        let mut res: HashMap<u64, u32> = HashMap::new();
        let sums: Vec<u64> =
            self.symmetric_edges
                .iter()
                .flat_map(|(n, hs)| {
                    hs.iter().cloned().filter_map(move |sn| {
                        if *n < sn {
                            Some(n + sn)
                        } else {
                            None
                        }
                    })
                })
                .collect();
        for s in sums {
            let current = *res.get(&s).unwrap_or(&0);
            res.insert(s, current + 1);
        }
        res
    }

    fn remove_number(&mut self, number: u64) {
        let neighbours = self.neighbours(number);
        for n in neighbours {
            match self.possibilities.entry(number + n) {
                Entry::Occupied(mut e) => {
                    if *e.get() > 1 as u32 {
                        e.insert(e.get() - 1);
                    } else {
                        e.remove();
                    }
                },
                Entry::Vacant(_) => {} //should not happen
            }
        }
        self.remove_node(number);
    }

    fn add_number(&mut self, number: u64) {
        let neighbours = self.nodes();
        self.add_node(number);

        for n in neighbours {
            self.add_edge(number, n);
            match self.possibilities.entry(n + number) {
                Entry::Occupied(mut e) => {
                    e.insert(e.get() + 1);
                },
                Entry::Vacant(e) => {
                    e.insert(1);
                }
            }
        }

    }

    fn remove_node(&mut self, number: u64) {
        if self.symmetric_edges.contains_key(&number) {
            let edges: Vec<u64> = self.symmetric_edges[&number].iter().cloned().collect();
            for n in edges {
                match self.symmetric_edges.entry(n) {
                    Entry::Occupied(mut e) => {
                        e.get_mut().remove(&number);
                    },
                    Entry::Vacant(_) => {} //should not happen
                }
            }
        }
        self.symmetric_edges.remove(&number);
    }

    fn add_node(&mut self, number: u64) {
        if !self.symmetric_edges.contains_key(&number) {
            self.symmetric_edges.insert(number, HashSet::new());
        }
    }

    fn add_edge(&mut self, from: u64, to: u64) {
        self.add_edge_impl(from, to);
        self.add_edge_impl(to, from);
    }

    fn add_edge_impl(&mut self, from: u64, to: u64) {
        match self.symmetric_edges.entry(from) {
            Entry::Occupied(mut e) => {
                e.get_mut().insert(to);
            },
            Entry::Vacant(e) => {
                e.insert(vec![to].iter().cloned().collect());
            }
        }
    }

    fn neighbours(&self, n: u64) -> Vec<u64> {
        match self.symmetric_edges.get(&n) {
            Some(s) => s.iter().cloned().collect(),
            None => vec![]
        }
    }

    fn nodes(&self) -> Vec<u64> {
        self.symmetric_edges.keys().cloned().collect()
    }

    fn is_possible(&self, sum: u64) -> bool {
        self.possibilities.contains_key(&sum)
    }

}

pub fn parse_input(input: Vec<String>) -> Result<Vec<u64>, ParseIntError> {
    input.iter().map(|s| s.parse::<u64>()).collect()
}

pub fn solve_simple(input: &Vec<u64>, grouping: usize) -> u64 {
    let sliding = &input[grouping..];
    let sliding_init = &input[0..(input.len() - grouping)];
    let start = &input[0..grouping];
    let mut state = Acceptable::init(start);
    sliding
        .iter()
        .zip(sliding_init.iter())
        .find_map(|(&next_num, &to_forget)| {
            if !state.is_possible(next_num) {
                Some(next_num)
            } else {
                state.remove_number(to_forget);
                state.add_number(next_num);
                None
            }
        })
        .unwrap()
}

pub fn solve_advanced(input: &Vec<u64>, grouping: usize) -> (u64, u64, u64) {
    let target = solve_simple(input, grouping);
    let mut sums = vec![0 as u64; input.len() + 1];
    let mut last_sum: u64 = 0;
    sums[0] = 0;
    for i in 0..input.len() {
        last_sum += input[i];
        sums[i + 1] = last_sum;
    }

    let (start, end) = sums
        .iter()
        .enumerate()
        .find_map(|(start_ind, &v)| {
            if v > target {
                None
            } else {
                let search = target + v;
                let res: Option<(usize, usize)> =
                    sums[(start_ind+1)..]
                        .binary_search(&search)
                        .map(|end_ind| (start_ind, start_ind + end_ind + 1))
                        .ok();
                res
            }
        })
        .unwrap();

    let min = input[start..end].iter().min().unwrap();
    let max = input[start..end].iter().max().unwrap();
    (*min, *max, min + max)
}

#[cfg(test)]
mod tests {
    use crate::day6_10::day9::{parse_input, solve_simple, solve_advanced};

    const INPUT: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn it_should_solve_simple_case() {
        let lines = INPUT.lines().map(|s| s.to_string()).collect();
        let parsed = parse_input(lines).expect("Failed to parse input");
        let result = solve_simple(&parsed, 5);
        assert_eq!(result, 127)
    }

    #[test]
    fn it_should_solve_advanced_case() {
        let lines = INPUT.lines().map(|s| s.to_string()).collect();
        let parsed = parse_input(lines).expect("Failed to parse input");
        let (min, max, sum) = solve_advanced(&parsed, 5);
        println!("Got: {}, {}, {}", min, max, sum);
        assert_eq!(min, 15);
        assert_eq!(max, 47);
        assert_eq!(sum, 15 + 47);
    }
    
}