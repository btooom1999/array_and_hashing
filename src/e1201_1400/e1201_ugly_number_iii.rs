fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a%b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a*b/gcd(a, b)
}

fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
    let (n, a, b, c) = (n as i64, a as i64, b as i64, c as i64);
    let mut l = 1;
    let mut r = 2_000_000_000;
    let lcm_ab = lcm(a, b);
    let lcm_ac = lcm(a, c);
    let lcm_bc = lcm(b, c);
    let lcm_abc = lcm(lcm_ab, c);

    while l < r {
        let m = (l+r)/2;
        let total = m/a + m/b + m/c - m/lcm_ab - m/lcm_bc - m/lcm_ac + m/lcm_abc;
        if total >= n {
            r = m;
        } else {
            l = m+1;
        }
    }

    l as i32
}

pub fn main() {
    let n = 3;
    let a = 2;
    let b = 3;
    let c = 5;
    println!("{}", nth_ugly_number(n, a, b, c));
}
