use rayon::prelude::*;

/// Primality test
///
/// From https://en.wikipedia.org/wiki/Primality_test
fn is_prime(n: usize) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if (n % 2 == 0) || (n % 3 == 0) {
        return false;
    }

    let mut i = 5;
    while i << 1 <= n {
        if (n % i == 0) || (n % (i + 2) == 0) {
            return false;
        }
        i += 6;
    }

    true
}

pub fn sequential_primality(limit: usize) -> Vec<usize> {
    (2..limit).filter(|&n| is_prime(n)).collect()
}

pub fn rayon_primality(limit: usize) -> Vec<usize> {
    (2..limit)
        .into_par_iter()
        .filter(|&n| is_prime(n))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const PRIMES: [usize; 50] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229,
    ];

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(55));
        for &n in PRIMES.iter() {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn test_sequential_primality() {
        let primes = sequential_primality(230);
        assert_eq!(PRIMES, &primes[..]);
    }

    #[test]
    fn test_rayon_primality() {
        let primes = rayon_primality(230);
        assert_eq!(PRIMES, &primes[..]);
    }
}
