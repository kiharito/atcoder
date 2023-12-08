use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (String, i32)
    }
    let mut num = i64::from_str_radix(&n, 8).unwrap();
    for _ in 0..k {
        let mut nine = shinsu(num, 9);
        nine = nine.replace('8', "5");
        num = i64::from_str_radix(&nine, 8).unwrap();
    }
    println!("{:o}", num);
}

// x(10進数)をn進数表記の文字列に変換する
fn shinsu(x: i64, n: i64) -> String {
    if x == 0 {
        return "0".to_string();
    }

    let mut remainders: Vec<String> = Vec::new();
    let mut num = x;
    while num > 0 {
        remainders.push((num % n).to_string());
        num /= n;
    }

    remainders.reverse();
    remainders.join("")
}
