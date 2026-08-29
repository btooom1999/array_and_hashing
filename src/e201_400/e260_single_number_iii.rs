fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let mut xor = 0;
    for &num in &nums {
        xor ^= num;
    }

    let mut diff = 1;
    while diff & xor == 0 {
        diff <<= 1;
    }

    let mut a = 0;
    let mut b = 0;
    for num in nums {
        if num & diff != 0 {
            a ^= num;
        } else {
            b ^= num;
        }
    }

    vec![a, b]
}

pub fn main() {
    let nums = [1,2,1,3,2,5].to_vec();
    println!("{:?}", single_number(nums));
}
