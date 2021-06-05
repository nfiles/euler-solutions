extern crate env_logger;
extern crate log;

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use std::iter::repeat_with;
    use std::sync::Once;

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn init() {
        INIT.call_once(env_logger::init);
    }

    /// Find the sum of all the multiples of 3 or 5 below 1000.
    #[test]
    fn problem001() {
        init();

        const MAX: u32 = 1000;
        let sum: u32 = (1..MAX).filter(|i| i % 3 == 0 || i % 5 == 0).sum();

        info!("problem 001: {}", sum);
    }

    /// By considering the terms in the Fibonacci sequence whose values do not exceed four million,
    /// find the sum of the even-valued terms.
    #[test]
    fn problem002() {
        init();

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

        info!("problem 002: {}", total);
    }

    /// What is the largest prime factor of the number 600851475143 ?
    #[test]
    fn problem003() {
        init();

        let mut num: u64 = 600851475143;
        let mut factor: u64 = 1;
        for n in 2.. {
            while num % n == 0 {
                factor = n;
                num /= n;
            }
            if num == 1 {
                break;
            }
        }

        info!("problem 003: {}", factor);
    }
}
