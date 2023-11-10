use proconio::{fastout, input};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        target: String
    }

    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back("".to_string());
    let words = vec![
        "dream".to_string(),
        "dreamer".to_string(),
        "erase".to_string(),
        "eraser".to_string(),
    ];

    while !queue.is_empty() {
        let str = queue.pop_front().unwrap();
        for (_, word) in words.iter().enumerate() {
            let tmp_str = format!("{}{}", str, word);
            if tmp_str == target {
                println!("YES");
                return;
            } else if tmp_str.len() < target.len() && tmp_str == target[0..tmp_str.len()] {
                queue.push_back(tmp_str);
            }
        }
    }
    println!("NO");
}
