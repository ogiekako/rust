use std::f32;
use std::io;
use std::io::{BufRead};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use std::time::Duration;

const L: i32 = 1000; // border
const N: usize = 1000;
const M: usize = 100000;

// params
const RAD: f32 = 0.0; // r
const RAD2: f32 = 0.0; // r * r
const NEXT_FRIEND: f32 = 0.1; // EDGE * NEXT_FRIEND
const EDGE: f32 = 1.0;
const EDGE_R: f32 = 0.0;
const POINT: f32 = 1.0;

const ALLOWED_DELAY: usize = 5; // if exceeds, force to put.
const DELAY_COUNT: usize = 10;

const MAX_WALK: i32 = 2;

const MAX_R: i32 = 200;

const RAND_ITER: usize = 1000;

struct Solver {
    vs: Vec<V>,
    used: Vec<usize>, // index of vs
    min: P,           // min(x-r)
    max: P,           // max(x+r)

    timer: std::time::SystemTime,
    tle: Duration,

    rand: Box<FnMut()->u32>,
}

impl Solver {
    fn priority(&mut self, i: usize) -> f32 {
        let v = &self.vs[i];
        // -1 if already used.

        // if v.delay > 0 {
        //     let mut v = &mut v;
        //     v.delay -= 1;
        //     if v.delay == 0 {
        //         v.num_delayed += 1;
        //     }
        //     return -1.0;
        // }
        if v.impossible {
            return -1.0;
        }
        for u in &self.used {
            if v.id == *u {
                return -1.0;
            }
        }
        let mut pri = 0.0;

        let vr = v.r as f32;
        let vs = v.s as f32;

        pri += vr * RAD;
        pri += vr * vr * RAD2;

        for e in &v.es {
            let u = &self.vs[e.to];
            let ur = u.r as f32;
            let es = e.s as f32;
            // Edge
            if u.is_used() {
                pri += es / ur / vr * (EDGE + EDGE_R * vr);
            } else {
                pri += es / ur / vr * (NEXT_FRIEND + EDGE_R * vr);
            }

            // TODO: consider friends' friends.
        }

        pri += vs / vr / vr / vr * POINT;

        pri
    }

    fn is_inside(&self, p: P, r: i32) -> bool {
        let max2 = self.max.max(p + P(r, r, r));
        let min2 = self.min.min(p - P(r, r, r));
        let ll = max2 - min2;
        ll.0 <= L && ll.1 <= L && ll.2 <= L
    }

    fn score_on(&self, p: P, i: usize) -> i64 {
        // -1 on invalid pos. O(|used| + |v.es|)
        let v = &self.vs[i];
        if !self.is_inside(p, v.r) {
            return -1;
        }
        for j in &self.used {
            let u = &self.vs[*j];
            let rr = u.r + v.r;
            if u.p.dist2(p) < rr * rr {
                return -1;
            }
        }
        let mut res = v.s;
        for e in &v.es {
            let u = &self.vs[e.to];
            if u.is_used() {
                if u.p.dist2(p) <= e.c * e.c {
                    res += e.s;
                }
            }
        }
        return res;
    }

