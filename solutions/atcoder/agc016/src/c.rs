use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let H: usize = sc.next().unwrap();
    let W: usize = sc.next().unwrap();
    let h: usize = sc.next().unwrap();
    let w: usize = sc.next().unwrap();
    let res = f(H, W, h, w);
    if let Some(res) = res {
        println!("Yes");
        for i in 0..H {
            for j in 0..W {
                if j > 0 {
                    print!(" ");
                }
                print!("{}", res[i][j]);
            }
            println!();
        }
        return;
    }
    let res = f(W, H, w, h);
    if let Some(res) = res {
        println!("Yes");
        for i in 0..H {
            for j in 0..W {
                if j > 0 {
                    print!(" ");
                }
                print!("{}", res[j][i]);
            }
            println!();
        }
        return;
    }
    println!("No");
}

const B: i32 = 100000;

fn f(H: usize, W: usize, h: usize, w: usize) -> Option<Vec<Vec<i32>>> {
    if H % h == 0 {
        return None;
    }
    let d = H % h;
    let mut res = vec![];
    for i in 0..H {
        if i % h == d {
            res.push(vec![-B * (h as i32- 1) - 1; W]);
        } else {
            res.push(vec![B; W]);
        }
    }
    Some(res)
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