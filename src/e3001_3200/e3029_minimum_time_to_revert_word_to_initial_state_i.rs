fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
    let n = word.len();
    let word = word.as_bytes();
    let mut z = vec![0; n];
    let mut left = 0;
    let mut right = 0;
    for k in 1..n {
        let k1 = k-left;
        if k > right || z[k1] > right-k {
            left = k;
            right = right.max(k);
            while right < n && word[right] == word[right-left] {
                right += 1;
            }

            z[k] = right-left;
            right -= 1;
        } else {
            z[k] = z[k1];
        }
    }

    let mut time = 1;
    let k = k as usize;
    for i in (k..n).step_by(k) {
        if z[i] == n-i {
            return time;
        } else {
            time += 1;
        }
    }

    time
}

pub fn main() {
    let word = "abacaba".to_string();
    let k = 3;
    println!("{}", minimum_time_to_initial_state(word, k));
}
