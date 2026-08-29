fn arrange_coins(n: i32) -> i32 {
    let n = n as i64;
    let mut l = 1;
    let mut r = n;

    while l < r {
        let m = (l + r) / 2;
        let val = m * (m + 1) / 2;
        if val == n {
            return m as i32;
        } else if val > n {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    if l * (l + 1) / 2 <= n { l as i32 } else { l as i32 - 1 }
}

pub fn main() {
    let n = 2;
    println!("{}", arrange_coins(n));
}
