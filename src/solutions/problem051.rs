use primes::prime_seive;
use std::collections::HashMap;
use std::collections::HashSet;
use std::vec::Vec;

/// By replacing the 1st digit of the 2-digit number *3, it turns out that six
/// of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
///
/// By replacing the 3rd and 4th digits of 56**3 with the same digit, this
/// 5-digit number is the first example having seven primes among the ten
/// generated numbers, yielding the family:
/// 56003, 56113, 56333, 56443, 56663, 56773, and 56993.
/// Consequently 56003, being the first member of this family, is the smallest
/// prime with this property.
///
/// Find the smallest prime which, by replacing part of the number (not
/// necessarily adjacent digits) with the same digit, is part of an eight prime
/// value family.

const MINIMUM_REPLACEMENTS: usize = 3;
const PRIME_FAMILY_SIZE: usize = 8;

pub fn run() -> String {
    let mut processed: HashSet<String> = HashSet::new();

    // guess at the max prime in the set
    let max = 1_000_000;

    let primes = prime_seive(max);
    let is_prime: HashSet<u64> = primes.clone().into_iter().collect();
    for prime in &primes {
        if *prime > max {
            return "not found".into();
        }

        // get all possible patterns for the number
        let patterns = get_patterns(prime, MINIMUM_REPLACEMENTS);

        for pattern in patterns {
            let key = get_pattern_key(&pattern);
            // skip if we've already processed this key
            if !processed.insert(key.clone()) {
                continue;
            }

            let candidates = generate_from_pattern(&pattern);
            let mut prime_count = 0;
            for candidate in &candidates {
                if is_prime.contains(&candidate) {
                    prime_count += 1;
                }
            }

            if prime_count >= PRIME_FAMILY_SIZE {
                return format!(
                    "lowest prime is {} with mask {}",
                    candidates.first().unwrap(),
                    get_pattern_key(&pattern)
                );
            }
        }
    }

    "not found :(".into()
}

/// splits a number into its component digits
fn split_digits(num: &u64) -> Vec<u64> {
    let mut num = *num;
    let mut digits: Vec<u64> = Vec::new();
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();

    digits
}

/// generate a number from the given digits
fn combine_digits(digits: &[u64]) -> u64 {
    let mut num = 0;
    for digit in digits {
        num = (num * 10) + digit;
    }

    num
}

/// gets the possible mask patterns for the given number
fn get_patterns(num: &u64, _minimum: usize) -> Vec<Vec<u64>> {
    let digits = split_digits(&num);

    let mut digit_counts: HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 0..digits.len() {
        let digit = digits[i];
        digit_counts
            .entry(digit)
            .or_insert(Vec::new())
            .push(i as u64);
    }

    digit_counts
        .values()
        // only consider digits that occur at least `minimum` times
        .filter(|occurrences| occurrences.len() >= _minimum)
        // create a pattern from the repeated digits
        .map(|occurrences| {
            let mut pattern = digits.clone();
            for i in occurrences {
                pattern[*i as usize] = 10;
            }
            pattern
        })
        .collect()
}

/// create a string key from the pattern
fn get_pattern_key(pattern: &[u64]) -> String {
    pattern
        .iter()
        .cloned()
        .map(|digit| {
            if digit < 10 {
                std::char::from_digit(digit as u32, 10).unwrap()
            } else {
                '*'
            }
        })
        .collect()
}

/// generate all possible numbers from the given pattern
fn generate_from_pattern(pattern: &[u64]) -> Vec<u64> {
    (0..=9)
        .filter_map(|fill| {
            if fill == 0 && pattern[0] > 9 {
                // cannot fill the first digit with '0'
                return None;
            }

            let candidate: Vec<_> = pattern
                .iter()
                .map(|&digit| match digit {
                    0..=9 => digit,
                    _ => fill,
                })
                .collect();

            Some(combine_digits(&candidate))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn split_digits_works() {
        let cases = [
            (12345, vec![1, 2, 3, 4, 5]),
            (54321, vec![5, 4, 3, 2, 1]),
            (101010101, vec![1, 0, 1, 0, 1, 0, 1, 0, 1]),
            (1234321, vec![1, 2, 3, 4, 3, 2, 1]),
            (14253647, vec![1, 4, 2, 5, 3, 6, 4, 7]),
            (000005, vec![5]),
        ];

        for (num, expected) in &cases {
            let actual = split_digits(&num);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    pub fn combine_digits_works() {
        let cases = [
            (vec![1, 2, 3, 4, 5], 12345),
            (vec![0, 0, 0, 0, 5], 5),
            (vec![5, 4, 3, 2, 1], 54321),
        ];

        for (digits, expected) in &cases {
            let actual = combine_digits(digits);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    pub fn get_pattern_key_works() {
        let cases: &[(&[u64], &str)] = &[
            (&[10, 2, 10, 3, 10, 3], "*2*3*3"),
            (&[10, 10, 10, 1, 2, 3], "***123"),
            (&[5, 4, 3, 10, 10, 10], "543***"),
        ];

        for (pattern, expected) in cases {
            let actual = get_pattern_key(pattern);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    pub fn generate_from_pattern_works() {
        let cases: &[(&[u64], &[u64])] = &[
            (
                &[10, 2, 10, 3, 10, 3],
                &[
                    121313, 222323, 323333, 424343, 525353, 626363, 727373, 828383, 929393,
                ],
            ),
            (
                &[10, 10, 10, 1, 2, 3],
                &[
                    111123, 222123, 333123, 444123, 555123, 666123, 777123, 888123, 999123,
                ],
            ),
            (
                &[5, 4, 3, 10, 10, 10],
                &[
                    543000, 543111, 543222, 543333, 543444, 543555, 543666, 543777, 543888, 543999,
                ],
            ),
        ];

        for (pattern, expected) in cases {
            let actual = generate_from_pattern(pattern);
            assert_eq!(*expected, actual);
        }
    }
}
