fn count_prime_set_bits(left: i32, right: i32) -> i32 {
    let mut primes = [false; 20];
    primes[2] = true;
    primes[3] = true;
    primes[5] = true;
    primes[7] = true;
    primes[11] = true;
    primes[13] = true;
    primes[17] = true;
    primes[19] = true;

    let mut res = 0;
    for num in left..=right {
        if primes[num.count_ones() as usize] {
            res += 1;
        }
    }

    res
}

pub fn main() {
    let left = 6;
    let right = 10;
    println!("{}", count_prime_set_bits(left, right));
}
