use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize)
    }

    if h == 1 || w == 1 {
        println!("{}", h * w);
        return;
    }
    println!("{}", (((h + 1) / 2) * ((w + 1) / 2)));
}
