use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        plans: [(i32, i32, i32); n]
    }

    let mut start_position = (0, 0);
    let mut last_t = 0;
    for (_, (t, x, y)) in plans.iter().enumerate() {
        let diff_xy = (x - start_position.0).abs() + (y - start_position.1).abs();
        let diff_t = t - last_t;
        if diff_xy > diff_t || (diff_t - diff_xy) % 2 == 1 {
            println!("No");
            return;
        }
        start_position = (*x, *y);
        last_t = *t;
    }
    println!("Yes");
}
