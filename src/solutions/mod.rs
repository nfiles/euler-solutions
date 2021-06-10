extern crate itertools;

pub mod problem001;
pub mod problem002;
pub mod problem003;
pub mod problem049;
pub mod problem050;

pub fn solve(num: u32) -> Option<String> {
    let solution = match num {
        1 => problem001::run(),
        2 => problem002::run(),
        3 => problem003::run(),
        49 => problem049::run(),
        50 => problem050::run(),
        _ => return None,
    };

    Some(solution)
}
