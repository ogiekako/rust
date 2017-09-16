use std::io;
use std::io::BufRead;

fn answer(n: u64) {
    println!("! {}", n);
}


fn main() {
    let mut sc = Scanner::new(std::io::stdin());

    if favorite(&mut sc, 10_000_000_000) {
        let mut ans = 1;
        loop {
            if favorite(&mut sc, ans * 2) {
                answer(ans);
                return;
            }
            ans *= 10;
        }
    }

    let mut dig = 1;
    let mut ten = 1;
    loop {
        if !favorite(&mut sc, ten * 10) {
            break;
        }
        ten *= 10;
        dig += 1;
    }

    let mut head = 0;
    for i in 0..dig {
        let mut left: i32 = -1;
        let mut right: i32 = 10;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if favorite(&mut sc,
                        (head * 10 + mid as u64) * (if i + 1 == dig { 10 } else { 1 })) {
                if i + 1 < dig {
                    left = mid;
                } else {
                    right = mid;
                }
            } else {
                if i + 1 < dig {
                    right = mid;
                } else {
                    left = mid;
                }
            }
        }
        head = head * 10 + left as u64;
    }
    answer(head + 1);
}

fn favorite(sc: &mut Scanner<std::io::Stdin>, n: u64) -> bool {
    if n == 0 {
        return true;
    }
    println!("? {}", n);
    sc.next::<String>().unwrap() == "Y".to_string()
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