    fn calc_pos(&mut self, i: usize) -> Option<P> {
        let v = &self.vs[i];

        if self.used.len() == 0 {
            return Some(P(0, 0, 0));
        }

        let mut best_score = 0;
        let mut best_score_cands = vec![]; // points

        let mut fav = vec![];
        let e_len = v.es.len();
        for j in 0..e_len {
            let e = &v.es[j];
            let mut u = &self.vs[e.to];
            if u.is_used() {
                fav.push(j);
            }
        }
        // if fav.len() > 1 {
        //     let n = fav.len();
        //     // eprintln!("fav: {}", n);
        //     for jj in 0..n {
        //         for kk in 0..jj {
        //             let ej = &v.es[jj];
        //             let ek = &v.es[kk];

        //             let pj = self.vs[ej.to].p;
        //             let pk = self.vs[ek.to].p;
        //             
        //             let mut rj = self.vs[ej.to].r;
        //             let mut rk = self.vs[ek.to].r;

        //             let cj = ej.c;
        //             let ck = ek.c;

        //             let r = v.r;

        //             if pj.dist2(pk) > (cj + ck) * (cj + ck) {
        //                 continue;
        //             }
        //             let dir = pk - pj;
        //             if pj.dist2(pk) <= (r + r + rj + rk) * (r + r + rj + rk) {
        //                 // do nothing.
        //             } else {
        //                 rj = ej.c - r;
        //             }
        //         }
        //     }
        // }

        // one
        for e in &v.es {
            let mut u = &self.vs[e.to];
            if !u.is_used() {
                continue;
            }
            // For each friend
            let rr = u.r + v.r;
            for x in -1..2 {
                // 27 iter
                for y in -1..2 {
                    for z in -1..2 {
                        if x == 0 && y == 0 && z == 0 {
                            continue;
                        }

                        let mut sq = 0.0f32;
                        if x != 0 {
                            sq += 1.0;
                        }
                        if y != 0 {
                            sq += 1.0;
                        }
                        if z != 0 {
                            sq += 1.0;
                        }
                        let m = (((rr as f32) / sq.sqrt()).ceil()) as i32;

                        let d = P(x, y, z);
                        for k in 0..MAX_WALK {
                            let pp = u.p + d * (m + k);
                            if !self.is_inside(pp, v.r) {
                                break;
                            }
                            let s = self.score_on(pp, i);
                            if s > best_score {
                                best_score = s;
                                best_score_cands.clear();
                                best_score_cands.push(pp);
                            } else if s == best_score {
                                best_score_cands.push(pp);
                            }
                        }
                    }
                }
            }
        }

        // TODO: two, three.
        // TODO: cosider delaying.

        if best_score_cands.len() > 0 {
            // eprintln!("best_score: {}", best_score);
            return Some(best_score_cands[0]);
        }
        
        None
    }
    fn put_random(&mut self, i: usize) -> Option<P>{
        let mut best_score = 0;
        let mut best_score_cands = vec![];
        // random
        for _ in 0..RAND_ITER {
            let pp = self.max - self.min;
            let x = pp.0 / 2 - L / 2 + ((self.rand)() as i32) % L;
            let y = pp.1 / 2 - L / 2 + ((self.rand)() as i32) % L;
            let z = pp.2 / 2 - L / 2 + ((self.rand)() as i32) % L;
            let pp = P(x, y, z);
            let s = self.score_on(pp, i);
            if s > best_score {
                best_score = s;
                best_score_cands.clear();
                best_score_cands.push(pp);
            } else if s == best_score {
                best_score_cands.push(pp);
            }
        }

        if best_score_cands.len() > 0 {
            // eprintln!("rand:  best_score: {}", best_score);
            return Some(best_score_cands[0]);
        }
        None
    }

