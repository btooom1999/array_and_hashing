fn helper(m: i32, n: i32, target: i32) -> i32 {
    let mut count = 0;
    let mut i = 1;
    let mut j = n;
    while i <= m && j > 0 {
        if i*j <= target {
            count += j;
            i += 1;
        } else {
            j -= 1;
        }
    }

    count
}

fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
    let mut l = 0;
    let mut r = m*n+1;
    let mut res = 0;
    while l < r {
        let target = (l+r)/2;
        if helper(m, n, target) < k {
            l = target + 1;
        } else {
            res = target;
            r = target;
        }
    }

    res
}

pub fn main() {
    let m = 3;
    let n = 3;
    let k = 5;
    println!("{}", find_kth_number(m, n, k));
}
