use std::num::ParseIntError;
use std::cmp::max;
use std::convert::TryFrom;

//wtf why it is so simple xd
pub fn solve_simple(arg: &Vec<u16>) -> u32 {
    let device_in_built = arg.iter().max().unwrap() + 3;
    let mut own_data: Vec<_> = arg.iter().chain(vec![0, device_in_built].iter()).cloned().collect();
    own_data.sort();
    let init = (0 as u32, 0 as u32);
    let (by_one, by_three) = own_data
        .iter()
        .zip(
            own_data[1..]
                .iter()
        )
        .fold(init, |(by_one, by_three), (&p, &n)| {
            if n - p == 3 {
                (by_one, by_three + 1)
            } else if n - p == 1{
                (by_one + 1, by_three)
            } else {
                (by_one, by_three)
            }
        });
    println!("Trivial results get me: {}, {}", by_one, by_three);
    by_one * by_three
}

pub fn solve_advanced(arg: &Vec<u16>) -> u64 {
    let mut possibilities = vec![0 as u64; arg.len() + 2];
    possibilities[0] = 1;
    let device_in_built = arg.iter().max().unwrap() + 3;
    let mut own_data: Vec<u16> = arg.iter().chain(vec![0, device_in_built].iter()).cloned().collect();
    own_data.sort();

    for ind in 0..own_data.len() {
        let min_ind = if ind >= 3 {
            ind - 3
        } else {
            let ind_min_candidate = i16::try_from(ind).expect("Should not happen");
            usize::try_from(max(0 as i16, ind_min_candidate - 3)).expect("Should not happen")
        };
        let focus = own_data[ind];
        for j in min_ind..ind {
            if focus - own_data[j] <= 3 {
                possibilities[ind] += possibilities[j];
            }
        }
    }
    possibilities[(own_data.len() - 1)]
}

pub fn parse_input(ipt: Vec<String>) -> Result<Vec<u16>, ParseIntError> {
    ipt.iter().map(|s| s.parse::<u16>()).collect()
}

#[cfg(test)]
mod tests {
    use crate::day6_10::day_10::{parse_input, solve_simple, solve_advanced};

    const INPUT: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn it_should_solve_simple() {
        let lines: Vec<_> = INPUT.lines().map(|s| s.to_string()).collect();
        let parsed = parse_input(lines).expect("Failed to parse input");
        assert_eq!(
            solve_simple(&parsed),
            22 * 10
        )
    }

    #[test]
    fn it_should_solve_advanced() {
        let lines: Vec<_> = INPUT.lines().map(|s| s.to_string()).collect();
        let parsed = parse_input(lines).expect("Failed to parse input");

        assert_eq!(
            solve_advanced(&parsed),
            19208
        )
    }
}