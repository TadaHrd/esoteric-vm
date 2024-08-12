//! A list of primes and semiprimes that are also fibonacci numbers.
//!
//! Determined with :
//! ```rust,ingore
//!     use std::collections::HashSet;
//!     use std::io::Write;
//!     // PRIMES
//!
//!     let mut primes = vec![true; 65536];
//!     for i in 2..=256 {
//!         if primes[i] {
//!             let mut j = i * i;
//!             while j < 65536 {
//!                 primes[j] = false;
//!                 j += i;
//!             }
//!         }
//!     }
//!     primes[0] = false;
//!
//!     let mut prime_nums = Vec::new();
//!
//!     for (i, &v) in primes.iter().enumerate() {
//!         if v {
//!             prime_nums.push(i);
//!         }
//!     }
//!
//!     // SEMIPRIMES AND PRIMES
//!
//!     let mut semiprimes_set = HashSet::new();
//!
//!     for i in 0..prime_nums.len() {
//!         for j in i..prime_nums.len() {
//!             let i = prime_nums[i];
//!             let j = prime_nums[j];
//!
//!             semiprimes_set.insert(i * j);
//!         }
//!     }
//!
//!     // FIBONACCI
//!
//!     let mut fibonacci_nums = Vec::new();
//!
//!     let mut a = 0;
//!     let mut b = 1;
//!
//!     while b < 65536 {
//!         fibonacci_nums.push(b);
//!
//!         let c = a + b;
//!         a = b;
//!         b = c;
//!     }
//!
//!     // FIBONACCI PRIMES AND SEMIPRIMES
//!
//!     let mut fibonacci_semiprimes = HashSet::new();
//!
//!     for i in fibonacci_nums.iter() {
//!         if semiprimes_set.contains(i) {
//!             fibonacci_semiprimes.insert(*i);
//!         }
//!     }
//!
//!     // Sort
//!
//!     let mut fibonacci_semiprimes_list: Vec<usize> = fibonacci_semiprimes.iter().copied().collect();
//!     fibonacci_semiprimes_list.sort();
//!
//!     // Construct a function's code
//!     let mut fn_code = "matches!(n, ".to_string();
//!
//!     for n in &fibonacci_semiprimes_list {
//!         fn_code.push_str(&format!("{n} | "));
//!     }
//!
//!     // Pop 3 characters (` | `)
//!     fn_code.pop();
//!     fn_code.pop();
//!     fn_code.pop();
//!
//!     fn_code.push(')');
//!
//!     // Write down here
//!     let mut file = std::fs::OpenOptions::new()
//!         .append(true)
//!         .open(file!())
//!         .expect("couldn't open its own path as a file");
//!
//!     println!(
//!         "\n// CODE:\n
//! /// A sorted list of [`u16`] primes and semprimes that are also fibonacci numbers.
//! ///
//! /// The list of numbers is as follows: 1, 2, 3, 5, 13, 21, 34, 55, 89, 233, 377, 1597, 4181, 17711, 28657
//! pub const FIB_PRIME_AND_SEMIPRIME_LIST_U16: [u16; {len}] = {:?};
//! /// Checks if a [`u16`] is a prime or semiprime and a fibonacci number.
//! ///
//! /// The list of numbers is as follows: 1, 2, 3, 5, 13, 21, 34, 55, 89, 233, 377, 1597, 4181, 17711, 28657
//! pub const fn is_fib_prime_or_semiprime_u16(n: u16) -> bool {{
//!     {fn_code}
//! }}
//! ",
//!         fibonacci_semiprimes_list,
//!         len = fibonacci_semiprimes_list.len()
//!     )
//! ```

/// A sorted list of [`u16`] primes and semprimes that are also fibonacci numbers.
///
/// The list of numbers is as follows: 1, 2, 3, 5, 13, 21, 34, 55, 89, 233, 377, 1597, 4181, 17711, 28657
pub const _FIB_PRIME_AND_SEMIPRIME_LIST_U16: [u16; 15] = [
    1, 2, 3, 5, 13, 21, 34, 55, 89, 233, 377, 1597, 4181, 17711, 28657,
];
/// Checks if a [`u16`] is a prime or semiprime and a fibonacci number.
///
/// The list of numbers is as follows: 1, 2, 3, 5, 13, 21, 34, 55, 89, 233, 377, 1597, 4181, 17711, 28657
pub const fn is_fib_prime_or_semiprime_u16(n: u16) -> bool {
    matches!(
        n,
        1 | 2 | 3 | 5 | 13 | 21 | 34 | 55 | 89 | 233 | 377 | 1597 | 4181 | 17711 | 28657
    )
}
