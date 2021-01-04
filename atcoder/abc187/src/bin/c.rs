use std::collections::BTreeSet;
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
    let mut x = BTreeSet::new();
    let mut y = BTreeSet::new();
    for _ in 0..n {
        let mut s: String = read();
        if s.starts_with("!") {
            s.remove(0);
            x.insert(s);
        } else {
            y.insert(s);
        }
    }
    for e in x.iter() {
        if y.contains(e) {
            println!("{}", e);
            return;
        }
    }
    println!("satisfiable");
}