fn max_score(s: String) -> i32 {
    let mut s = s
        .chars()
        .map(|v| v.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();

    let mut res = 0;
    let mut sum: i32 = s.iter().sum();
    let mut zero_count = 0;
    for num in s.iter().take(s.len() - 1) {
        if *num == 0 {
            zero_count += 1;
        }
        if *num == 1 {
            sum -= 1;
        }
        res = std::cmp::max(res, sum + zero_count);
    }

    res
}

pub fn main() {
    let s = "011101".to_string();
    println!("{}", max_score(s));
}
