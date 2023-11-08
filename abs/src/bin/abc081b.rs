use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    let mut cnt = 0;
    loop {
        if a.iter().any(|&i| i % 2 == 1) {
            break;
        } else {
            a = a.iter().map(|i| i / 2).collect();
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
