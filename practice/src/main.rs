use proconio::input;

fn main() {
    input! {
        a: usize,
        (b, c): (usize, usize),
        s: String
    }
    println!("{} {}", a + b + c, s);
}
