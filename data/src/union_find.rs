/// Implementation of disjoint set union algorithm using link-by-size and path
/// compression. It performs any intermixed sequence of m >= n find and n - 1
/// union operations in O(m α(m,n)) time where α is the inverse of the
/// Ackermann function [Tarjan-van Leeuwen 1984].
///
/// # Examples
///
/// ```
/// use data::union_find::UF;
/// let mut uf = UF::new(4);
/// uf.union(0,1);
/// assert_eq!(2, uf.size(0));
/// assert_eq!(uf.root(0), uf.root(1));
/// ```
pub struct UF {
    tree: Vec<isize>,
}

impl UF {
    /// Creates n disjoint singleton sets.
    pub fn new(n: usize) -> UF {
        UF { tree: vec![-1; n] }
    }
    /// Returns the root of the tree containing i.
    pub fn root(&mut self, i: usize) -> usize {
        if self.tree[i] < 0 {
            return i;
        }
        let j = self.tree[i];
        let root = self.root(j as usize);
        // Path decomposition.
        self.tree[i] = root as isize;
        root
    }
    /// Returns the size of the tree containing i.
    #[inline]
    pub fn size(&mut self, i: usize) -> usize {
        let root = self.root(i);
        -self.tree[root] as usize
    }
    /// Unions i and j.
    pub fn union(&mut self, i: usize, j: usize) {
        let mut i = self.root(i);
        let mut j = self.root(j);
        if i == j {
            return;
        }
        if -self.tree[i] > -self.tree[j] {
            let tmp = i;
            i = j;
            j = tmp;
        }
        // Append i to j since size(i) <= size(j).
        self.tree[j] += self.tree[i];
        self.tree[i] = j as isize;
    }
}

#[cfg(test)]
mod test {
    use super::UF;
    #[test]
    fn it_works() {
        let mut uf = UF::new(4);
        uf.union(0, 1);
        assert_eq!(2, uf.size(0));
        assert_eq!(2, uf.size(1));
        assert_eq!(1, uf.size(2));
        uf.union(2, 3);
        uf.union(2, 1);
        assert_eq!(4, uf.size(3));
    }
}

// http://codeforces.com/problemset/problem/722/C
fn main() {
    use std::io;
    use UF;
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();

    let n = words[0].parse::<usize>().unwrap();
    let mut a: Vec<i32> = vec![0; n];
    let mut p: Vec<usize> = vec![0; n];

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    for i in 0..n {
        a[i] = words[i].parse().unwrap()
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    for i in 0..n {
        p[i] = words[i].parse::<usize>().unwrap() - 1;
    }

    let mut uf = UF::new(n);
    let mut res: Vec<i64> = vec![0; n];
    let mut b: Vec<i64> = vec![-1; n];
    let mut max: i64 = 0;
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
        max = std::cmp::max(max, b[uf.root(p[i])]);
    }
    for i in 0..n {
        println!("{}", res[i]);
    }
}
