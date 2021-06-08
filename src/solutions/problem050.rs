/// Which prime, below one-million, can be written as the sum of the most consecutive primes?
use std::collections::HashSet;

use primes::prime_seive;

pub fn run() -> String {
    const MAX: u64 = 1_000_000;

    let primes = prime_seive(MAX);
    let is_prime: HashSet<u64> = primes.clone().into_iter().collect();

    // find all sums
    let mut sums: Vec<u64> = vec![0];
    // assume that the final sequence lies within the first 1/10th of possible primes
    for p in &primes[..primes.len() / 10] {
        sums.push(sums[sums.len() - 1] + p);
    }

    // there must be at least one term in the set
    for terms in (1..sums.len()).rev() {
        for start in 0..(sums.len() - terms) {
            let end = start + terms;
            let sum = sums[end] - sums[start];

            if is_prime.contains(&sum) {
                return sum.to_string();
            }
        }
    }

    panic!("a sequence of length 1 should have passed")
}
