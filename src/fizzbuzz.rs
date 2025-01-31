//! Fizz buzz challenge
// Write a program that prints the numbers from 1 to n, each on a separate line.
// If the number is a multiple of 3, print "Fizz" instead.
// If the number is a multiple of 5, print "Buzz" instead.
// If the number is a multiple of 15, print "FizzBuzz" instead.
// Else, print the number.

use std::{
    fmt::Display,
    ops::{Add, Rem},
};

/// A number type that can be provided to `fizz_buzz` function,
/// `Rem` - to support module operation
/// `From<u8> + Copy` - to be able to have explicit values 0, 1, 3, 5 and implicit copy
trait FizzBuzzable:
    Add<Output = Self> + Rem<Output = Self> + PartialOrd + From<u8> + Display + Copy
{
}

impl FizzBuzzable for u8 {}
impl FizzBuzzable for u16 {}
impl FizzBuzzable for u32 {}
impl FizzBuzzable for u64 {}
impl FizzBuzzable for u128 {}

/// Inner function used by `fizz_buzz`, designed this way to test output strings.
/// Works for every type that implements the `FizzBuzzable` traits
fn fizz_buzz_inner<T: FizzBuzzable>(n: T) -> impl Iterator<Item = String> {
    let zero: T = 0.into();
    let mut i: T = zero;

    let one: T = 1.into();

    std::iter::from_fn(move || {
        if i == n {
            return None;
        }
        i = i + one;

        if i % 3.into() == zero && i % 5.into() == zero {
            Some("FizzBuzz".to_string())
        } else if i % 3.into() == zero {
            Some("Fizz".to_string())
        } else if i % 5.into() == zero {
            Some("Buzz".to_string())
        } else {
            Some(i.to_string())
        }
    })
}

/// Run inner function and print each value from iterator
fn fizz_buzz<T: FizzBuzzable>(n: T) {
    let _ = fizz_buzz_inner(n)
        .map(|val| println!("{val}"))
        .collect::<Vec<()>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    // This won't print but verifies it's working
    #[test]
    fn fizz_buzz_inner_works() {
        let expected_values = [
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz", "16", "17", "Fizz", "19", "Buzz", "Fizz", "22", "23", "Fizz", "Buzz",
            "26", "Fizz", "28", "29", "FizzBuzz", "31",
        ]
        .map(|s| s.to_string());
        let values = fizz_buzz_inner::<u128>(31).collect::<Vec<_>>();
        assert_eq!(expected_values.to_vec(), values);
    }

    // Note: this will print
    #[test]
    fn fizz_buzz_more() {
        fizz_buzz(120_u8);
        fizz_buzz::<u16>(449);
        fizz_buzz(300_u64);
        fizz_buzz::<u128>(1234);
    }
}
