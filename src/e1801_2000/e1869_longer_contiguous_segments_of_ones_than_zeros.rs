fn check_zero_ones(s: String) -> bool {
    let (mut count_zeroes, mut max_zeroes) = (0, 0);
    let (mut count_ones, mut max_ones) = (0, 0);

    for &byte in s.as_bytes() {
        // count zeroes
        if byte == b'1' {
            count_zeroes = 0;
            count_ones += 1;
        } else {
            count_ones = 0;
            count_zeroes += 1;
        }

        max_zeroes = max_zeroes.max(count_zeroes);
        max_ones = max_ones.max(count_ones);
    }

    max_ones > max_zeroes
}

pub fn main() {
    let s = "01111110".to_string();
    println!("{}", check_zero_ones(s));
}
