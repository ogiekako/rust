use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next();

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n - 1 {
        let a: usize = sc.next();
        let b: usize = sc.next();
        let a = a - 1;
        let b = b - 1;
        g[a].push(b);
        g[b].push(a);
    }

    let mut d1 = vec![0; n];
    let mut d2 = vec![0; n];

    f(&g, &mut d1, 0, n, 0);
    f(&g, &mut d2, n - 1, n, 0);

    let mut a = 0;
    for i in 0..n {
        if d1[i] <= d2[i] {
            a += 1;
        }
    }
    println!("{}", if a > n - a { "Fennec" } else { "Snuke" });
}

fn f(g: &Vec<Vec<usize>>, ds: &mut Vec<usize>, i: usize, p: usize, d: usize) {
    ds[i] = d;
    for j in g[i].iter() {
        if *j != p {
            f(g, ds, *j, i, d + 1);
        }
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
