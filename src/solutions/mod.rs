extern crate itertools;

use std::collections::HashMap;
use std::fmt::Write;

pub mod problem001;
pub mod problem002;
pub mod problem003;
pub mod problem049;
pub mod problem050;
pub mod problem051;

pub type Solver = fn() -> String;

pub fn all_solutions() -> HashMap<u32, Solver> {
    let mut solutions: HashMap<u32, Solver> = HashMap::new();

    solutions.insert(1, problem001::run);
    solutions.insert(2, problem002::run);
    solutions.insert(3, problem003::run);
    solutions.insert(49, problem049::run);
    solutions.insert(50, problem050::run);
    solutions.insert(51, problem051::run);

    solutions
}

pub fn solve(num: u32) -> Option<String> {
    let solutions = all_solutions();
    let solution = solutions.get(&num);

    solution.map(|solver| solver())
}

pub fn progress() -> String {
    let solutions = all_solutions();
    let max_solution = *solutions.keys().max().unwrap_or(&0);
    let num_width = String::from(format!("{}", max_solution)).len();

    let row_len = 10;

    let mut buffer = String::new();

    writeln!(&mut buffer, "Project Euler solutions").unwrap();

    for row in 0..=(max_solution / row_len) {
        for col in 0..row_len {
            let num = row * row_len + col + 1;
            let solution = solutions.get(&num);
            write!(
                &mut buffer,
                "{: >width$}: [{}]",
                num,
                match solution {
                    Some(_) => "X",
                    None => " ",
                },
                width = num_width,
            )
            .unwrap();

            if col + 1 < row_len {
                write!(&mut buffer, "  ").unwrap();
            }
        }
        writeln!(&mut buffer).unwrap();
    }

    buffer
}
