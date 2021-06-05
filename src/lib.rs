extern crate env_logger;
extern crate log;

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
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
}
