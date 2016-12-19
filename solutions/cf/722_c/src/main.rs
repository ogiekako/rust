extern crate lib;
// http://codeforces.com/problemset/problem/722/C
fn main() {
  use std::cmp;
  use std::io;
  use lib::data::union_find::UF;
  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  let words:Vec<&str> = line.split_whitespace().collect();

  let n = words[0].parse::<usize>().unwrap();
  let mut a:Vec<i32> = vec![0; n];
  let mut p:Vec<usize> = vec![0; n];

  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  let words:Vec<&str> = line.split_whitespace().collect();
  for i in 0..n {
    a[i] = words[i].parse().unwrap()
  }

  let mut line = String::new();
  io::stdin().read_line(&mut line).unwrap();
  let words:Vec<&str> = line.split_whitespace().collect();
  for i in 0..n {
    p[i] = words[i].parse::<usize>().unwrap() - 1;
  }

  let mut uf = UF::new(n);
  let mut res:Vec<i64> = vec![0; n];
  let mut b:Vec<i64> = vec![-1; n];
  let mut max:i64 = 0;
  for i in (0..n).rev() {
    res[i] = max;
    b[p[i]] = a[p[i]] as i64;
    let r1 = uf.root(p[i]);
    if p[i] + 1 < n && b[p[i] + 1] >= 0 {
      let r2 = uf.root(p[i] + 1);
      uf.union(r1, r2);
      if r1 == uf.root(r1) {
        b[r1] += b[r2];
      } else {
        b[r2] += b[r1];
      }
    }

    let r1 = uf.root(p[i]);
    if p[i] > 0 && b[p[i] - 1] >= 0 {
      let r2 = uf.root(p[i] - 1);
      uf.union(r1, r2);
      if r1 == uf.root(r1) {
        b[r1] += b[r2];
      } else {
        b[r2] += b[r1];
      }
    }
    max = cmp::max(max, b[uf.root(p[i])]);
  }
  for i in 0..n {
    println!("{}", res[i]);
  }
}
