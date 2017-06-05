extern crate contest;

use std::collections;
use contest::scanner;

fn main() {
    let mut sc = scanner::new(std::io::stdin());
    let t = sc.next().unwrap();
    for case in 0..t {
        let n: i32 = sc.next().unwrap();
        let mut s = collections::BTreeSet::new();
        for _ in 0..(2 * n - 1) * n {
            let v: i32 = sc.next().unwrap();
            if s.contains(&v) {
                s.remove(&v);
            } else {
                s.insert(v);
            }
        }
        print!("Case #{}:", case + 1);
        for i in s.into_iter() {
            print!(" {}", i);
        }
        println!("");
    }
}
