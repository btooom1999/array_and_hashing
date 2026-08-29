fn largest_combination(candidates: Vec<i32>) -> i32 {
    let mut bits = [0; 32];
    let mut res = 0;
    for num in candidates {
        let mut num = num;
        let mut i = 0;
        while num > 0 {
            if 1 & num == 1 {
                bits[i] += 1;
                res = res.max(bits[i]);
            }
            i += 1;
            num >>= 1;
        }
    }

    res
}

pub fn main() {
    let candidates = [16,17,71,62,12,24,14].to_vec();
    println!("{}", largest_combination(candidates));
}
