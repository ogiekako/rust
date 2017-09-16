use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let d:i64 = sc.next();
    let mut res:i64 = 0;
    for len in 2..19 {
        res += solve(len, d);
    }
    println!("{}", res);
}

fn solve(len: i32, d: i64) -> i64 {
    let half = len / 2;
    let tens:Vec<i64> = (0..len).map({|i|(10u64).pow(i as u32) as i64}).collect();
    let threes:Vec<i64> = (0..half+1).map({|i|3u64.pow(i as u32) as i64}).collect();
    let cap = threes[half as usize];
    let mut res = 0;

    for mask in 0..cap {
        let mut mask = mask;
        let mut val:i64 = d;
        let mut ways:i64 = 1;
        if len % 2 == 1 {
            ways *= 10;
        }
        for i in 0..half {
            let u = mask % 3;
            mask /= 3;
            let j = (len - 1 - i) as usize;
            let i = i as usize;

            let mut v = val / tens[i] % 10;
            if v < 0 {
                v += 10
            }
            if u == 0 { // -
                if v == 0 {
                    ways = 0;
                } else {
                    val -= (v - 10) * tens[i];
                    val -= (10 - v) * tens[j];

                    if i == 0 {
                        ways *= v - 1;
                    } else {
                        ways *= v;
                    }
                }
            } else if u == 1 { // 0
                if v != 0 {
                    ways = 0;
                } else {
                    if i == 0 {
                        ways *= 9;
                    } else {
                        ways *= 10;
                    }
                }
            } else if u == 2 { // +
                if v == 0 {
                    ways = 0;
                } else {
                    val -= v * tens[i];
                    val -= -v * tens[j];

                    ways *= 10 - v;
                }
            }

            if ways > 0 && val / tens[i] % 10 != 0 {
                panic!("!");
            }
        }
        if val == 0 {
            res += ways;
        }
    }
    res
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
