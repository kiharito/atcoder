use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }

    let mut cnt = 0;
    for i in 0..=a {
        for j in 0..=b {
            if (x >= 500 * i + 100 * j) && ((x - 500 * i - 100 * j) / 50 <= c) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
