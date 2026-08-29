fn reverse_str(s: String, k: i32) -> String {
    let mut res = s.chars().collect::<Vec<_>>();
    let k = std::cmp::min(s.len() as i32, k);
    let mut l = 0;
    let mut end = k;
    for j in 0..s.len() {
        if (j + 1) as i32 == end {
            let mut r = j;
            while l < r {
                (res[l], res[r]) = (res[r], res[l]);
                l += 1;
                r -= 1;
            }

            end += 2 * k;
            l = (end - k) as usize;
            end = std::cmp::min(s.len() as i32, end);
        }
    }

    res.iter().collect::<String>()
}

pub fn main() {
    let s = "hyzqyljrnigxvdtneasepfahmtyhlohwxmkqcdfehybknvdmfrfvtbsovjbdhevlfxpdaovjgunjqlimjkfnqcqnajmebeddqsgl".to_string();
    let k = 39;
    println!("{}", reverse_str(s, k));
}
