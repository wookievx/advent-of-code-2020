mod utils;
mod day1;
mod day2;

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

    let args2: Vec<String> = utils::files::lines_from_file("resources/day2/input.txt");
    let parsed_day2 = day2::parse_input(args2);
    let res2 = day2::solve(&parsed_day2);
    let res_as2adv = day2::solve_adv(&parsed_day2);
    println!("Result of day2: {res}", res = res2);
    println!("Result of day2(part 2): {res}", res = res_as2adv);
    println!("Hello, world!");
}
