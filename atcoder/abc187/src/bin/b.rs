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
    for _ in 0..n {
        let x: f32 = read();
        let y: f32 = read();
        v.push((x, y));
    }
    let mut c = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let g = (v[j].1 - v[i].1) / (v[j].0 - v[i].0);
            if -1f32 <= g && g <= 1f32 {
                c += 1;
            }
        }
    }
    println!("{}", c);
}