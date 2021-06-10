use itertools::Itertools;
use std::collections::HashSet;

pub fn run() -> String {
    let known_triple: [u64; 3] = [1487, 4817, 8147];

    let primes = primes::prime_seive(10_000);
    let is_prime: HashSet<_> = primes.clone().into_iter().collect();
    let possible_primes = primes
        .into_iter()
        // must be at least 4 digits
        .filter(|&num| num >= 1_000);

    let result = possible_primes
        .into_iter()
        .filter_map(|num| {
            if num < 1000 || num >= 10_000 {
                return None;
            }

            // split out the digits
            let mut digits = Vec::with_capacity(4);
            let mut num = num;
            while num > 0 {
                digits.push(num % 10);
                num /= 10;
            }

            // produce all the permutations
            let mut permutations: Vec<_> = digits
                .into_iter()
                .permutations(4)
                // join the digits back into a number
                .map(|dd| {
                    let mut num = 0;
                    for i in dd {
                        num *= 10;
                        num += i;
                    }
                    num
                })
                // only 4-digit numbers
                .filter(|num| *num > 1_000)
                // only primes
                .filter(|num| is_prime.contains(num))
                .unique()
                .collect();

            // must have at least three terms
            if permutations.len() < 3 {
                return None;
            }

            // ascending order
            permutations.sort();

            Some(permutations)
        })
        // produce all possible triples from the permutations of each number,
        // maintaining the ascending order of each triple
        .flat_map(|prime_set| {
            let mut options: Vec<[u64; 3]> = vec![];
            for i in 0..prime_set.len() - 2 {
                for j in i + 1..prime_set.len() - 1 {
                    for k in j + 1..prime_set.len() {
                        options.push([prime_set[i], prime_set[j], prime_set[k]]);
                    }
                }
            }
            options
        })
        // exclude the known triple
        .filter(|prime_triple| {
            prime_triple[0] != known_triple[0]
                || prime_triple[1] != known_triple[1]
                || prime_triple[2] != known_triple[2]
        })
        // find the first triple that is evenly spaced
        .find_map(|prime_set| {
            if prime_set[2] - prime_set[1] == prime_set[1] - prime_set[0] {
                return Some([prime_set[0], prime_set[1], prime_set[0]]);
            }
            None
        });

    match result {
        Some(primes) => (primes[0] * 100_000_000 + primes[1] * 10_000 + primes[2]).to_string(),
        None => "no_result".into(),
    }
}
