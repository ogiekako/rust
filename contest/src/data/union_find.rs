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
  pub fn new(n:usize) -> UF {
    UF{tree:vec![-1; n]}
  }
  /// Returns the root of the tree containing i.
  pub fn root(&mut self, i:usize) -> usize {
    if self.tree[i] < 0 {
      return i
    }
    let j = self.tree[i];
    let root = self.root(j as usize);
    // Path decomposition.
    self.tree[i] = root as isize;
    root
  }
  /// Returns the size of the tree containing i.
  #[inline]
  pub fn size(&mut self, i:usize) -> usize {
    let root = self.root(i);
    -self.tree[root] as usize
  }
  /// Unions i and j.
  pub fn union(&mut self, i:usize, j:usize) {
    let mut i = self.root(i);
    let mut j = self.root(j);
    if i == j { return; }
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
  use self::UF;
  #[test]
  fn it_works() {
    let mut uf = UF::new(4);
    uf.union(0,1);
    assert_eq!(2, uf.size(0));
    assert_eq!(2, uf.size(1));
    assert_eq!(1, uf.size(2));
    uf.union(2,3);
    uf.union(2,1);
    assert_eq!(4, uf.size(3));
  }
}
