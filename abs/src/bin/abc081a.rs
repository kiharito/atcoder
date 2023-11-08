use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }

    let mut cnt = 0;
    for c in s.chars() {
        if c == '1' {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
