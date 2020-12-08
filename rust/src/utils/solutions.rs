use std::fmt;
use nom::lib::std::fmt::Debug;

pub struct Input<T>(T);

impl <RI: Clone> Input<RI> {
    pub fn perform_solution<I, E, R1, R2, F1, F2, F3>(
        &self,
        parse_input: F1,
        solve_one: F2,
        solve_two: F3,
        name: &str
    )  where
        F1: Fn(RI) -> Result<I, E>,
        F2: Fn(&I) -> R1,
        F3: Fn(&I) -> R2,
        E: fmt::Debug,
        R1: Debug,
        R2: Debug,
    {
        let parsed = parse_input(self.0.clone()).expect(format!("Failed to parse input: {}", name).as_str());
        let res_one = solve_one(&parsed);
        println!("Result of {name}: {res:?}", name = name, res = res_one);
        let res_two = solve_two(&parsed);
        println!("Result of {name} (day 2): {res:?}", name = name, res = res_two);
    }
}

impl Input<()> {
    pub fn new<T>(arg: T) -> Input<T> {
       Input(arg)
    }
}