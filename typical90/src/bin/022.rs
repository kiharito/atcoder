use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize)
    }
    let gcd = num::integer::gcd(a, num::integer::gcd(b, c));
    println!("{}", a / gcd + b / gcd + c / gcd - 3);
}
