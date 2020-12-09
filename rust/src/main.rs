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
    day_6_10();
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
    fn parse_5(arg: &Vec<String>) -> Result<Vec<String>, ()> { Ok(arg.clone()) }
    input.perform_solution(parse_5, day5::solve, day5::solve_advanced, "day 5");
    println!("Hello, world!");
}

fn day_6_10() {
    let day_6_contents = utils::files::resource_contents("resources/day6/input.txt");
    let input = Input::new(day_6_contents);
    input.perform_solution(day6_10::day6::parse_input, day6_10::day6::solve_simple, |_| 0, "day 6");

    let day_7_contents = utils::files::resource_contents("resources/day7/input.txt");
    let day_7_input = Input::new(day_7_contents.as_str());
    fn solve_simple(arg: &day6_10::day7::BagSpecs) -> u32 {
        day6_10::day7::solve_simple(arg, "shiny gold")
    }
    fn solve_advanced(arg: &day6_10::day7::BagSpecs) -> u32 {
        day6_10::day7::solve_advanced(arg, "shiny gold")
    }

    day_7_input.perform_solution(day6_10::day7::parse_all_bags_specs, solve_simple, solve_advanced, "day 7");

    let day_8_contents = utils::files::lines_from_file("resources/day8/input.txt");
    let day_8_input = Input::new(day_8_contents);
    day_8_input.perform_solution(
        day6_10::day8::parse_input,
        day6_10::day8::solve_simple,
        day6_10::day8::solve_advanced,
        "day 8"
    );

    let day_9_contents = utils::files::lines_from_file("resources/day9/input.txt");
    let day_9_input = Input::new(day_9_contents);
    day_9_input.perform_solution(
        day6_10::day9::parse_input,
        |ipt| day6_10::day9::solve_simple(ipt, 25),
        |ipt| day6_10::day9::brute_force_advanced(ipt, 25),
        "day 9"
    );
}