fn primes() -> Vec<bool> {
    let n = 100_000;
    let mut prime= vec![true; n+1];
    prime[0] = false;
    prime[1] = false;

    for i in 2..(n as f32).sqrt().ceil() as usize {
        if prime[i] {
            let mut num = i*i;

            while num < n {
                prime[num] = false;
                num += i;
            }
        }
    }

    prime
}

fn non_special_count(l: i32, r: i32) -> i32 {
    let primes = primes();

    let mut count = r-l+1;
    let mut num = 2;
    while num * num <= r {
        if primes[num as usize] && num * num >= l {
            count -= 1;
        }
        num += 1;
    }

    count
}

pub fn main() {
    let l = 1;
    let r = 1_000_000_000;
    println!("{}", non_special_count(l, r));
}
