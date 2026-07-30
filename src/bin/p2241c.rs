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

    let s: String = sc.next();

    let a: Vec<i32> = s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mut count: i32 = 0;

    for i in 1..n {
        if a[i - 1] != a[i] {
            count += 1;
        }
    }

    if count >= 2 {
        println!("1");
        return;
    }

    if count == 1 {
        println!("2");
        return;
    }

    if count == 0 {
        println!("1");
        return;
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
