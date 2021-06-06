/// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
/// find the sum of the even-valued terms.
use std::iter::repeat_with;

pub fn run() -> String {
    const MAX: u32 = 4_000_000;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let total: u32 = repeat_with(|| {
        let next = a + b;
        a = b;
        b = next;
        next
    })
    .take_while(|&num| num <= MAX)
    .filter(|&num| num % 2 == 0)
    .sum();

    total.to_string()
}
