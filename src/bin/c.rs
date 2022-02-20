use std::io;
use std::io::Read;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let n = read::<i32>();
    let q = read::<i32>();

    let mut a = HashMap::<i32, Vec<i32>>::new();

    (1..=n).into_iter()
           .map(|index| (read::<i32>(), index))
           .for_each(|(num, index)| a.entry(num).or_insert(Vec::<i32>::new()).push(index));

    (1..=q).into_iter()
           .map(|_| (read::<i32>(), read::<usize>()))
           .map(|(x, k)| search(k, a.get(&x)))
           .for_each(|ans| println!("{}", ans));
}

fn search(k: usize, indexes: Option<&Vec<i32>>) -> i32 {
    match indexes {
        Some(indexes) => {
            match indexes.get(k - 1) {
                Some(&index) => index,
                None => -1,
            }
        },

        None => -1,
    }
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
