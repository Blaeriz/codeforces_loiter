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
    let n: i32 = sc.next(); // size of binary string

    let k: i32 = sc.next(); // pairs of equal adjacent characters

    // start with a string of all 0, and then check if 1 can be added in such a way that satisfies the condition
    // assuming 5 2, 0 0 0 0 0, now we need 2 pairs so 1 1 0 1 1, but difference between number of 0s and 1s should be 1 so 0 1 1 1 0 is the only valid possibility
    //
    // case 1 : n-k = 1 print "-1"
    //
    // case 2 : k = 0 = print alternating 1s and 0s
    //
    // note: 3 consecutive = 2 pairs, after that 4 consecutive -> 0 0 0 0 is still only 3 but 0 0 0 1 1 is also 3 pairs but differrent lengths
    //
    // this wont hold up the |0s-1s| = 1 condition tho
    //
    // build in pairs. 00 11 00 11 ..... number of blocks required would be n-k lets assume

    let r = n - k;

    if r == 1 {
        println!("-1");
        return;
    }

    // we want 0s and 1s to differ by 1 at most
    // therefore we target
    //
    // n = 5 -> 000 / 11
    // n = 6 -> 000 / 111

    let n0 = (n + 1) / 2;
    let n1 = n / 2;

    // we need r blocks but the blocks must alternate so as to not merge.

    let nr0 = (r + 1) / 2;
    let nr1 = r / 2;

    let extra0 = n0 - nr0;
    let extra1 = n1 - nr1;

    let mut ans = String::new();

    for i in 0..r {
        if i % 2 == 0 {
            // This is a 0-block.

            if i == 0 {
                // First 0-block gets all extra zeros.
                ans.push_str(&"0".repeat((1 + extra0) as usize));
            } else {
                // Other 0-blocks have length 1.
                ans.push('0');
            }
        } else {
            // This is a 1-block.

            if i == 1 {
                // First 1-block gets all extra ones.
                ans.push_str(&"1".repeat((1 + extra1) as usize));
            } else {
                // Other 1-blocks have length 1.
                ans.push('1');
            }
        }
    }

    println!("{}", ans);
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
