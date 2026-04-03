fn count_primes(n: i32) -> i32 {
    if n <= 2 {
        return 0;
    }

    let mut remaining = n-2;
    let mut dp = vec![true; (n-2) as usize];
    for i in 2..(n as f32).sqrt().ceil() as usize {
        if dp[i-2] {
            let mut num = i*i;
            while num < n as usize {
                if dp[num-2] { remaining -= 1; }
                dp[num-2] = false;
                num += i;
            }
        }
    }


    remaining
}

pub fn main() {
    let n = 10;
    println!("{}", count_primes(n));
}
