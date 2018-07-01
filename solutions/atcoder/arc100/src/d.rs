use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next().unwrap();

    let mut a: Vec<i64> = vec![];
    for i in 0..n {
        a.push(sc.next().unwrap());
    }
    let mut s: Vec<i64> = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }
    let mut res = 1_000_000_000_000_000_000;
    for i in 2..(n-1) {
        let i2 = f(&s, 0, i);
        let i3 = f(&s, i, n);
        // println!("{} {} {}", i, i2, i3);
        for j in i2..(i2+2) {
            for k in i3..(i3+2) {
                if 0 < j && j < i && i < k && k < n {
                    let mut v = vec![s[j]-s[0], s[i]-s[j], s[k]-s[i], s[n]-s[k]];
                    v.sort();
                    res = std::cmp::min(res, v[3] - v[0]);
                }
            }
        }
    }

    println!("{}", res);
}

fn f(s: &Vec<i64>, l: usize, r: usize) -> usize {
    let mut ll = l + 1;
    let mut rr = r;
    while rr - ll > 1 {
        let m = (ll + rr) / 2;
        let sl = s[m] - s[l];
        let sr = s[r] - s[m];
        if sl > sr {
            rr = m;
        } else {
            ll = m;
        }
    }
    ll
}

/// Scanner
///
/// # Example
/// ```
/// use contest::io::scanner;
/// let mut sc = scanner::Scanner::new("1 2 \n\n \r\t \n 3.5".as_bytes());
/// assert_eq!("1".to_string(), sc.next::<String>().unwrap());
/// assert_eq!(2, sc.next());
/// assert_eq!(3.5, sc.next());
/// assert_eq!(None, sc.next::<i32>());
/// ```

pub struct Scanner<R: io::Read> {
    br: io::BufReader<R>,
    // Read tokens are stored in reversed order per line.
    buf: Vec<String>,
}

impl<R: io::Read> Scanner<R> {
    pub fn new(r: R) -> Scanner<R> {
        Scanner {
            br: io::BufReader::new(r),
            buf: vec![],
        }
    }
    #[inline]
    pub fn next<T>(&mut self) -> Option<T>
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug
    {
        self.next_string().map(|s| s.parse::<T>().expect("Parse failed: "))
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
