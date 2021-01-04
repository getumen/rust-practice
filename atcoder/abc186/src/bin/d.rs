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

fn main() {
    let n: usize = read();
    let mut a = vec![];
    for _ in 0..n {
        let x: i64 = read();
        a.push(x);
    }
    a.sort();

    let mut cum = vec![];
    for i in 0..n {
        if i == 0 {
            cum.push(a[i])
        } else {
            cum.push(cum[i - 1] + a[i]);
        }
    }

    let mut result = 0;
    for i in 0..n {
        result += cum[n - 1] - cum[i] - a[i] * (n - i - 1) as i64;
    }
    println!("{}", result);
}
