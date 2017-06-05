extern crate contest;

use contest::scanner;
use contest::math;

fn main() {
    let mut sc = scanner::new(std::io::stdin());
    let t = sc.next().unwrap();
    for test_case in 0..t {
        let n = sc.next().unwrap();
        let m = sc.next().unwrap();
        let mut rs = vec![0; n as usize];
        for i in 0..n {
            rs[i as usize] = sc.next().unwrap();
        }
        println!("Case #{}: {}", test_case + 1, solve(n, m, rs));
    }
}

static MOD: i64 = 1_000_000_007;

fn solve(n: i64, m: i64, rs: Vec<i64>) -> i64 {
    if n == 1 {
        return m;
    }
    let mut sum = 0;
    for i in 0..n {
        sum += rs[i as usize];
    }
    sum *= 2;
    let mut inv = vec![0; (n + 1) as usize];
    for i in 1..(n + 1) {
        inv[i as usize] = math::inv(i as i32, MOD as i32) as i64;
    }
    let mut a = vec![0; 4001];
    for s in 0..4001 {
        let n2 = n + m - sum - 1 + s;
        if n2 < n {
            continue;
        }
        let s = s as usize;
        a[s] = 1;
        for i in 0..n {
            a[s] *= n2 - i;
            a[s] %= MOD;
            a[s] *= inv[(i + 1) as usize];
            a[s] %= MOD;
        }
    }
    let mut res = 0;
    for i in 0..n {
        let i = i as usize;
        for j in 0..n {
            let j = j as usize;
            if i == j {
                continue;
            }
            let s = rs[i] + rs[j];
            res += a[s as usize];
            if res >= MOD {
                res -= MOD;
            }
        }
    }
    for i in 0..(n - 2) {
        res = res * (i + 1) % MOD;
    }
    res
}

extern crate rand;
#[test]
fn it_works() {
    use rand::distributions::{IndependentSample, Range};

    let mut rng = rand::thread_rng();

    let t = 100;
    for _ in 0..t {
        let n = Range::new(1, 6).ind_sample(&mut rng);
        let m = Range::new(1, 15).ind_sample(&mut rng);
        let mut rs = vec![0; n as usize];
        for i in 0..n {
            rs[i as usize] = Range::new(1, 4).ind_sample(&mut rng);
        }
        let mut exp = 0;
        let mut is: Vec<i32> = (0..n).collect();
        loop {
            for i in 0..(1 << m) {
                let i = i as usize;
                if i.count_ones() != n as u32 {
                    continue;
                }
                let mut k = 0;
                let mut pos = vec![0; n as usize];
                for j in 0..m {
                    if ((i >> j) & 1) == 0 {
                        continue;
                    }
                    pos[k as usize] = j;
                    k += 1;
                }
                let mut ok = true;
                for j in 1..n {
                    let j = j as usize;
                    if pos[j] - pos[j - 1] < rs[is[j - 1] as usize] + rs[is[j] as usize] {
                        ok = false;
                    }
                }
                if ok {
                    exp += 1;
                }
            }
            if !contest::vec::next_permutation(&mut is) {
                break;
            }
        }

        println!("n: {}, m: {}, rs: {:?}, exp: {}", n, m, rs, exp);
        let res = solve(n as i64, m, rs);

        assert_eq!(exp, res);
    }
}
