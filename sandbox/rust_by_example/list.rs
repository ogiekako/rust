use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // try!: https://doc.rust-lang.org/beta/std/macro.try.html
    // write!: https://doc.rust-lang.org/beta/std/macro.write.html
    try!(write!(f, "["))
  }
}
