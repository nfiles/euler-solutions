use primes::prime_seive;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

    let max = 10_000_000;

    // println!("find primes up to {}", max);
    let before = std::time::Instant::now();
    let primes = prime_seive(max);
    let is_prime: HashSet<u64> = primes.clone().into_iter().collect();
    println!(
        "found primes up to {} in {} seconds",
        max,
        before.elapsed().as_secs_f32(),
    );
    let primes = [121313];
    for prime in &primes {
        if *prime > max {
            return "not found".into();
        }

        // let digits = split_digits(prime);
        // let masks = get_masks(&digits, MINIMUM_REPLACEMENTS);

        let patterns = get_patterns2(prime, MINIMUM_REPLACEMENTS);

        // get all possible patterns for the number
        for pattern in patterns {
            // for mask in masks {
            // let pattern = generate_pattern(prime, &mask);
            let key = get_pattern_key(&pattern);
            println!("pattern ({}) {:?}", key, pattern);
            if !processed.insert(key.clone()) {
                continue;
            }

            let candidates = generate_from_pattern2(&pattern);
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

    // for digit_count in 5_u32.. {
    //     let masks = generate_masks(digit_count as usize);

    //     let before = std::time::Instant::now();
    //     let primes = prime_seive(10_u64.pow(digit_count));
    //     let is_prime: HashSet<u64> = primes.clone().into_iter().collect();
    //     let elapsed = before.elapsed().as_secs_f32();
    //     println!(
    //         "found primes up to {} in {} seconds...",
    //         10_u64.pow(digit_count),
    //         elapsed
    //     );

    //     for num in 10_u64.pow(digit_count - 3)..10_u64.pow(digit_count - 2) {
    //         for mask in &masks {
    //             let candidates = generate_from_pattern(num, &mask);
    //             let mut prime_count = 0;
    //             for digits in &candidates {
    //                 if digits[0] == 0 {
    //                     continue;
    //                 }
    //                 let candidate = combine_digits(&digits);
    //                 if processed.contains(&candidate) {
    //                     continue;
    //                 } else {
    //                     processed.insert(candidate);
    //                 }

    //                 if is_prime.contains(&candidate) {
    //                     prime_count += 1;
    //                 }
    //             }

    //             if prime_count >= 8 {
    //                 let result = combine_digits(&candidates[0]);
    //                 return format!("lowest prime is {}", result);
    //             }
    //         }
    //     }
    // }

    "not found :(".into()
}

/// generates a set of masks of length `size` with `replace` `true` values in all
/// possible position combinations
fn generate_masks2(size: usize, replace: usize) -> Vec<Vec<bool>> {
    let num_possiblities = size * (size - 1) / 2;
    let mut masks: Vec<Vec<bool>> = Vec::with_capacity(num_possiblities);

    for i in 0..size - 1 {
        for j in i + 1..size {
            let mut mask = vec![false; size];
            mask[i] = true;
            mask[j] = true;
            masks.push(mask);
        }
    }

    masks
}

// fn generate_mask(size: usize, mask: usize) -> Vec<Vec<bool>> {
//     if mask == 0 {
//         return vec![vec![false; size]];
//     }

//     if size == 0 {
//         return vec![];
//     } else if size == 1 {
//         return vec![vec![true], vec![false]];
//     } else {
//         let base = generate_mask(size - 1);
//     }
// }

/// generates a set of masks of length `count` with 2 `true` values in all
/// possible position combinations
fn generate_masks(count: usize) -> Vec<Vec<bool>> {
    let num_possiblities = count * (count - 1) / 2;
    let mut masks: Vec<Vec<bool>> = Vec::with_capacity(num_possiblities);

    for i in 0..count - 1 {
        for j in i + 1..count {
            let mut mask = vec![false; count];
            mask[i] = true;
            mask[j] = true;
            masks.push(mask);
        }
    }

    masks
}

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

fn get_patterns2(num: &u64, _minimum: usize) -> Vec<Vec<u64>> {
    let digits = split_digits(&num);

    // TODO: don't hard-code 2 wildcard digits
    generate_masks(digits.len())
        .iter()
        .filter(|mask| mask[0] != true)
        .map(|mask| {
            (0..mask.len())
                .map(|i| match mask[i] {
                    true => 10,
                    false => digits[i],
                })
                .collect()
        })
        .collect()
}

fn get_patterns(num: &u64, minimum: usize) -> Vec<Vec<u64>> {
    let digits = split_digits(&num);
    let mut digit_counts: HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 0..digits.len() {
        digit_counts
            .entry(digits[i])
            .or_insert(Vec::new())
            .push(i as u64);
    }

    // get the set of all masks
    let patterns = digit_counts
        .values()
        .filter(|occurrences| occurrences.len() >= minimum)
        .flat_map(|occurrences| {
            // get all combinations of mask

            vec![occurrences]
        })
        .map(|occurrences| {
            let mut mask = vec![false; digits.len()];
            for i in occurrences {
                mask[*i as usize] = true;
            }
            mask
        })
        .map(|mask| {
            let mut pattern = digits.clone();
            for i in 0..mask.len() {
                pattern[i] = 10;
            }
            pattern
        })
        .collect();

    patterns

    // let mut patterns = Vec::new();

    // for mask in &masks {
    //     let mut pattern = digits.clone();
    //     for i in 0..mask.len() {
    //         pattern[i] = 10;
    //     }
    //     patterns.push(pattern);
    // }

    // patterns
}

fn generate_pattern(num: &u64, mask: &[bool]) -> Vec<u64> {
    let mut digits = split_digits(&num);
    for i in 0..mask.len() {
        digits[i] = 10;
    }
    digits
}

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

fn generate_from_pattern2(pattern: &[u64]) -> Vec<u64> {
    (0..=9)
        .map(|fill| {
            let candidate: Vec<_> = pattern
                .iter()
                .map(|&digit| match digit {
                    0..=9 => digit,
                    _ => fill,
                })
                .collect();
            // let mut candidate = pattern.clone();

            // for i in 0..mask.len() {
            //     if mask[i] {
            //         candidate[i] = fill;
            //     }
            // }

            combine_digits(&candidate)
        })
        .collect()
}

fn generate_from_pattern(num: u64, pattern: &[bool]) -> [Vec<u64>; 10] {
    let digits: VecDeque<_> = split_digits(&num).into_iter().collect();
    if pattern.len() != digits.len() + 2 {
        panic!("the pattern must be x longer than the digits",);
    }

    let mut results: [Vec<u64>; 10] = Default::default();

    for fill in 0..=9 {
        let candidate = &mut results[fill as usize];
        let mut digits = digits.clone();
        for i in 0..pattern.len() {
            let digit = match pattern[i] {
                true => fill,
                false => digits.pop_front().expect("no digits left"),
            };

            candidate.push(digit);
        }
    }

    results
}

fn get_masks(digits: &[u64], minimum: usize) -> Vec<Vec<bool>> {
    let mut digit_counts: HashMap<u64, Vec<u64>> = HashMap::new();

    for i in 0..digits.len() {
        let digit = digits[i];
        digit_counts
            .entry(digit)
            .or_insert(Vec::new())
            .push(i as u64);
    }

    let passing_digits: Vec<Vec<bool>> = digit_counts
        .values()
        .filter(|occurrences| occurrences.len() >= minimum)
        .map(|occurrences| {
            let mut mask = vec![false; digits.len()];
            for i in occurrences {
                mask[*i as usize] = true;
            }
            mask
        })
        .collect();

    passing_digits
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
    pub fn generate_masks_works() {
        let cases = [
            (
                3,
                vec![
                    vec![true, true, false],
                    vec![true, false, true],
                    vec![false, true, true],
                ],
            ),
            (
                4,
                vec![
                    vec![true, true, false, false],
                    vec![true, false, true, false],
                    vec![true, false, false, true],
                    vec![false, true, true, false],
                    vec![false, true, false, true],
                    vec![false, false, true, true],
                ],
            ),
            (
                5,
                vec![
                    vec![true, true, false, false, false],
                    vec![true, false, true, false, false],
                    vec![true, false, false, true, false],
                    vec![true, false, false, false, true],
                    vec![false, true, true, false, false],
                    vec![false, true, false, true, false],
                    vec![false, true, false, false, true],
                    vec![false, false, true, true, false],
                    vec![false, false, true, false, true],
                    vec![false, false, false, true, true],
                ],
            ),
        ];

        for (count, expected) in &cases {
            let actual = generate_masks(*count as usize);
            assert_eq!(*expected, actual);
        }
    }

    #[test]
    pub fn generate_from_pattern_works() {
        let cases: &[(u64, &[bool], [Vec<u64>; 10])] = &[
            (
                111,
                &[false, true, false, true, false],
                [
                    vec![1, 0, 1, 0, 1],
                    vec![1, 1, 1, 1, 1],
                    vec![1, 2, 1, 2, 1],
                    vec![1, 3, 1, 3, 1],
                    vec![1, 4, 1, 4, 1],
                    vec![1, 5, 1, 5, 1],
                    vec![1, 6, 1, 6, 1],
                    vec![1, 7, 1, 7, 1],
                    vec![1, 8, 1, 8, 1],
                    vec![1, 9, 1, 9, 1],
                ],
            ),
            (
                11,
                &[false, true, true, false],
                [
                    vec![1, 0, 0, 1],
                    vec![1, 1, 1, 1],
                    vec![1, 2, 2, 1],
                    vec![1, 3, 3, 1],
                    vec![1, 4, 4, 1],
                    vec![1, 5, 5, 1],
                    vec![1, 6, 6, 1],
                    vec![1, 7, 7, 1],
                    vec![1, 8, 8, 1],
                    vec![1, 9, 9, 1],
                ],
            ),
        ];

        for (num, mask, expected) in cases {
            let actual = generate_from_pattern(*num, mask);
            assert_eq!(*expected, actual);
        }
    }
}
