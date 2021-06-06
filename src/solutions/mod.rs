pub mod problem001;
pub mod problem002;
pub mod problem003;

pub fn solve(num: u32) -> Option<String> {
    let solution = match num {
        1 => problem001::run(),
        2 => problem002::run(),
        3 => problem003::run(),
        _ => return None,
    };

    Some(solution)
}
