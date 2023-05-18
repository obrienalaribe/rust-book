/// Fibonacci via Dynamic Programming
use std::collections::HashMap;

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn fibonacci(n: u32) -> u128 {
    // Use a and b to store the previous two values in the sequence
    let mut a = 0;
    let mut b = 1;
    for _i in 0..n {
        // As we iterate through, move b's value into a and the new computed
        // value into b.
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn recursive_fibonacci(n: u32) -> u128 {
    // Call the actual tail recursive implementation, with the extra
    // arguments set up.
    _recursive_fibonacci(n, 0, 1)
}

fn _recursive_fibonacci(n: u32, previous: u128, current: u128) -> u128 {
    if n == 0 {
        current
    } else {
        _recursive_fibonacci(n - 1, current, current + previous)
    }
}

/// classical_fibonacci(n) returns the nth fibonacci number
/// This function uses the definition of Fibonacci where:
/// F(0) = 0, F(1) = 1 and F(n+1) = F(n) + F(n-1) for n>0
///
/// Warning: This will overflow the 128-bit unsigned integer at n=186
pub fn classical_fibonacci(n: u32) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let k = n / 2;
            let f1 = classical_fibonacci(k);
            let f2 = classical_fibonacci(k - 1);

            match n % 4 {
                0 | 2 => f1 * (f1 + 2 * f2),
                1 => (2 * f1 + f2) * (2 * f1 - f2) + 2,
                _ => (2 * f1 + f2) * (2 * f1 - f2) - 2,
            }
        }
    }
}