mod utils;
mod day1;

fn main() {
    let args1: Vec<u32> =
        utils::files::lines_from_file("resources/day1/input.txt")
            .iter()
            .map(|s| s.parse::<u32>().expect(format!("Illegal input {}", s).as_str()))
            .collect();
    if let Some((res, a, b)) = day1::solve(&args1, 2020) {
        println!("Result of day1: {}, a = {}, b = {}", res, a, b);
    } else {
        println!("Failed to find solution for day1");
    }

    if let Some((res, a, b, c)) = day1::solve_two(&args1, 2020) {
        println!("Result of day1 (part 2): {res}, a = {a}, b = {b}, c = {c}", res = res, a = a, b = b, c = c)
    } else {
        println!("Failed to find solution for day1");
    }
    println!("Hello, world!");
}
