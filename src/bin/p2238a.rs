use std::io::{self, Read};

struct Scanner<'a> {
    input: std::str::SplitWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            input: s.split_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.input.next().unwrap().parse().ok().unwrap()
    }
}

fn solve(sc: &mut Scanner) {
    let n: usize = sc.next();

    let cost: usize = sc.next();

    let mut a: Vec<i32> = (0..n).map(|_| sc.next()).collect();
    let mut b: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let mut steps = 0;

    let mut need_reorder: bool = false;

    for i in 0..a.len() {
        if a[i] < b[i] {
            need_reorder = true;
        }
        steps += a[i] - b[i];
    }

    if need_reorder {
        need_reorder = false;
        a.sort_unstable();
        b.sort_unstable();

        steps += cost as i32;

        for i in 0..a.len() {
            if a[i] < b[i] {
                need_reorder = true;
            }
        }
    }

    if !need_reorder {
        println!("{steps}");
    } else {
        println!("-1");
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut sc = Scanner::new(&input);

    let t: usize = sc.next();

    for _ in 0..t {
        solve(&mut sc);
    }
}
