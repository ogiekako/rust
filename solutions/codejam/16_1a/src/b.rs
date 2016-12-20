extern crate contest;

use std::collections;

fn main() {
    use contest::io::scanner;
    let mut sc = scanner::Scanner::new();
    let t = sc.next();
    for case in 0..t {
        let n:i32 = sc.next();
        let mut s = collections::BTreeSet::new();
        for _ in 0..(2*n-1)*n {
            let v:i32 = sc.next();
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
