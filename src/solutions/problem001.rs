/// Find the sum of all the multiples of 3 or 5 below 1000.

pub fn run() -> String {
    const MAX: u32 = 1000;
    let sum: u32 = (1..MAX).filter(|i| i % 3 == 0 || i % 5 == 0).sum();

    sum.to_string()
}
