use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next().unwrap();
    let m: usize = sc.next().unwrap();
    let mut xy: Vec<(usize, usize)> = vec![];
    for _ in 0..m {
        let x: usize = sc.next().unwrap();
        let y: usize = sc.next().unwrap();
        xy.push((x - 1, y - 1));
    }
    let mut bads = vec![false; n];
    let mut bs = vec![];
    for i in 0..n {
        let mut b = vec![-1; n];
        b[i] = m as i32;
        let mut bad = false;
        for i in (0..m).rev() {
            let (x, y) = xy[i];
            if b[x] >= 0 && b[y] >= 0 {
                bad = true;
            }
            if b[x] >= 0 {
                b[y] = i as i32;
            } else if b[y] >= 0 {
                b[x] = i as i32;
            }
        }
        for i in 0..m {
            let (x, y) = xy[i];
            if b[x] >= 0 && b[x] < i as i32 && b[y] == -1 {
                b[y] = i as i32;
            } else if b[y] >= 0 && b[y] < i as i32 && b[x] == -1 {
                b[x] = i as i32;
            }
        }
        bs.push(b);
        bads[i] = bad;
    }
    let mut res = 0;
    for i in 0..n {
        for j in 0..i {
            if !bads[i] && !bads[j] {
                if bs[i][j] == -1 && bs[j][i] == -1 {
                    res += 1;
                }
            }
        }
    }
    println!("{}", res);
}

/// Scanner
///
/// # Example
/// ```
/// use contest::io::scanner;
/// let mut sc = scanner::Scanner::new("1 2 \n\n \r\t \n 3.5".as_bytes());
/// assert_eq!("1".to_string(), sc.next::<String>().unwrap());
/// assert_eq!(2, sc.next().unwrap());
/// assert_eq!(3.5, sc.next().unwrap());
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
