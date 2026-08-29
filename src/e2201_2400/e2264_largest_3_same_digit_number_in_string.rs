fn largest_good_integer(num: String) -> String {
    let mut candidate = -1;
    let mut count = 0;
    let mut max = -1;
    for n in num.chars() {
        let n = n.to_digit(10).unwrap() as i32;
        if n == candidate {
            count += 1;

            if count >= 3 {
                max = std::cmp::max(max, candidate);
            }
        } else {
            count = 1;
            candidate = n;
        }
    }

    if max == -1 {
        String::new()
    } else {
        vec![max.to_string(); 3].join("")
    }
}

pub fn main() {
    let num = "6777133339".to_string();
    println!("{}", largest_good_integer(num));
}
