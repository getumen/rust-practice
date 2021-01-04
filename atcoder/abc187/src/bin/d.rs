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
    let mut v = vec![];
    let mut x = 0;
    for _ in 0..n {
        let a: i64 = read();
        let b: i64 = read();
        x -= a;
        v.push(2 * a + b);
    }
    v.sort();
    let mut c = 0;
    while x <= 0 {
        x += v.pop().unwrap();
        c += 1;
    }
    println!("{}", c);
}
