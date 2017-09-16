fn main() {
    let mut sc = new(std::io::stdin());
    let a:usize = sc.next();
    let b:usize = sc.next();
    let c:usize = sc.next();
    let d:usize = sc.next();
    let e:usize = sc.next();
    let f:usize = sc.next();

    let mut water = vec![false; 3010];
    for i in 0..3010 {
        for j in 0..3010 {
            let v = i * a * 100 + j * b * 100;
            if v < 3010 {
                water[v] = true;
            }
        }
    }
    let mut sugar = vec![false; 3010];
    for i in 0..3010 {
        for j in 0..3010 {
            let v = i * c + j * d;
            if v < 3010 {
                sugar[v] = true;
            }
        }
    }
    let mut best = 0.0;
    let mut bestTot = 0;
    let mut bestSugar = 0;
    for i in 0..3010 {
        for j in 0..3010 {
            if (i > 0 || j > 0) && water[i] && sugar[j] {
                if i + j <= f && (100 + e) * j <= e * (i + j) {
                    let d = j as f64 / (i + j) as f64;
                    if d >= best {
                        best = d;
                        bestTot = i + j;
                        bestSugar = j;
                    }
                }
            }
        }
    }
    println!("{} {}", bestTot, bestSugar);
}

use std::io;
use std::io::BufRead;

pub struct Scanner<R: io::Read> {
    br: io::BufReader<R>,
    // Read tokens are stored in reversed order per line.
    buf: Vec<String>,
}

pub fn new<R: io::Read>(r: R) -> Scanner<R> {
    Scanner::new(r)
}

impl<R: io::Read> Scanner<R> {
    #[inline]
    fn new(r: R) -> Scanner<R> {
        Scanner {
            br: io::BufReader::new(r),
            buf: vec![],
        }
    }
    #[inline]
    pub fn next<T>(&mut self) -> T
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug
    {
        self.next_string().map(|s| s.parse::<T>().expect("Parse failed: ")).unwrap()
    }
    fn next_string(&mut self) -> Option<String> {
        self.buf.pop().or_else(|| match self.update() {
            true => self.next_string(),
            false => None,
        })
    }
    #[inline]
    fn update(&mut self) -> bool {
        let mut s = String::new();
        let res = self.br.read_line(&mut s);
        match res.expect("I/O error.") {
            0 => false,
            _ => {
                self.buf = s.split_whitespace().map(|x| x.to_string()).rev().collect();
                true
            }
        }
    }
}
