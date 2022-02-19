use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok().unwrap();

    let _n = buf.trim()
                .to_string()
                .parse::<i32>()
                .expect("parse error 1");

    buf.clear();
    io::stdin().read_line(&mut buf).ok().unwrap();

    let mut left: i32 = 0;
    let (ans, _) = buf.split_whitespace()
                      .map(|s| String::from(s).parse::<i32>().expect("parse error 2"))
                      .map(|num| cmp_substitute(&mut left, num))
                      .take_while(|&(_, right)| right)
                      .last()
                      .unwrap();

    println!("{}", ans);
}

fn cmp_substitute(left: &mut i32, num: i32) -> (i32, bool) {
    let ans = *left < num;
    *left = num;
    (num, ans)
}
