use crate::utils::solutions::Input;

mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6_10;

fn main() {
    day_1_5();
}

fn day_1_5() {
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

    let args_3: Vec<String> = utils::files::lines_from_file("resources/day3/input.txt");
    let parsed_day_3 = day3::parse(args_3);
    let res = day3::solve(&parsed_day_3, &day3::Move{right: 3, down: 1}, day3::Move{right: 0, down: 0});
    let multiple_config: [day3::Move; 5] =
        [day3::Move {right: 1, down: 1}, day3::Move{right: 3, down: 1}, day3::Move{right: 5, down: 1}, day3::Move{right:7, down: 1}, day3::Move{right: 1, down: 2}];
    println!("Result of day 3: {res}", res = res);
    let res = multiple_config.iter()
        .map(|m| {
            day3::solve(&parsed_day_3, m, day3::Move{right: 0, down: 0})
        })
        .fold(1usize, |l, r| l * r);

    println!("Result of day 3 (part 2): {res}", res = res);

    let day_4_contents = utils::files::resource_contents("resources/day4/input.txt");
    let parsed_day_4 = day4::parse_input(day_4_contents.as_str());
    let simple_day_4_res = day4::solve_simple(&parsed_day_4);
    println!("Result of day 4: {res}", res = simple_day_4_res);
    let advanced_day_4_res = day4::solve_advanced(&parsed_day_4);
    println!("Result of day 4 (part 2): {res}", res = advanced_day_4_res);

    let day_5_contents = utils::files::lines_from_file("resources/day5/input.txt");
    let input = Input::new(&day_5_contents);
    fn parse_5(arg: &Vec<String>) -> Result<&Vec<String>, ()> { Ok(arg) }
    input.perform_solution(parse_5, day5::solve, day5::solve_advanced, "day 5");
    println!("Hello, world!");
}

fn day_6_10() {

}