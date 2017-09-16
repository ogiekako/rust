use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next();
    let a: i64 = sc.next();
    let b: i64 = sc.next();
    let c: i64 = sc.next();
    let d: i64 = sc.next();
    for i in 0..n {
        let i = i as i64;
        let j = n as i64 - 1 - i;
        let l1 = a + i * c;
        let r1 = a + i * d;
        let l2 = b + j * c;
        let r2 = b + j * d;
        let l = std::cmp::max(l1, l2);
        let r = std::cmp::min(r1, r2);
        if l <= r {
            println!("YES");
            return;
        }
    }
    println!("NO");
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
