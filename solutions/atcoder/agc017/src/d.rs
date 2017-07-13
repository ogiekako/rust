use std::io;
use std::io::BufRead;

fn main() {
    let mut sc = Scanner::new(std::io::stdin());
    let n: usize = sc.next().unwrap();
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 1..n {
        let mut x: usize = sc.next().unwrap();
        let mut y: usize = sc.next().unwrap();
        x -= 1;
        y -= 1;
        graph[x].push(y);
        graph[y].push(x);
    }
    let res = recur(&graph, 0, n);
    println!("{}", if res == 0 { "Bob" } else { "Alice" });
}

fn recur(graph: &Vec<Vec<usize>>, i: usize, p: usize) -> usize {
    let mut res = 0;
    for j in graph[i].iter() {
        if *j != p {
            res ^= recur(graph, *j, i) + 1;
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
