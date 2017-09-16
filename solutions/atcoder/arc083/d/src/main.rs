fn main() {
    let mut sc = new(std::io::stdin());
    let n: usize = sc.next();
    let mut g = vec![vec![(0 as usize, false); n]; n];
    for i in 0..n {
        for j in 0..n {
            g[i][j] = (sc.next(), false);
        }
    }
    let mut ng = false;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let d = g[i][k].0 + g[k][j].0;
                if d < g[i][j].0 {
                    ng = true;
                }
                if i != k && k != j && d == g[i][j].0 {
                    g[i][j].1 = true;
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..n {
            if !g[i][j].1 {
                res += g[i][j].0;
            }
        }
    }
    if ng {
        println!("-1");
    } else {
        println!("{}", res / 2);
    }
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
