use std::hint::black_box;
use std::time::Instant;

fn mod_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result.wrapping_mul(base) % modulus;
        }
        base = base.wrapping_mul(base) % modulus;
        exponent /= 2;
    }
    result
}

/// Works for numbers up to 341,550,071,728,321
/// https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test#Testing_against_small_sets_of_bases
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }

    let nums_to_check: &[u64] = if n < 2_047 {
        &[2]
    } else if n < 1_373_653 {
        &[2, 3]
    } else if n < 9_080_191 {
        &[31, 73]
    } else if n < 25_326_001 {
        &[2, 3, 5]
    } else if n < 3_215_031_751 {
        &[2, 3, 5, 7]
    } else if n < 4_759_123_141 {
        &[2, 7, 61]
    } else if n < 1_122_004_669_633 {
        &[2, 13, 23, 1662803]
    } else if n < 2_152_302_898_747 {
        &[2, 3, 5, 7, 11]
    } else if n < 3_474_749_660_383 {
        &[2, 3, 5, 7, 11, 13]
    } else if n < 341_550_071_728_321 {
        &[2, 3, 5, 7, 11, 13, 17]
    } else if n < 3_825_123_056_546_413_051 {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23]
    } else {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    };

    // Write n-1 as d*2^k
    let mut d = n - 1;
    let mut k = 0;
    while d.is_multiple_of(2) {
        d /= 2;
        k += 1;
    }

    for &base in nums_to_check.iter() {
        if base >= n {
            continue;
        }

        let mut x = mod_pow(base, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }

        let mut passed = false;
        for _ in 0..(k - 1) {
            x = mod_pow(x, 2, n);
            if x == n - 1 {
                passed = true;
                break;
            }
        }

        if !passed {
            return false;
        }
    }

    true
}

fn main() {
    let n = black_box(999966000289u64);
    let iterations = 100;

    // Warm up
    for _ in 0..10 {
        black_box(is_prime(n));
    }

    let start = Instant::now();

    for _ in 0..iterations {
        black_box(is_prime(n));
    }

    let elapsed = start.elapsed();
    println!(
        "Elapsed time for {} iterations: {:.6?}",
        iterations, elapsed
    );
}
