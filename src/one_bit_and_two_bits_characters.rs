fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut i = 0;
    let n = bits.len();
    while i < n {
        if i == n-1 {
            return true;
        }
        if bits[i] == 1 {
            i += 2;
        } else {
            i += 1;
        }
    }

    false
}

pub fn main() {
    let bits = [0,1,0].to_vec();
    println!("{}", is_one_bit_character(bits));
}
