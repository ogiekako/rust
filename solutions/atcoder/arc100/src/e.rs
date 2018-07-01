use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next().unwrap();

    let mut a: Vec<i64> = vec![0; 1 << n];
    for i in 0..(1<<n) {
        a[i] = sc.next().unwrap();
    }
    let neg = 0usize.wrapping_sub(1);
    let mut dp: Vec<(usize, usize)> = vec![(neg, neg); 1<<n];
    dp[0] = (0, neg); // first, second
    for i in 1..(1<<n) {
        dp[i].0 = i;
        for j in 0..n {
            if (i & (1<<j)) > 0 {
                let i2 = i ^ (1<<j);
                for j in vec![dp[i2].0, dp[i2].1] {
                    if j == dp[i].0 || j == dp[i].1 || j == neg {
                        continue;
                    }
                    if a[j] > a[dp[i].0] {
                        dp[i].1 = dp[i].0;
                        dp[i].0 = j;
                    } else if dp[i].1 == neg || a[j] > a[dp[i].1] {
                        dp[i].1 = j;
                    }
                }
            }
        }
    }
    for k in 1..(1<<n) {
        let mut res = a[dp[k].0] + a[dp[k].1];
        for i in (0..n).rev() {
            let i2 = 1 << i;
            if k & i2 > 0 {
                let i3 = (k & (((1<<n) - 1) ^ (i2 - 1))) - 1;
                if i3 == 0 {
                    continue;
                }
                let v = a[dp[i3].0] + a[dp[i3].1];
                res = std::cmp::max(res, v);
            }
        }
        println!("{}", res);
    }
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
