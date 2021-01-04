fn main() {
    let s = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_end().to_owned()
    };
    let (n, w) = {
        let mut ws = s.split_whitespace();
        let n: i32 = ws.next().unwrap().parse().unwrap();
        let w: i32 = ws.next().unwrap().parse().unwrap();
        (n, w)
    };
    println!("{:?}", n / w);
}
