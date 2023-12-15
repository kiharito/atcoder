use im_rc::HashMap;
use proconio::{fastout, input};
use std::cmp;

#[fastout]
fn main() {
    input! {
        (n, m): (i32, i32)
    }
    // keyより小さい隣接頂点の個数をvalueとしてもつ
    let mut graph: HashMap<i32, i32> = HashMap::new();
    for i in 1..=n {
        graph.insert(i, 0);
    }

    for _ in 0..m {
        input! {(a, b): (i32, i32)}
        let max = cmp::max(a, b);
        let cnt = *graph.get(&max).unwrap();
        graph.insert(max, cnt + 1);
    }

    let mut ans = 0;
    for (_, v) in graph {
        if v == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
