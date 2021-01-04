extern crate num;
extern crate num_integer;

use num_integer::Integer;
use std::io::Read;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn extended_gcd<T: Copy + Integer>(a: T, b: T) -> (T, T, T) {
    if a == T::zero() {
        return (a, T::zero(), T::one());
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        return (g, y - (b / a) * x, x);
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (_, x, _) = extended_gcd(a, m);
    (x % m + m) % m
}

#[test]
fn another() {
    let (_, x, y) = extended_gcd(111, 30);
    assert_eq!(x, 3);
    assert_eq!(y, -11);
}

fn main() {
    let t: usize = read();
    for _ in 0..t {
        let n: i64 = read();
        let s: i64 = read();
        let k: i64 = read();

        let d = num::integer::gcd(num::integer::gcd(k, s), n);
        let n = n / d;
        let s = s / d;
        let k = k / d;

        if num::integer::gcd(k, n) != 1 {
            println!("-1");
            continue;
        }
        let k_inv = mod_inverse(k, n);
        println!("{}", (n - (k_inv * s) % n) % n);
    }
}
