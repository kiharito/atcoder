use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, i32),
        a: [i32; n],
        b: [i32; n]
    }
    let mut diff_total = 0;
    for i in 0..n {
        diff_total += (a[i] - b[i]).abs();
    }
    if k >= diff_total && (k - diff_total) % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
