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
    let n: i32 = read();
    let mut c = 0;
    for i in 1..n + 1 {
        let o = format!("{:o}", i);
        let d = format!("{}", i);
        if !o.contains("7") && !d.contains("7") {
            c += 1;
        }
    }
    println!("{}", c);
}
