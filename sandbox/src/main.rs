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

#[test]
fn let_test() {
  let x = 5;
  assert_eq!(x, 5);

  let (x, y) = (1, 2);
  assert_eq!((x,y), (1,2));

  let x: i32 = 5;
  assert_eq!(x, 5);

  let mut x = 5;
  assert_eq!(x, 5);
  x = 10;
  assert_eq!(x, 10);

  let x = 8;
  {
    let x = 12;
    assert_eq!(x, 12);
  }
  assert_eq!(x, 8);

  let mut x = 1;
  assert_eq!(x, 1);
  x = 7;
  let x = x;
  assert_eq!(x, 7);
}
