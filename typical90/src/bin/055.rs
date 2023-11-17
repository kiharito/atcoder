use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, p, q): (usize, i64, i64),
        a: [i64; n]
    }

    let mut ans = 0;
    for nums in a.iter().combinations(5) {
        let mut prod = 1;
        for num in &nums {
            prod *= *num % p;
            prod %= p;
        }
        if prod % p == q {
            ans += 1;
        }
    }
    println!("{}", ans);
}
