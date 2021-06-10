use std::collections::HashSet;

/// Creates a list of all primes in the range (0..max)
pub fn prime_seive(max: u64) -> Vec<u64> {
    let mut composite: HashSet<u64> = HashSet::new();
    let mut primes: Vec<u64> = Vec::new();

    for num in 2..max {
        if composite.contains(&num) {
            continue;
        }

        primes.push(num);

        for mult in num..(max / (num - 1)) {
            composite.insert(num * mult);
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_seive_works() {
        let expected = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];

        let actual = prime_seive(25);

        assert_eq!(expected, actual);
    }
}
