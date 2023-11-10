use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, y): (i32, i32)
    }

    let mut ans = (-1, -1, -1);
    for i in 0..=n {
        for j in 0..=(n - i) {
            let k = n - i - j;
            let sum = 10000 * i + 5000 * j + 1000 * k;
            if sum == y {
                ans = (i, j, k);
                break;
            }
        }
    }
    println!("{} {} {}", ans.0, ans.1, ans.2);
}
