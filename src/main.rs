extern crate env_logger;
extern crate log;
#[macro_use]
extern crate clap;

extern crate primes;

use clap::{App, Arg};

mod solutions;

const PROBLEM: &str = "number";
const TIMING: &str = "timing";

fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name(PROBLEM)
                .value_name("problem")
                .help("number of the project euler solution"),
        )
        .arg(
            Arg::with_name(TIMING)
                .long("time")
                .takes_value(false)
                .help("report how long it took to find the solution"),
        )
        .setting(clap::AppSettings::AllowLeadingHyphen)
        .get_matches();

    let timing = matches.is_present(TIMING);

    let number_arg = match matches.value_of(PROBLEM) {
        None => {
            print!("{}", solutions::progress());
            std::process::exit(0);
        }
        Some(number_str) => number_str,
    };

    let number = match number_arg.parse::<u32>() {
        Err(err) => {
            eprintln!("Error parsing the number: {}", err);
            std::process::exit(1);
        }
        Ok(number) => number,
    };

    let start = std::time::Instant::now();
    let solution = solutions::solve(number);
    let elapsed = start.elapsed();

    match solution {
        None => {
            eprintln!("no solution for problem {}", number);
            std::process::exit(2);
        }
        Some(result) => {
            println!("problem {}: {}", number, result);
            if timing {
                println!("completed in {} seconds", elapsed.as_secs_f32());
            }
        }
    }
}
