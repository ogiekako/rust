
pub fn next_permutation(is: &mut Vec<i32>) -> bool {
    let n = is.len();
    for i in (1..n).rev() {
        let i = i as usize;
        if is[i - 1] < is[i] {
            let mut j = n - 1;
            while is[i - 1] >= is[j] {
                j -= 1;
            }
            is.swap(i - 1, j);
            reverse(is, i, n);
            return true;
        }
    }
    is.reverse();
    false
}

pub fn reverse(v: &mut Vec<i32>, from_inclusive: usize, to_exclusive: usize) {
    let mut i = from_inclusive;
    let mut j = to_exclusive - 1;
    while i < j {
        v.swap(i, j);
        i += 1;
        j -= 1;
    }
}
