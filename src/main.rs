use num_bigint::BigInt;
use num_traits::identities::{Zero, One};
use std::cmp;
use std::thread;

#[derive(Debug, Clone)]
struct FibPair {
    fn_: BigInt,
    fn_plus_1: BigInt,
}

fn fib_fast_doubling(n: u64) -> FibPair {
    if n == 0 {
        return FibPair {
            fn_: BigInt::zero(),
            fn_plus_1: BigInt::one(),
        };
    }

    let k = n / 2;
    let a = fib_fast_doubling(k);

    let c = &a.fn_ * (&a.fn_plus_1 * 2 - &a.fn_);
    let d = &a.fn_ * &a.fn_ + &a.fn_plus_1 * &a.fn_plus_1;

    if n % 2 == 0 {
        FibPair { fn_: c, fn_plus_1: d }
    } else {
        FibPair { fn_: d.clone(), fn_plus_1: &c + &d }
    }
}

fn fib_parallel(n: u64, depth: u32, max_depth: u32) -> FibPair {
    if n == 0 {
        return FibPair {
            fn_: BigInt::zero(),
            fn_plus_1: BigInt::one(),
        };
    }

    let k = n / 2;

    if depth < cmp::min(max_depth, 12) {
        let handle = thread::spawn(move || fib_parallel(k, depth + 1, max_depth));
        let a = handle.join().unwrap();

        let c = &a.fn_ * (&a.fn_plus_1 * 2 - &a.fn_);
        let d = &a.fn_ * &a.fn_ + &a.fn_plus_1 * &a.fn_plus_1;

        if n % 2 == 0 {
            FibPair { fn_: c, fn_plus_1: d }
        } else {
            FibPair { fn_: d.clone(), fn_plus_1: &c + &d }
        }
    } else {
        fib_fast_doubling(n)
    }
}

// Helper function to calculate log10 of a BigInt
fn log10_bigint(num: &BigInt) -> u64 {
    let mut n = num.clone();
    let mut log = 0;
    
    while n >= 10.into() {
        n /= 10;
        log += 1;
    }
    
    log
}

// Updated format_number function
fn format_number(num: &BigInt) -> String {
    let num_str = num.to_string();
    let len = num_str.len();

    // Calculate log base 10 to estimate the scientific notation
    let log10 = log10_bigint(num);

    // Estimate the approximate scientific notation
    let exponent = log10;
    let mut mantissa = num.clone();
    while mantissa >= 10.into() {
        mantissa /= 10;
    }

    // Generate scientific notation like "1.23e+456"
    let scientific_notation = format!("{:.1}e+{}", mantissa, exponent);

    format!("Digits: {}, Approximate Scientific Notation: {}", len, scientific_notation)
}

fn main() {
    let n: u64 = 4_000_000;
    let max_depth: u32 = 12;
    let start = std::time::Instant::now();

    let result = fib_parallel(n, 0, max_depth);

    let elapsed = start.elapsed();
    let formatted_result = format_number(&result.fn_);
    println!("Fibonacci number F({}) is approximately: {}", n, formatted_result);
    println!("Time taken: {:.5?}", elapsed);
}