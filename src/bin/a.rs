use std::io;
use std::io::Read;

fn main() {
    let sum: i32 = io::stdin().bytes()
                         .map(|c| String::from(c.unwrap() as char))
                         .map(|s| s.parse::<i32>().ok().unwrap())
                         .sum();

    println!("{}", 100 * sum + 10 * sum + sum);
}
