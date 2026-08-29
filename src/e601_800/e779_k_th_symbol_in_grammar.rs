fn kth_grammar(n: i32, k: i32) -> i32 {
    let mut left = 1;
    let mut right = 1 << (n-1);

    let mut cur = 0;
    for _ in 0..n-1 {
        let mid = (left+right)/2;
        if k <= mid {
            right = mid;
        } else {
            left = mid+1;
            cur = if cur == 0 { 1 } else { 0 };
        }
    }

    cur
}

pub fn main() {
    let n = 3;
    let k = 4;
    println!("{}", kth_grammar(n, k));
}
