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

    let a: Vec<i32> = (0..n).map(|_| sc.next()).collect();

    let mut pref1: Vec<i32> = vec![0; n + 1];
    let mut pref2: Vec<i32> = vec![0; n + 1];

    for i in 0..n {
        pref1[i + 1] = pref1[i] + if a[i] == 1 { 1 } else { -1 };
        pref2[i + 1] = pref2[i] + if a[i] == 3 { -1 } else { 1 };
    }

    let mut mn = 1_000_000_007;
    for i in 1..n {
        if (pref2[i] - mn >= 0) {
            println!("YES");
            return;
        }
        if (pref1[i] >= 0) {
            mn = mn.min(pref2[i]);
        }
    }

    println!("NO");
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
