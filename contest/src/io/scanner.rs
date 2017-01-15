/// Scanner
///
/// # Example
/// ```
/// use contest::io::scanner;
/// let mut sc = scanner::Scanner::new();
/// let n:i32 = sc.next();
/// ```
use std;

pub struct Scanner {
    token_buffer : Vec<String>,
    index : usize,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { token_buffer: vec![], index: 0 }
    }

    fn wrapped<T>(& mut self) -> Result<T,&str> where T: std::str::FromStr {
        let s = try!(self.fetch_token());
        let t = try!(s.parse::<T>().map_err(|_| "Parse error"));
        Ok(t)
    }

    pub fn next<T>(& mut self) -> T where T: std::str::FromStr {
        self.wrapped::<T>().expect("Fail to get next: ")
    }

    fn fetch_token(&mut self) -> Result<&String,&str> {
        while self.index >= self.token_buffer.len() {
            let mut st = String::new();
            while st.trim() == "" {
                match std::io::stdin().read_line(&mut st) {
                    Ok(l) if l > 0 => continue,
                    Ok(_)  => return Err("End of file"),
                    Err(_) => return Err("Failed to read line"),
                }
            }
            self.token_buffer = st.split_whitespace()
                .map(|x| x.to_string())
                .collect();
            self.index = 0;
        }

        self.index += 1;
        Ok(&self.token_buffer[self.index - 1])
    }
}
