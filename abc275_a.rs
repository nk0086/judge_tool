#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars},
};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let h_max = h.iter().max().unwrap();
    for i in 0..n {
        if h[i] == *h_max {
            println!("{}", i + 1);
            break;
        }
    }
}

