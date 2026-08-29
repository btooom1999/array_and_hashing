fn minimum_sum(num: i32) -> i32 {
    let mut num = num.to_string().into_bytes();
    num.sort();

    let (mut even, mut odd) = (0, 0);
    for (i, c) in num.into_iter().enumerate() {
        let c = (c - b'0') as i32;
        if i % 2 == 0 {
            even = even * 10 + c;
        } else {
            odd = odd * 10 + c;
        }
    }

    even + odd
}

pub fn main() {
    let num = 1002;
    println!("{}", minimum_sum(num));
}
