fn largest_time_from_digits(arr: Vec<i32>) -> String {
    let mut counts = [0; 10];
    for num in arr {
        counts[num as usize] += 1;
    }

    for x in [(2,0,3), (1,0,9), (0,0,9)] {
        if counts[x.0] ==  0 {
            continue;
        }

        counts[x.0] -= 1;
        for f in (x.1..x.2+1).rev() {
            if counts[f] == 0 {
                continue;
            }

            counts[f] -= 1;
            for s in (0..6).rev() {
                if counts[s] == 0 {
                    continue;
                }

                counts[s] -= 1;
                for t in (0..10).rev() {
                    if counts[t] == 0 {
                        continue;
                    }

                    return format!("{}{}:{}{}", x.0, f, s, t);
                }
                counts[s] += 1;
            }
            counts[f] += 1;
        }
        counts[x.0] += 1;
    }

    String::new()
}

pub fn main() {
    let arr = [1,9,2,6].to_vec();
    println!("{}", largest_time_from_digits(arr))
}
