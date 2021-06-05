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

    #[test]
    fn problem001() {
        init();

        const MAX: u32 = 1000;
        let sum: u32 = (1..MAX).filter(|i| i % 3 == 0 || i % 5 == 0).sum();

        info!("problem 001: {}", sum);
    }

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
}
