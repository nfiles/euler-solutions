/// What is the largest prime factor of the number 600851475143 ?

pub fn run() -> String {
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

    factor.to_string()
}
