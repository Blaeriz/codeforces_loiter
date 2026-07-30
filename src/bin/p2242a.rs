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

    let mut count: i32 = 0;

    if n == 1 {
        if a[0] == 1 || a[0] == 2 {
            println!("NO");
            return;
        } else {
            println!("YES");
            return;
        }
    } else {
        for i in 0..n {
            if a[i] > 1 {
                if a[i] >= 3 {
                    println!("YES");
                    return;
                }
                count += 1;
            }
            if count >= 2 {
                println!("YES");
                return;
            }
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
