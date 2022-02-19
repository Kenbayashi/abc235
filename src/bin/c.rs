use std::io;
use std::io::Read;
use std::str::FromStr;

fn main() {
    let n = read::<usize>();
    let q = read::<usize>();

    let a = (1..=n).into_iter()
                   .map(|_| read::<i32>())
                   .enumerate()
                   .collect::<Vec<(usize, i32)>>();

    let xk = (1..=q).into_iter()
                    .map(|_| (read::<i32>(), read::<i32>()))
                    .collect::<Vec<(i32, i32)>>();

    xk.into_iter()
      .map(|(x, k)| get_index(x, k, &a))
      .for_each(|index| println!("{}", if 0 <= index {index + 1} else {index}));
}

fn get_index(x: i32, mut k: i32, a: &Vec<(usize, i32)>) -> i32 {
    let mut ans = -1;

    for &(index, num) in a {
        if x == num {
            k -= 1;

            if k == 0 {
                ans = index as i32;
            }
        }
    }
    
    ans
}

fn read<T: FromStr>() -> T {
    io::stdin().bytes()
               .map(|c| c.unwrap() as char)
               .skip_while(|c| c.is_whitespace())
               .take_while(|c| !c.is_whitespace())
               .collect::<String>()
               .parse::<T>()
               .ok()
               .unwrap()
}
