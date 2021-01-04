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
    let h: i32 = read();
    let w: i32 = read();
    let mut min = std::i32::MAX;
    let mut count = 0;
    let mut sum = 0;
    for _ in 0..h {
        for _ in 0..w {
            let a = read();
            min = std::cmp::min(min, a);
            count += 1;
            sum += a;
        }
    }
    println!("{}", sum - min * count);
}
