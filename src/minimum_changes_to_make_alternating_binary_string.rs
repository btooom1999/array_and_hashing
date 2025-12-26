fn min_operations(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();

    let mut diff0 = 0;
    let mut diff1 = 0;
    for (i, n) in s.into_iter().enumerate() {
        if n != if i % 2 == 0 { '0' } else { '1' } {
            diff0 += 1;
        }

        if n != if i % 2 == 0 { '1' } else { '0' } {
            diff1 += 1;
        }
    }

    std::cmp::min(diff1, diff0)
}

pub fn main() {
    let s = "10".to_string();
    println!("{}", min_operations(s));
}
