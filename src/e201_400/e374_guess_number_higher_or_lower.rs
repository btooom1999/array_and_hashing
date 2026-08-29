use std::cmp::Ordering;

const N: i32 = 2126753390;
const PICK: i32 = 1702766719;

fn guess(num: i32) -> i32 {
    match PICK.cmp(&num) {
        Ordering::Less => -1,
        Ordering::Greater => 1,
        Ordering::Equal => 0,
    }

}

fn guess_number(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;

    while l < r {
        let m = ((l as i64 + r as i64) / 2) as i32;
        let val = guess(m);
        if val == 0 {
            return m;
        } else if val == 1 {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }

    l
}

pub fn main() {
    println!("{}", guess_number(N));
}