    fn solve(&mut self) {
        for i in 0..N {
            if self.vs[i].r > MAX_R {
                self.vs[i].impossible = true;
            }
        }

        let mut ii = 0;

        let mut calc_pos_time = Duration::from_secs(0);
        let mut pri_time = Duration::from_secs(0);

        let mut num_imp = 0;

        loop {
            ii += 1;
            // eprintln!("solve: {} {}", ii, num_imp);

            if self.timer.elapsed().unwrap() > self.tle {
                break;
            }

            let mut pi = vec![];

            let current = self.timer.elapsed().unwrap();
            // O(M)
            for i in 0..N {
                let p = self.priority(i);
                pi.push((p, i));
            }
            pri_time += self.timer.elapsed().unwrap() - current;

            // reverse order
            pi.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

            let mut ok = false;

            let current = self.timer.elapsed().unwrap();
            for (pri, i) in pi {
                if pri == -1.0 {
                    break;
                }
                if let Some(p) = self.calc_pos(i) {
                    // // eprintln!("put {:?} to {}", p, i);
                    self.vs[i].p = p;
                    self.used.push(i);
                    let r = self.vs[i].r;
                    self.max = self.max.max(p + P(r, r, r));
                    self.min = self.min.min(p - P(r, r, r));

                    ok = true;
                    break;
                } else if self.vs[i].delay > 0 {
                    // consider later
                    // Do nothing.
                    if self.vs[i].num_delayed >= ALLOWED_DELAY {
                        panic!("Should not reach");
                    }
                    panic!("Not implemented");
                } else {
                    self.vs[i].impossible = true;
                    num_imp += 1;
                }
            }
            calc_pos_time += self.timer.elapsed().unwrap() - current;
            if !ok {
                break;
            }
        }
        for i in 0..N {
            if self.vs[i].is_used() {
                continue;
            }
            if let Some(p) = self.put_random(i) {
                    self.vs[i].p = p;
                    self.used.push(i);
                    let r = self.vs[i].r;
                    self.max = self.max.max(p + P(r, r, r));
                    self.min = self.min.min(p - P(r, r, r));

            }
        }
        self.adjust();

        for i in 0..N {
            println!("{} {} {}", self.vs[i].p.0, self.vs[i].p.1, self.vs[i].p.2);
        }


        // eprintln!("pri: {:?}   put: {:?}", pri_time, calc_pos_time);

        let mut used_backet = [0,0,0,0]; // 1-50, ...
        let mut unused_backet = [0,0,0,0];
        for i in 0..N {
            let v = &self.vs[i];
            if v.is_used() {
                used_backet[((v.r-1) / 50) as usize] += 1;
            } else {
                unused_backet[((v.r -1) / 50) as usize] += 1;
            }
        }
        // eprintln!("used: {:?},  unused: {:?}", used_backet, unused_backet);
    }
    fn adjust(&mut self) {
        for i in 0..N {
            if self.vs[i].p.is_none() {
                continue;
            }
            self.vs[i].p -= self.min;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct P(i32, i32, i32);

impl Mul<i32> for P {
    type Output = P;

    fn mul(self, rhs: i32) -> P {
        P(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Add for P {
    type Output = P;

    fn add(self, other: P) -> P {
        P(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for P {
    fn add_assign(&mut self, other: P) {
        *self = P(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for P {
    type Output = P;

    fn sub(self, other: P) -> P {
        P(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for P {
    fn sub_assign(&mut self, other: P) {
        *self = P(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl P {
    fn is_none(self) -> bool {
        self == P(-1, -1, -1)
    }
    fn dist2(self, o: P) -> i32 {
        (self - o).norm2()
    }
    fn norm2(self) -> i32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    fn max(self, o: P) -> P {
        P(
            std::cmp::max(self.0, o.0),
            std::cmp::max(self.1, o.1),
            std::cmp::max(self.2, o.2),
        )
    }
    fn min(self, o: P) -> P {
        P(
            std::cmp::min(self.0, o.0),
            std::cmp::min(self.1, o.1),
            std::cmp::min(self.2, o.2),
        )
    }
}

struct V {
    id: usize,
    r: i32,
    s: i64, // score
    es: Vec<E>,

    delay: usize, // consider later.
    num_delayed: usize,
    p: P,
    impossible: bool,
}

impl V {
    fn is_used(&self) -> bool {
        !self.p.is_none()
    }
    fn new(i: usize, r: i32, s: i64) -> V {
        V {
            id: i,
            r: r,
            s: s,
            es: vec![],

            delay: 0,
            num_delayed: 0,
            p: P(-1, -1, -1),
            impossible: false,
        }
    }
}

struct E {
    from: usize,
    to: usize,
    c: i32,
    s: i64, // score
}

fn score(ps: &Vec<V>) -> i64 {
    let mut res = 0;
    let mut bonus = 0;
    for i in 0..N {
        if ps[i].p.is_none() {
            continue;
        }
        let p = &ps[i].p;
        let r = ps[i].r;
        if p.0 < r || p.1 < r || p.2 < r {
            panic!("point {} is too small", i);
        }
        if p.0 + r > L || p.1 + r > L || p.2 + r > L {
            panic!("point {} is too big", i);
        }
        res += ps[i].s;
        for j in 0..i {
            if ps[j].p.is_none() {
                continue;
            }
            let d2 = ps[i].p.dist2(ps[j].p);
            let r2 = (ps[i].r + ps[j].r) * (ps[i].r + ps[j].r);
            if d2 < r2 {
                panic!("{} and {} too close.", i, j)
            }
        }
        for e in &ps[i].es {
            if !ps[e.to].p.is_none() {
                if ps[i].p.dist2(ps[e.to].p) <= e.c * e.c {
                    bonus += e.s;
                }
            }
        }
    }
    res + bonus / 2
}

fn main() {
    let mut input = Scanner::new(std::io::stdin());
    let _l: usize = input.next().unwrap();
    let n: usize = input.next().unwrap();
    let m: usize = input.next().unwrap();

    let mut vs: Vec<V> = vec![];
    for i in 0..n {
        vs.push(V::new(i, input.next().unwrap(), input.next().unwrap()));
    }
    for _ in 0..m {
        let mut e = E {
            from: input.next().unwrap(),
            to: input.next().unwrap(),
            c: input.next().unwrap(),
            s: input.next().unwrap(),
        };
        e.from -= 1;
        e.to -= 1;
        let r = E {
            from: e.to,
            to: e.from,
            c: e.c,
            s: e.s,
        };
        vs[e.from].es.push(e);
        vs[r.from].es.push(r);
    }

    let mut tle = 2500;
    if let Some(outfile) = std::env::args().nth(1) {
        if outfile == "local" {
            tle = 5000;
            // local config
        } else {
            // judge
            let mut out = Scanner::new(std::io::BufReader::new(
                std::fs::File::open(&outfile).unwrap(),
            ));

            for i in 0..n {
                vs[i].p = P(
                    out.next().unwrap(),
                    out.next().unwrap(),
                    out.next().unwrap(),
                );
            }
            let res = score(&vs);
            println!("score: {}", res);
            return;
        }
    }

    // Solver
    let mut solver = Solver {
        vs: vs,
        used: vec![],
        min: P(0, 0, 0),
        max: P(0, 0, 0),

        timer: std::time::SystemTime::now(),
        tle: Duration::from_millis(tle),
        rand: xorshift32(),
    };


    solver.solve();
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
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.next_string()
            .map(|s| s.parse::<T>().expect("Parse failed: "))
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
fn xorshift32() -> Box<FnMut() -> u32> {
        let mut y = 2463534242 ;
            Box::new(move || {
                        y = y ^ (y << 13);
                                y = y ^ (y >> 17);
                                        y = y ^ (y << 5);
                                                y
                                                        })
}
