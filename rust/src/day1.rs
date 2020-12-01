
pub fn solve(lines: &Vec<u32>, sum_to: u32) -> Option<(u32, u32, u32)> {
    let mut local_lines: Vec<u32> = lines.to_vec();
    let slice: &mut [u32] = local_lines.as_mut_slice();
    slice.sort_unstable();
    solve_in_place(slice, sum_to)
}

pub fn solve_two(lines: &Vec<u32>, sum_to: u32) -> Option<(u32, u32, u32, u32)> {
    let mut local_lines: Vec<u32> = lines.to_vec();
    let slice: &mut [u32] = local_lines.as_mut_slice();
    slice.sort_unstable();
    slice.iter().find_map(|&n| {
        let sub_solution: Option<(u32, u32, u32)> = solve_in_place(slice, sum_to - n);
        sub_solution.map(|(res, b, c)| (n * res, n, b, c))
    })
}

fn solve_in_place(slice: &[u32], sum_to: u32) -> Option<(u32, u32, u32)> {
    slice.iter().find_map(|&n| match_value(slice, n, sum_to))
}

fn match_value(slice: &[u32], n: u32, sum_to: u32) -> Option<(u32, u32, u32)> {
    if sum_to > n {
        let to_find = sum_to - n;
        let best_match = slice.binary_search(&to_find);
        match best_match {
            Result::Ok(v) => Option::Some((n * slice[v], n, slice[v])),
            Result::Err(_e) => Option::None
        }
    } else {
        Option::None
    }
}