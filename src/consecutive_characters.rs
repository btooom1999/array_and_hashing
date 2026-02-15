fn max_power(s: String) -> i32 {
    let mut count = 0;
    let mut res = 0;
    let mut cur_byte = 0;
    for &byte in s.as_bytes() {
        if cur_byte != byte {
            count = 1;
            cur_byte = byte;
        } else {
            count += 1;
        }

        res = res.max(count);
    }

    res
}

pub fn main() {
    let s = "abbcccddddeeeeedcba".to_string();
    println!("{}", max_power(s));
}
