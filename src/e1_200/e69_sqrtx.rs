fn my_sqrt(num: i32) -> i32 {
    let num = num as i64;
    let mut l = 0;
    let mut r = num;

    while l < r {
        let m = (l + r) / 2;
        let val = m * m;
        if val == num {
            return m as i32;
        } else if val > num {
            r = m - 1;
        } else {
            l = m + 1;
        }
    }

    if (l * l) > num { l as i32 - 1 } else { l as i32 }
}

pub fn main() {
    let x = 1;
    println!("{}", my_sqrt(x));
}
