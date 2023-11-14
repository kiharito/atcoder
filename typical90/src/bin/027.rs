use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n]
    }
    let mut cache: HashMap<String, i32> = HashMap::new();
    for (i, user) in s.iter().enumerate() {
        if cache.get(user).is_none() {
            println!("{}", i + 1);
            cache.insert(user.to_string(), 1);
        }
    }
}
