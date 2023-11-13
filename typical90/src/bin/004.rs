use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        matrix: [[i32; w]; h]
    }

    let mut row_sums = vec![0; h];
    let mut col_sums = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row_sums[i] += matrix[i][j];
            col_sums[j] += matrix[i][j];
        }
    }
    for i in 0..h {
        let mut row = vec![0; w];
        for j in 0..w {
            row[j] = row_sums[i] + col_sums[j] - matrix[i][j];
        }
        println!("{}", row.iter().join(" "));
    }
}
