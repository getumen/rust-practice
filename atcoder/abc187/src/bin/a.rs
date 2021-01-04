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
 
fn f(x: i32) -> i32 {
    x / 100 + (x % 100) / 10 + x % 10
}
 
fn main() {
    let a: i32 = read();
    let b: i32 = read();
    let a_f = f(a);
    let b_f = f(b);
 
    if a_f > b_f {
        println!("{}", a_f)
    } else {
        println!("{}", b_f)
    }
}