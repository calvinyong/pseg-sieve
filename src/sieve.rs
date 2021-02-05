use rayon::prelude::*;

/// Sequential sieve of Eratosthenes
///
/// The code was written based on pseudocode from
/// https://research.cs.wisc.edu/techreports/1990/TR909.pdf.
///
/// # Arguments
///
/// * `upper_bound` - The primes up to to this number
pub fn sequential_sieve(upper_bound: usize) -> Vec<usize> {
    // Edge case
    if upper_bound == 0 {
        return vec![];
    }

    // Array of bools. 0 and 1 are not prime
    let mut sieve: Vec<bool> = vec![true; upper_bound + 1];
    sieve[0] = false;
    sieve[1] = false;

    // Sieve
    for i in 2..((upper_bound as f64).sqrt().ceil() as usize) {
        if sieve[i] {
            let mut j = i * i;
            while j <= upper_bound {
                sieve[j] = false;
                j += i;
            }
        }
    }

    // Find all indices that are true
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x { Some(i) } else { None })
        .collect()
}

/// Given start and end indices, return indices of the chunks such that the
/// chunks are equal size or at most differ by one element. The first chunks
/// will be bigger than the last chunks.
fn get_partition_indices(start: usize, end: usize, num_chunks: usize) -> Vec<usize> {
    let dist = end - start;
    let div = dist / num_chunks;
    let modulus = dist % num_chunks;

    let mut vec = vec![start];
    let mut cumsum = start;
    for _i in 0..modulus {
        cumsum += div + 1;
        vec.push(cumsum);
    }
    for _i in 0..(num_chunks - modulus) {
        cumsum += div;
        vec.push(cumsum);
    }

    vec
}

fn single_segment_sieve(primes: &[usize], start: usize, end: usize) -> Vec<usize> {
    if end <= start {
        panic!("end <= start");
    }
    let mut sieve = vec![true; end - start + 1];

    // Sieve the prime factors from the segment
    for prime in primes.iter() {
        let mut curr_num = start / prime * prime;
        if curr_num < start {
            curr_num += prime;
        }
        while curr_num <= end {
            sieve[curr_num - start] = false;
            curr_num += prime;
        }
    }

    // Get the indices that are true
    sieve
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x { Some(i + start) } else { None })
        .collect::<Vec<usize>>()
}

/// Segmented sieve of Eratosthenes
///
/// This is also implemented from https://research.cs.wisc.edu/techreports/1990/TR909.pdf.
pub fn sequential_segmented_sieve(limit: usize, num_segments: usize) -> Vec<usize> {
    // Edge case
    if limit == 0 {
        return vec![];
    }

    // Sieve the primes up to sqrt
    let sqrt_bound = (limit as f64).sqrt().ceil() as usize;
    let mut primes = sequential_sieve(sqrt_bound);

    // Partition the end of the sieve, get indices for chunks
    let indices = get_partition_indices(sqrt_bound, limit, num_segments);

    // Spawn thread for each chunk
    let mut segment_primes = Vec::new();
    for i in 0..num_segments {
        let (start, end) = (indices[i], indices[i + 1] - 1);
        segment_primes.push(single_segment_sieve(&primes[..], start, end));
    }

    // Get primes from each thread, and append to first primes
    for mut segment in segment_primes.into_iter() {
        primes.append(&mut segment);
    }
    primes
}

pub fn rayon_segmented_sieve(limit: usize, num_segments: usize) -> Vec<usize> {
    // Edge case
    if limit == 0 {
        return vec![];
    }

    // Sieve the primes up to sqrt
    let sqrt_bound = (limit as f64).sqrt().ceil() as usize;
    let mut primes = sequential_sieve(sqrt_bound);

    // Partition the end of the sieve, get indices for chunks
    let indices = get_partition_indices(sqrt_bound, limit, num_segments);

    let segment_primes: Vec<Vec<usize>> = indices
        .par_windows(2)
        .map(|slice| {
            if let [start, end] = slice {
                single_segment_sieve(&primes, *start, *end - 1)
            } else {
                vec![]
            }
        })
        .collect();

    for mut segment in segment_primes.into_iter() {
        primes.append(&mut segment);
    }

    primes
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
    fn test_partition_indices() {
        assert_eq!(11, get_partition_indices(0, 100, 10).len());

        let indices = get_partition_indices(31623, 1_000_000_000, 10000);
        assert_eq!(10001, indices.len());

        let windows: Vec<&[usize]> = indices.windows(2).collect();
        assert_eq!(10000, windows.len());
    }

    #[test]
    fn test_sequential_sieve() {
        assert_eq!(PRIMES, &sequential_sieve(230)[..]);

        let primes = sequential_sieve(100_000_000);
        assert_eq!(279209790387276usize, primes.iter().sum());
    }

    #[test]
    fn test_seq_segmented_sieve() {
        // Test first few primes
        assert_eq!(PRIMES, &sequential_segmented_sieve(230, 12)[..]);

        // Big test
        for &i in [20, 1000, 10000, 20000].iter() {
            let primes = sequential_segmented_sieve(100_000_000, i);
            assert_eq!(5761455, primes.len(), "num_segments: {}", i);
            assert_eq!(
                279209790387276usize,
                primes.iter().sum(),
                "num_segments: {}",
                i
            );
        }
    }

    #[test]
    fn test_rayon_sieve() {
        // Test first few primes
        assert_eq!(PRIMES, &rayon_segmented_sieve(230, 12)[..]);

        // Big test
        for &i in [20, 1000, 10000, 20000].iter() {
            let primes = rayon_segmented_sieve(100_000_000, i);
            assert_eq!(5761455, primes.len(), "num_segments: {}", i);
            assert_eq!(
                279209790387276usize,
                primes.into_iter().sum(),
                "num_segments: {}",
                i
            );
        }
    }
}
