fn main() {
    let mut args = std::env::args();
    args.next().expect("skipping the executable name");
    let argument = args.next().expect("number argument missing");
    let number: u64 = argument.parse().expect("argument is not a number");
    println!("{}: {:?}", number, factorize(number));
    //let n_sqrt = (number as f64).sqrt().ceil() as u64;
    //println!("{} prime numbers computed", prime_sieve(n_sqrt).len());
}

struct PrimeIterator {
    i: u64,
    primes: Vec<u64>,
}

impl PrimeIterator {
    fn new() -> Self {
        PrimeIterator {
            i: 2,
            primes: Vec::new(),
        }
    }
}

impl Iterator for PrimeIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let mut i = self.i;
        loop {
            if !self.primes.iter().any(|p| i % p == 0) {
                self.primes.push(i);
                self.i = i + 1;
                return Some(i);
            }
            i += 1;
        }
    }
}

fn prime_sieve(n: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for i in 2..=n {
        if !primes.iter().any(|p| i % p == 0) {
            primes.push(i);
        }
    }
    primes
}

fn factorize(n: u64) -> Vec<u64> {
    let mut primes = PrimeIterator::new();
    let mut factors = Vec::new();
    let mut n = n;
    let mut prime = match primes.next() {
        Some(p) => p,
        None => {
            return vec![n];
        }
    };
    while n > 1 {
        if n % prime == 0 {
            factors.push(prime);
            n /= prime;
        } else {
            prime = match primes.next() {
                Some(p) => p,
                None => {
                    factors.push(n);
                    return factors;
                }
            }
        }
    }
    factors
}

mod tests {
    use super::*;

    #[test]
    fn prime_sieve_up_to_20() {
        assert_eq!(prime_sieve(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn prime_iterator() {
        let mut iter = PrimeIterator::new();
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next(), Some(11));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(17));
        assert_eq!(iter.next(), Some(19));
    }

    #[test]
    fn factorize_234() {
        assert_eq!(factorize(234), vec![2, 3, 3, 13]);
    }
}
