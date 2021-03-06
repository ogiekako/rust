extern crate contest;

use std::collections;
use contest::scanner;

fn main() {
    let mut sc = scanner::new(std::io::stdin());
    let n = sc.next();
    for case in 0..n {
        let mut res = collections::VecDeque::new();

        let s: String = sc.next();
        for c in s.chars() {
            if res.len() == 0 || res[0] > c {
                res.push_back(c);
            } else {
                res.push_front(c);
            }
        }
        let ss: String = res.into_iter().collect();
        println!("Case #{}: {}", case + 1, ss);
    }
}
