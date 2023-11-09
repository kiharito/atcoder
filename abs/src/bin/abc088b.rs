use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }

    a.sort();
    a.reverse();

    let mut alice_point = 0;
    let mut bob_point = 0;
    for (i, score) in a.iter().enumerate() {
        if i % 2 == 0 {
            alice_point += score;
        } else {
            bob_point += score;
        }
    }

    println!("{}", alice_point - bob_point);
}
