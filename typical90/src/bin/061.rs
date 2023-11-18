use bitvec::macros::internal::funty::Fundamental;
use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize
    }
    let mut queue: VecDeque<i64> = VecDeque::from([]);
    for _ in 1..=q {
        input! { (t, x): (i32, i64) }
        if t == 1 {
            queue.push_front(x);
        } else if t == 2 {
            queue.push_back(x);
        } else {
            println!("{}", queue.get((x - 1).as_usize()).unwrap());
        }
    }
}
