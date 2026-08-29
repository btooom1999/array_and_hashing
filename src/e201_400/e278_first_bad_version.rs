fn is_bad_version(n: i32) -> bool {
    const BAD: i32 = 4;
    n >= BAD
}

fn first_bad_version(n: i32) -> i32 {
    let mut l = 1;
    let mut r = n;
    while l < r {
        let m = l + (r-l) / 2;
        if !is_bad_version(m) {
            l = m+1;
        } else {
            r = m;
        }
    }

    l
}

pub fn main() {
    let n = 5;
    println!("{}", first_bad_version(n));
}
