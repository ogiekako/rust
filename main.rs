use std::ops::Add;

struct P(f64, f64);

struct C {
  o: P,
  r: f64,
}

impl Add for P {
  type Output = P;

  fn add(self, o: P) -> P {
    P(self.0 + o.0, self.1 + o.1)
  }
}

impl Add<P> for C {
  type Output = C;

  fn add(self, o: P) -> C {
    C { o: self.o + o, r: self.r }
  }
}

fn main() {
  let p = P(1.0, 2.0);
  let c = C { o: P(2.0, 3.0), r: 5.0};
  let c2 = c + p;

  println!("{} {} {}", c2.o.0, c2.o.1, c2.r);
}
