fn roman_to_int(s: String) -> i32 {
    let mut romans = vec![0; 256];
    romans[b'I' as usize] = 1;
    romans[b'V' as usize] = 5;
    romans[b'X' as usize] = 10;
    romans[b'L' as usize] = 50;
    romans[b'C' as usize] = 100;
    romans[b'D' as usize] = 500;
    romans[b'M' as usize] = 1000;

    let s = s.as_bytes();
    let n = s.len();
    let mut i = n-1;
    let mut res = 0;
    while i < s.len() {
        if let Some(prev) = i.checked_sub(1) && romans[s[prev] as usize] < romans[s[i] as usize] {
            res += romans[s[i] as usize] - romans[s[prev] as usize];
            i -= 1;
        } else {
            res += romans[s[i] as usize];
        }
        i = i.wrapping_sub(1);
    }

    res
}

pub fn main() {
    let s = "MCMXCIV".to_string();
    println!("{}", roman_to_int(s));
}
