use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize)
    }

    let mut ans = 0;
    for i in 1..=n {
        let mut sum = 0;
        let mut tmp_i = i;
        loop {
            sum += tmp_i % 10;
            tmp_i /= 10;
            if tmp_i == 0 {
                break;
            }
        }
        if a <= sum && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
