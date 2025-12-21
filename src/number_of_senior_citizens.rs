fn count_seniors(details: Vec<String>) -> i32 {
    let mut count = 0;

    for d in details {
        if let Ok(age) = d[11..13].parse::<i32>() && age > 60 {
            count += 1;
        }
    }

    count
}

pub fn main() {
    let details = ["7868190130M7522","5303914400F9211","9273338290F4010"].iter().map(|v| v.to_string()).collect::<Vec<_>>();

    println!("{}", count_seniors(details));
}
